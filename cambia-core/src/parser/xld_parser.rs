mod sha256custom;

use std::{collections::{HashMap, HashSet}, iter::zip, str::FromStr};

use regex::{Regex, RegexBuilder};
use base64::{Engine as _, engine::GeneralPurpose, engine::general_purpose::PAD, alphabet::Alphabet};

use crate::{extract::{Extractor, Gap, MediaType, Quartet, ReadMode, ReleaseInfo, Ripper, TrackExtractor}, integrity::IntegrityChecker, toc::{Toc, TocEntry, TocRaw}, track::{AccurateRipConfidence, AccurateRipConfidenceTotal, AccurateRipOffset, AccurateRipUnit, TestAndCopy, TrackEntry, TrackError, TrackErrorData, TrackErrorRange}, translate::{Translator, TranslatorCombined}, util::Time};
use simple_text_decode::DecodedText;

use self::sha256custom::Sha256Custom;

use super::{Parser, ParsedLog, ParserCombined, ParsedLogCombined, ParserTrack};

lazy_static! {
    static ref RIPPER_VERSION: Regex = Regex::new(r"X Lossless Decoder version (.+)").unwrap();
    static ref RELEASE_INFO: Regex = Regex::new(r"XLD extraction logfile from .+[\r\n]+(?P<relinfo>.+)").unwrap();
    static ref USED_DRIVE: Regex = Regex::new(r"Used drive( *): (.+)").unwrap();
    static ref MEDIA_TYPE: Regex = Regex::new(r"Media type( *): (.+)").unwrap();

    static ref READ_MODE: Regex = Regex::new(r"Ripper mode( *): (.+)").unwrap();
    static ref READ_MODE_LEGACY: Regex = Regex::new(r"Use cdparanoia mode( *): (.+)").unwrap();
    static ref ACCURATE_STREAM: Regex = Regex::new(r"AccurateRip( v1| v2)? signature(\s*:) ([0-9A-F]{8})").unwrap();
    static ref DEFEAT_AUDIO_CACHE: Regex = Regex::new(r"Disable audio cache( *): (?P<boolean>OK|YES|NO)").unwrap();
    static ref USE_C2: Regex = Regex::new(r"Make use of C2 pointers( *): (?P<boolean>YES|NO)").unwrap();

    static ref READ_OFFSET_CORRECTION: Regex = Regex::new(r"Read offset correction( *): ([+-]?[0-9]+)").unwrap();
    static ref GAP_HANDLING: Regex = Regex::new(r"Gap status( *): (.+)").unwrap();

    static ref TEST_AND_COPY: Regex = Regex::new(r"CRC32 hash \(test run\)(\s*:) ([0-9A-F]{8})").unwrap();

    static ref CHECKSUM: Regex = Regex::new(r"\n-----BEGIN XLD SIGNATURE-----\n(.+)\n-----END XLD SIGNATURE-----").unwrap();
    static ref TOC: Regex = Regex::new(r"\s+(?P<track>\d+)\s+\|\s+(?P<start>[0-9:\.]+)\s+\|\s+(?P<length>[0-9:\.]+)\s+\|\s+(?P<start_sector>\d+)\s+\|\s+(?P<end_sector>\d+)").unwrap();
    
    static ref TRACKS: Regex = RegexBuilder::new(r"^Track \d+\s+(Filename|Pre-gap length)").multi_line(true).build().unwrap();
    static ref LOG_EOF: Regex = Regex::new(r"((No|Some) (errors|inconsistencies) (occurred|found)\s+)?End of status report").unwrap();

    // TODO: Some track fields that don't affect scoring were skipped
    // FIXME: There could be CRLF issues due to how regex crate dot_matches_new_line works
    // FIXME: This will definitely miss unusual encoders
    static ref TRACK_NUMBER: Regex = Regex::new(r"Track\s*(?P<value>\d+)").unwrap();
    static ref FILENAME: Regex = RegexBuilder::new(r"Filename(\s*):(\s*)(?P<value>(.+?)\.(flac|wav|mp3|m4a|ape|tta|ogg))").case_insensitive(true).dot_matches_new_line(true).build().unwrap();
    static ref FILENAME_MULTI: Regex = RegexBuilder::new(r"Filename(\s*):(\s*)(?P<value>((.+?)\.(flac|wav|mp3|m4a|ape|tta|ogg)(\r\n|\r|\n))+)").case_insensitive(true).build().unwrap();
    static ref PREGAP: Regex = Regex::new(r"Pre-gap length(\s*):(\s*)(?P<time>\d{2}:\d{2}:\d{2})").unwrap();
    static ref PEAK_LEVEL: Regex = Regex::new(r"Peak(\s*):(\s*)(?P<value>\d+\.\d+)").unwrap();
    static ref TEST_CRC: Regex = Regex::new(r"CRC32 hash \(test run\)(\s*):(\s*)(?P<value>[A-F0-9]{8})").unwrap();
    static ref COPY_CRC: Regex = Regex::new(r"CRC32 hash(\s*):(\s*)(?P<value>[A-F0-9]{8})").unwrap();
    static ref ERROR: Regex = Regex::new(r"(?P<type>Read error|Skipped \(treated as error\)|Damaged sector count|Inconsistency in error sectors|Missing samples|((Jitter error|Edge jitter error|Atom jitter error|Drift error|Dropped bytes error|Duplicated bytes error) \(maybe fixed\)))((\s*):(\s*)(?P<count>\d+))?").unwrap();
    static ref DAMAGED_SECTORS: Regex =  Regex::new(r"List of damaged sector positions\s*:(?:\s*\(\d+\)\s*\d+:\d+:\d+)+").unwrap();
    static ref SUSPICIOUS_POSITIONS: Regex = Regex::new(r"List of suspicious positions\s*:(?:\s*\(\d+\)\s*\d+:\d+:\d+)+").unwrap();
    static ref ERROR_TIME: Regex = Regex::new(r"\s*\(\d+\)\s*(?P<time>\d+:\d+:\d+)").unwrap();

    static ref AR_BLOCK: Regex = RegexBuilder::new(r"(?P<ar>AccurateRip( v\d+)? signature(.+))Statistics").dot_matches_new_line(true).build().unwrap();
    static ref AR_SIGNS: Regex = Regex::new(r"AccurateRip( v(?P<version>\d))? signature\s*:\s*(?P<sign>[A-F0-9]{8})( \((?P<off_sign>[A-F0-9]{8}) w/correction\))?").unwrap();
    static ref AR_FOUND: Regex = Regex::new(r"Accurately ripped(.+)\((?P<version>AR\d+|(v\d+(\+v\d+)*)), confidence (?P<cm>\d+(\+\d+)*)(/(?P<ct>\d+))?(, offset (?P<offset>-?\d+))?\)").unwrap();
}

pub struct XldParser {
    encoded_log: DecodedText,
}

struct XldParserSingle {
    log: String,
    translated_log: String,
    language: String,
}

impl XldParser {
    pub fn new(encoded_log: DecodedText) -> XldParser {
        XldParser {
            encoded_log,
        }
    }
}

struct XldParserTrack {
    is_range: bool,
    raw: String,
}

impl XldParserSingle {
    pub fn new(log: String) -> XldParserSingle {
        let (language, translated_log) = XldParserSingle::translate(log.clone());
        XldParserSingle {
            log,
            translated_log,
            language,
        }
    }

    fn boolean_matcher(&self, regex: &Regex) -> Quartet {
        let captures = regex.captures(&self.translated_log);
        match captures {
            Some(captures) => {
                let value = captures.name("boolean").unwrap().as_str();
                match value {
                    "YES" => Quartet::True,
                    "NO" => Quartet::False,
                    "OK" => Quartet::True,
                    _ => Quartet::Unknown,
                }
            },
            None => Quartet::Unknown,
        }
    }
}

impl ParserCombined for XldParser {
    fn parse_combined(&self) -> ParsedLogCombined {
        let parsed_logs: Vec<ParsedLog> = vec![XldParserSingle::new(self.encoded_log.text.trim().to_string()).parse()];

        ParsedLogCombined {
            parsed_logs,
            encoding: self.encoded_log.orig_encoding.to_string()
        }
    }
}

impl TranslatorCombined for XldParser {
    fn translate_combined(&self) -> String {
        self.encoded_log.text.clone()
    }
}

impl Parser for XldParserSingle {}

impl Extractor for XldParserSingle {
    fn extract_ripper(&self) -> Ripper {
        Ripper::XLD
    }

    fn extract_ripper_version(&self) -> String {
        let captures = RIPPER_VERSION.captures(&self.translated_log);
        match captures {
            Some(captures) => captures.get(1).unwrap().as_str().to_string(),
            None => String::from("Unknown"),
        }
    }

    fn extract_release_info(&self) -> ReleaseInfo {
        let captures: Option<regex::Captures<'_>> = RELEASE_INFO.captures(&self.translated_log);
        match captures {
            Some(captures) => {
                let split = captures.name("relinfo").unwrap().as_str().split_once(" / ");
                match split {
                    Some(s) => ReleaseInfo::new(s.0.trim().to_owned(), s.1.trim().to_owned()),
                    None => ReleaseInfo::default(),
                }
            },
            None => ReleaseInfo::default(),
        }
    }

    fn extract_read_offset(&self) -> Option<i16> {
        let captures = READ_OFFSET_CORRECTION.captures(&self.translated_log);
        captures.map(|captures| captures.get(2).unwrap().as_str().parse::<i16>().unwrap())
    }

    fn extract_language(&self) -> String {
        self.language.clone()
    }

    fn extract_drive(&self) -> String {
        let captures = USED_DRIVE.captures(&self.translated_log);
        match captures {
            Some(captures) => captures.get(2).unwrap().as_str().trim().to_string(),
            None => String::default(),
        }
    }

    fn extract_media_type(&self) -> MediaType {
        let captures = MEDIA_TYPE.captures(&self.translated_log);
        match captures {
            Some(captures) => {
                let value = captures.get(2).unwrap().as_str().trim();
                match value {
                    "Pressed CD" => MediaType::Pressed,
                    "CD-Recordable" => MediaType::CDR,
                    _ => MediaType::Other,
                }
            },
            None => MediaType::Unknown,
        }
    }

    fn extract_accurate_stream(&self) -> Quartet {
        let captures = ACCURATE_STREAM.captures(&self.translated_log);
        match captures {
            Some(_) => Quartet::True,
            None => Quartet::False,
        }
    }

    fn extract_defeat_audio_cache(&self) -> Quartet {
        self.boolean_matcher(&DEFEAT_AUDIO_CACHE)
    }

    fn extract_use_c2(&self) -> Quartet {
        self.boolean_matcher(&USE_C2)
    }

    fn extract_use_null_samples(&self) -> Quartet {
        Quartet::True
    }

    fn extract_test_and_copy(&self) -> Quartet {
        let captures = TEST_AND_COPY.captures(&self.translated_log);
        match captures {
            Some(_) => Quartet::True,
            None => Quartet::False,
        }
    }

    fn extract_read_mode(&self) -> ReadMode {
        let captures = READ_MODE.captures(&self.translated_log);
        let legacy_captures = READ_MODE_LEGACY.captures(&self.translated_log);
        match captures {
            Some(captures) => {
                let value = captures.get(2).unwrap().as_str().trim();
                match value {
                    "XLD Secure Ripper" => ReadMode::Secure,
                    "Burst" => ReadMode::Burst,
                    paranoid if paranoid.contains("CDParanoia") => ReadMode::Paranoid,
                    _ => ReadMode::Unknown,
                }
            },
            None => match legacy_captures {
                Some(legacy_captures) => {
                    let legacy_value = legacy_captures.get(2).unwrap().as_str();
                    match legacy_value {
                        paranoid if paranoid.contains("YES") => ReadMode::Paranoid,
                        _ => ReadMode::Burst,
                    }
                },
                None => ReadMode::Unknown,
            },
        }
    }

    fn extract_gap_handling(&self) -> Gap {
        let captures = GAP_HANDLING.captures(&self.translated_log);
        match captures {
            Some(captures) => {
                let value = captures.get(2).unwrap().as_str().trim();
                match value {
                    "Not analyzed" => Gap::Unknown,
                    "Analyzed, Appended (except HTOA)" => Gap::AppendNoHtoa,
                    "Analyzed, Appended" => Gap::Append,
                    _ => Gap::Unknown,
                }
            },
            None => Gap::Unknown,
        }
    }

    fn extract_audio_encoder(&self) -> Vec<String> {
        // No use checking all the tracks since this setting seems to be global for all the tracks
        let captures = FILENAME_MULTI.captures(&self.translated_log);
        match captures {
            Some(captures) => {
                let mut set: HashSet<String> = HashSet::new();
                let value = captures.name("value").unwrap().as_str().trim();

                for filename in value.lines() {
                    let extension = std::path::Path::new(filename.trim())
                                                .extension()
                                                .unwrap_or_default()
                                                .to_str()
                                                .unwrap_or_default()
                                                .to_ascii_lowercase();
                    if !extension.is_empty() {
                        set.insert(extension);
                    }
                }

                set.into_iter().collect()
            },
            None => Vec::new(),
        }
    }

    fn extract_toc(&self) -> Toc {
        let mut entries: Vec<TocEntry> = Vec::new();
        let captures_all = TOC.captures_iter(&self.translated_log);

        for captures in captures_all  {
            entries.push(TocEntry::new(
                str::parse(&captures["track"]).unwrap(),
                Time::from_mm_ss(&captures["start"]),
                Time::from_mm_ss(&captures["length"]),
                str::parse(&captures["start_sector"]).unwrap(),
                str::parse(&captures["end_sector"]).unwrap(),
            ))
        }

        Toc::new(TocRaw::new(entries))
    }

    fn extract_tracks(&self) -> Vec<TrackEntry> {
        let mut tracks: Vec<TrackEntry> = Vec::new();

        let last_idx = match LOG_EOF.find(&self.translated_log) {
            Some(idx) => idx.start(),
            None => return tracks,
        };

        let captures_all = TRACKS.captures_iter(&self.translated_log);
        let mut prev_start: usize = 0;
        let mut is_range = false;

        for (idx, captures) in captures_all.enumerate() {
            if let Some(m) = captures.get(0) {
                let start = m.start();

                // Match already exists, group 1 is bound to exist
                if !is_range && captures.get(1).unwrap().as_str() != "Filename" {
                    is_range = true;
                }

                if idx > 0 {
                    tracks.push(XldParserTrack::new(is_range, self.translated_log[prev_start..start].trim().to_owned()).parse_track());
                }

                prev_start = m.start();
            } else {
                break;
            }
        }

        if prev_start > 0 {
            tracks.push(XldParserTrack::new(is_range, self.translated_log[prev_start..last_idx].trim().to_owned()).parse_track());
        }

        tracks
    }
}

impl ParserTrack for XldParserTrack {}

impl Translator for XldParserSingle {
    fn translate(log: String) -> (String, String) {
        (String::from("English"), log)
    }
}

impl IntegrityChecker for XldParserSingle {
    fn extract_checksum(&self) -> String {
        let captures = CHECKSUM.captures(&self.translated_log);
        match captures {
            Some(captures) => captures.get(1).unwrap().as_str().trim().to_string(),
            None => String::new(),
        }
    }

    fn calculate_checksum(&self) -> String {
        let checksum_stripped = CHECKSUM.replace_all(&self.log, "");
        let mut hasher = Sha256Custom::new([0x1D95E3A4, 0x06520EF5, 0x3A9CFB75, 0x6104BCAE, 0x09CEDA82, 0xBA55E60B, 0xEAEC16C6, 0xEB19AF15]);
        let mut utf8bytes = checksum_stripped.as_bytes().to_vec();

        let mut enc = hasher.encrypt(&mut utf8bytes);
        enc.push_str("\nVersion=0001");
        let scrambled = Sha256Custom::scramble(&mut enc);
        
        let xld_b64_alphabet = Alphabet::new("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz._").unwrap();
        let xld_b64_engine = GeneralPurpose::new(&xld_b64_alphabet, PAD);

        xld_b64_engine.encode(scrambled).trim_end_matches('=').to_string()
    }
}

impl XldParserTrack {
    fn new(is_range: bool, raw: String) -> Self {
        XldParserTrack { is_range, raw }
    }

    fn string_match(&self, regex: &Regex) -> String {
        match regex.captures(&self.raw) {
            Some(val) => val.name("value").unwrap().as_str().trim().to_string(),
            None => String::default(),
        }
    }

    fn optional_match<T: FromStr>(&self, regex: &Regex) -> Option<T> 
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug, {
        regex.captures(&self.raw).map(|val| val.name("value").unwrap().as_str().trim().parse::<T>().unwrap())
    }
}

impl TrackExtractor for XldParserTrack {
    fn extract_num(&self) -> u8 {
        self.string_match(&TRACK_NUMBER).parse::<u8>().unwrap_or_default()
    }

    fn extract_is_range(&self) -> bool {
        self.is_range
    }

    fn extract_filenames(&self) -> Vec<String> {
        let mut filenames: Vec<String> = Vec::new();
        let first_file = self.string_match(&FILENAME);

        if first_file.is_empty() {
            return Vec::new();
        }

        filenames.push(first_file.clone());

        let captures = FILENAME_MULTI.captures(&self.raw);
        if let Some(c) = captures {
            let value = c.name("value").unwrap().as_str().trim();

            // FIXME: This is not gonna work with filename linebreaks
            for filename in value.lines().skip(1) {
                let filename = std::path::Path::new(filename.trim())
                                            .to_str()
                                            .unwrap_or_default();
                if !filename.is_empty() {
                    filenames.push(filename.to_owned())
                }
            }
        }

        filenames
    }

    fn extract_peak_level(&self) -> Option<f64> {
        self.optional_match::<f64>(&PEAK_LEVEL)
    }

    fn extract_pregap_length(&self) -> Option<Time> {
        let captures = PREGAP.captures(&self.raw);
        captures.map(|captures| Time::from_mm_ss_cs(captures.name("time").unwrap().as_str()))
    }

    fn extract_test_and_copy(&self) -> TestAndCopy {
        let test_crc = self.string_match(&TEST_CRC);
        let copy_crc = self.string_match(&COPY_CRC);

        // TODO: Redundant but parse and add skipzero hashes
        TestAndCopy::new_no_skipzero(test_crc, copy_crc)
    }

    // TODO: Feels kind of ugly
    fn extract_errors(&self) -> TrackError {
        let captures_all = ERROR.captures_iter(&self.raw);

        let (mut r_c, mut s_c, mut drf_c, mut drp_c, mut dup_c, mut dmg_c, mut inc_c) = (0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32);
        let (mut jg_c, mut je_c, mut ja_c) = (0_u32, 0_u32, 0_u32);
        let mut m_s = false;

        for captures in captures_all {
            let error_type = captures.name("type").unwrap().as_str();
            let count = match captures.name("count") {
                Some(m) => m.as_str().parse::<u32>().unwrap_or_default(),
                None => 0,
            };

            match error_type {
                "Read error" => { r_c = count },
                "Skipped (treated as error)" => { s_c = count },
                "Damaged sector count" => { dmg_c = count },
                "Jitter error (maybe fixed)" => { jg_c = count },
                "Edge jitter error (maybe fixed)" => { je_c = count },
                "Atom jitter error (maybe fixed)" => { ja_c = count },
                "Drift error (maybe fixed)" => { drf_c = count },
                "Dropped bytes error (maybe fixed)" => { drp_c = count },
                "Duplicated bytes error (maybe fixed)" => { dup_c = count },
                "Inconsistency in error sectors" => { inc_c = count },
                "Missing samples" => { m_s = true },
                _ => {}
            }
        }

        // Damaged sector positions
        let mut dmg_r: Vec<TrackErrorRange> = Vec::new();
        if let Some(c) = DAMAGED_SECTORS.captures(&self.raw) {
            let positions = ERROR_TIME.captures_iter(c.get(0).unwrap().as_str());

            for position in positions {
                dmg_r.push(
                    TrackErrorRange::new(
                        Time::from_mm_ss_cs(position.name("time").unwrap().as_str()),
                        Time::from_ss("0")
                    )
                );
            }
        }
        let dmg_d = TrackErrorData::new(dmg_c, dmg_r);

        // Suspicious positions
        let mut inc_r: Vec<TrackErrorRange> = Vec::new();
        if let Some(c) = SUSPICIOUS_POSITIONS.captures(&self.raw) {
            let positions = ERROR_TIME.captures_iter(c.get(0).unwrap().as_str());

            for position in positions {
                inc_r.push(
                    TrackErrorRange::new(
                        Time::from_mm_ss_cs(position.name("time").unwrap().as_str()),
                        Time::from_ss("0")
                    )
                );
            }
        }
        let inc_d = TrackErrorData::new(inc_c, inc_r);

        TrackError::new_xld(r_c, s_c, jg_c, je_c, ja_c, drf_c, drp_c, dup_c, dmg_d, inc_d, m_s)
    }

    fn extract_ar_info(&self) -> Vec<AccurateRipUnit> {
        let mut ars: Vec<AccurateRipUnit> = Vec::new();
        
        if let Some(c) = AR_BLOCK.captures(&self.raw) {
            let ar_raw = c.name("ar").unwrap().as_str();
            let ar_found = AR_FOUND.is_match(ar_raw);
            
            let mut conf_lut: HashMap<u8, AccurateRipConfidence> = HashMap::new();
            if let Some(ar_f_raw) = AR_FOUND.captures(ar_raw) {
                let confidence_versions = ar_f_raw
                    .name("version").unwrap().as_str()
                    .split('+')
                    .map(|m| m.trim_start_matches(char::is_alphabetic).parse::<u8>().unwrap_or_default())
                    .collect::<Vec<u8>>().into_iter();
                let confidence_matches = ar_f_raw
                    .name("cm").unwrap().as_str()
                    .split('+')
                    .map(|m| m.parse::<u32>().unwrap_or_default())
                    .collect::<Vec<u32>>().into_iter();
                let confidence_total = ar_f_raw.name("ct").map(|t| AccurateRipConfidenceTotal::All(t.as_str().parse().unwrap()));
                let offset = ar_f_raw.name("offset").map_or(AccurateRipOffset::Same, |t| AccurateRipOffset::Different(Some(t.as_str().parse().unwrap())));
                let conf_zip = zip(confidence_versions, confidence_matches);

                for (v, m) in conf_zip {
                    conf_lut.insert(v, AccurateRipConfidence::new(Some(m), confidence_total, offset));
                }
            }

            for sign_line in AR_SIGNS.captures_iter(ar_raw) {
                let mut version = sign_line.name("version").map(|v| v.as_str().parse::<u8>().unwrap());
                let sign = sign_line.name("sign").unwrap().as_str();
                let offset_sign = sign_line
                    .name("off_sign")
                    .map_or_else(|| if ar_found { sign } else { "" }, |os| os.as_str());

                // AccurateRip entry exists but legacy XLD logging
                // The only entry in the LUT will have the version
                if version.is_none() && ar_found {
                    version = Some(conf_lut.keys().next().unwrap().to_owned());
                }

                let confidence = conf_lut.remove(&version.unwrap_or_default());

                let ar = AccurateRipUnit::new_xld(
                    version,
                    sign.to_owned(),
                    offset_sign.to_owned(),
                    confidence,
                );

                ars.push(ar);
            }
        } else {
            ars.push(AccurateRipUnit::new_disabled());
        }
        
        ars
    }
}