// Python class definitions for PyO3
use cambia_core::{
    evaluate::{
        Evaluation, EvaluationCombined, EvaluationUnit, EvaluationUnitClass, EvaluationUnitData,
        EvaluationUnitField, EvaluationUnitScope, EvaluatorType,
    },
    extract::{Gap, MediaType, Quartet, ReadMode, ReleaseInfo, Ripper},
    integrity::{Checksum, Integrity},
    parser::{ParsedLog, ParsedLogCombined},
    response::CambiaResponse,
    toc::{Toc, TocEntry, TocHash, TocRaw},
    track::{
        AccurateRipConfidence, AccurateRipConfidenceTotal, AccurateRipOffset, AccurateRipStatus,
        AccurateRipUnit, TestAndCopy, TrackEntry, TrackError, TrackErrorData, TrackErrorRange,
    },
    util::Time,
};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pythonize::pythonize;
/// Convert a Time value to std::time::Duration via serde serialization.
/// Time serializes as f64 seconds, which we use to reconstruct a Duration.
fn time_to_duration(time: &Time) -> std::time::Duration {
    let secs = serde_json::to_value(time).unwrap().as_f64().unwrap();
    std::time::Duration::from_secs_f64(secs)
}

// ============= Enums =============

#[pyclass(name = "Ripper", eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum PyRipper {
    EAC,
    XLD,
    Whipper,
    CueRipper,
    DBPA,
    CyanRip,
    EZCD,
    Morituri,
    Rip,
    FreAc,
    Other,
}

#[pymethods]
impl PyRipper {
    #[getter]
    fn name(&self) -> &str {
        match self {
            PyRipper::EAC => "EAC",
            PyRipper::XLD => "XLD",
            PyRipper::Whipper => "Whipper",
            PyRipper::CueRipper => "CueRipper",
            PyRipper::DBPA => "DBPA",
            PyRipper::CyanRip => "CyanRip",
            PyRipper::EZCD => "EZCD",
            PyRipper::Morituri => "Morituri",
            PyRipper::Rip => "Rip",
            PyRipper::FreAc => "FreAc",
            PyRipper::Other => "Other",
        }
    }

    #[getter]
    fn value(&self, py: Python) -> PyResult<Py<PyAny>> {
        // Convert to original Rust enum and serialize with pythonize
        let rust_enum = match self {
            PyRipper::EAC => Ripper::EAC,
            PyRipper::XLD => Ripper::XLD,
            PyRipper::Whipper => Ripper::Whipper,
            PyRipper::CueRipper => Ripper::CueRipper,
            PyRipper::DBPA => Ripper::DBPA,
            PyRipper::CyanRip => Ripper::CyanRip,
            PyRipper::EZCD => Ripper::EZCD,
            PyRipper::Morituri => Ripper::Morituri,
            PyRipper::Rip => Ripper::Rip,
            PyRipper::FreAc => Ripper::FreAc,
            PyRipper::Other => Ripper::Other,
        };
        Ok(pythonize(py, &rust_enum)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?
            .unbind())
    }

    fn __repr__(&self) -> String {
        format!("<Ripper.{}>", self.name())
    }
}

impl From<&Ripper> for PyRipper {
    fn from(ripper: &Ripper) -> Self {
        match ripper {
            Ripper::EAC => PyRipper::EAC,
            Ripper::XLD => PyRipper::XLD,
            Ripper::Whipper => PyRipper::Whipper,
            Ripper::CueRipper => PyRipper::CueRipper,
            Ripper::DBPA => PyRipper::DBPA,
            Ripper::CyanRip => PyRipper::CyanRip,
            Ripper::EZCD => PyRipper::EZCD,
            Ripper::Morituri => PyRipper::Morituri,
            Ripper::Rip => PyRipper::Rip,
            Ripper::FreAc => PyRipper::FreAc,
            Ripper::Other => PyRipper::Other,
        }
    }
}

#[pyclass(name = "MediaType", eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum PyMediaType {
    Pressed,
    CDR,
    Other,
    Unknown,
}

#[pymethods]
impl PyMediaType {
    #[getter]
    fn name(&self) -> &str {
        match self {
            PyMediaType::Pressed => "Pressed",
            PyMediaType::CDR => "CDR",
            PyMediaType::Other => "Other",
            PyMediaType::Unknown => "Unknown",
        }
    }

    #[getter]
    fn value(&self, py: Python) -> PyResult<Py<PyAny>> {
        // Convert to original Rust enum and serialize with pythonize
        let rust_enum = match self {
            PyMediaType::Pressed => MediaType::Pressed,
            PyMediaType::CDR => MediaType::CDR,
            PyMediaType::Other => MediaType::Other,
            PyMediaType::Unknown => MediaType::Unknown,
        };
        Ok(pythonize(py, &rust_enum)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?
            .unbind())
    }

    fn __repr__(&self) -> String {
        format!("<MediaType.{}>", self.name())
    }
}

impl From<&MediaType> for PyMediaType {
    fn from(media: &MediaType) -> Self {
        match media {
            MediaType::Pressed => PyMediaType::Pressed,
            MediaType::CDR => PyMediaType::CDR,
            MediaType::Other => PyMediaType::Other,
            MediaType::Unknown => PyMediaType::Unknown,
        }
    }
}

#[pyclass(name = "Quartet", eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum PyQuartet {
    #[pyo3(name = "TRUE")]
    True,
    #[pyo3(name = "FALSE")]
    False,
    #[pyo3(name = "UNKNOWN")]
    Unknown,
    #[pyo3(name = "UNSUPPORTED")]
    Unsupported,
}

#[pymethods]
impl PyQuartet {
    #[getter]
    fn name(&self) -> &str {
        match self {
            PyQuartet::True => "True",
            PyQuartet::False => "False",
            PyQuartet::Unknown => "Unknown",
            PyQuartet::Unsupported => "Unsupported",
        }
    }

    #[getter]
    fn value(&self) -> &str {
        match self {
            PyQuartet::True => "True",
            PyQuartet::False => "False",
            PyQuartet::Unknown => "Unknown",
            PyQuartet::Unsupported => "Unsupported",
        }
    }

    fn __repr__(&self) -> String {
        format!("<Quartet.{}>", self.name())
    }
}

impl From<&Quartet> for PyQuartet {
    fn from(q: &Quartet) -> Self {
        match q {
            Quartet::True => PyQuartet::True,
            Quartet::False => PyQuartet::False,
            Quartet::Unknown => PyQuartet::Unknown,
            Quartet::Unsupported => PyQuartet::Unsupported,
        }
    }
}

#[pyclass(name = "ReadMode", eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum PyReadMode {
    Secure,
    Paranoid,
    Fast,
    Burst,
    Unknown,
}

#[pymethods]
impl PyReadMode {
    #[getter]
    fn name(&self) -> &str {
        match self {
            PyReadMode::Secure => "Secure",
            PyReadMode::Paranoid => "Paranoid",
            PyReadMode::Fast => "Fast",
            PyReadMode::Burst => "Burst",
            PyReadMode::Unknown => "Unknown",
        }
    }

    #[getter]
    fn value(&self, py: Python) -> PyResult<Py<PyAny>> {
        // Convert to original Rust enum and serialize with pythonize
        let rust_enum = match self {
            PyReadMode::Secure => ReadMode::Secure,
            PyReadMode::Paranoid => ReadMode::Paranoid,
            PyReadMode::Fast => ReadMode::Fast,
            PyReadMode::Burst => ReadMode::Burst,
            PyReadMode::Unknown => ReadMode::Unknown,
        };
        Ok(pythonize(py, &rust_enum)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?
            .unbind())
    }

    fn __repr__(&self) -> String {
        format!("<ReadMode.{}>", self.name())
    }
}

impl From<&ReadMode> for PyReadMode {
    fn from(mode: &ReadMode) -> Self {
        match mode {
            ReadMode::Secure => PyReadMode::Secure,
            ReadMode::Paranoid => PyReadMode::Paranoid,
            ReadMode::Fast => PyReadMode::Fast,
            ReadMode::Burst => PyReadMode::Burst,
            ReadMode::Unknown => PyReadMode::Unknown,
        }
    }
}

#[pyclass(name = "Gap", eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum PyGap {
    Append,
    AppendNoHtoa,
    AppendUndetected,
    Prepend,
    Discard,
    Unknown,
    Inapplicable,
}

#[pymethods]
impl PyGap {
    #[getter]
    fn name(&self) -> &str {
        match self {
            PyGap::Append => "Append",
            PyGap::AppendNoHtoa => "AppendNoHtoa",
            PyGap::AppendUndetected => "AppendUndetected",
            PyGap::Prepend => "Prepend",
            PyGap::Discard => "Discard",
            PyGap::Unknown => "Unknown",
            PyGap::Inapplicable => "Inapplicable",
        }
    }

    #[getter]
    fn value(&self, py: Python) -> PyResult<Py<PyAny>> {
        // Convert to original Rust enum and serialize with pythonize
        let rust_enum = match self {
            PyGap::Append => Gap::Append,
            PyGap::AppendNoHtoa => Gap::AppendNoHtoa,
            PyGap::AppendUndetected => Gap::AppendUndetected,
            PyGap::Prepend => Gap::Prepend,
            PyGap::Discard => Gap::Discard,
            PyGap::Unknown => Gap::Unknown,
            PyGap::Inapplicable => Gap::Inapplicable,
        };
        Ok(pythonize(py, &rust_enum)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?
            .unbind())
    }

    fn __repr__(&self) -> String {
        format!("<Gap.{}>", self.name())
    }
}

impl From<&Gap> for PyGap {
    fn from(gap: &Gap) -> Self {
        match gap {
            Gap::Append => PyGap::Append,
            Gap::AppendNoHtoa => PyGap::AppendNoHtoa,
            Gap::AppendUndetected => PyGap::AppendUndetected,
            Gap::Prepend => PyGap::Prepend,
            Gap::Discard => PyGap::Discard,
            Gap::Unknown => PyGap::Unknown,
            Gap::Inapplicable => PyGap::Inapplicable,
        }
    }
}

#[pyclass(name = "Integrity", eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum PyIntegrity {
    Match,
    Mismatch,
    Unknown,
}

#[pymethods]
impl PyIntegrity {
    #[getter]
    fn name(&self) -> &str {
        match self {
            PyIntegrity::Match => "Match",
            PyIntegrity::Mismatch => "Mismatch",
            PyIntegrity::Unknown => "Unknown",
        }
    }

    #[getter]
    fn value(&self, py: Python) -> PyResult<Py<PyAny>> {
        // Convert to original Rust enum and serialize with pythonize
        let rust_enum = match self {
            PyIntegrity::Match => Integrity::Match,
            PyIntegrity::Mismatch => Integrity::Mismatch,
            PyIntegrity::Unknown => Integrity::Unknown,
        };
        Ok(pythonize(py, &rust_enum)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?
            .unbind())
    }

    fn __repr__(&self) -> String {
        format!("<Integrity.{}>", self.name())
    }
}

impl From<&Integrity> for PyIntegrity {
    fn from(integrity: &Integrity) -> Self {
        match integrity {
            Integrity::Match => PyIntegrity::Match,
            Integrity::Mismatch => PyIntegrity::Mismatch,
            Integrity::Unknown => PyIntegrity::Unknown,
        }
    }
}

#[pyclass(name = "AccurateRipStatus", eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum PyAccurateRipStatus {
    Match,
    Mismatch,
    Offsetted,
    NotFound,
    Disabled,
}

#[pymethods]
impl PyAccurateRipStatus {
    #[getter]
    fn name(&self) -> &str {
        match self {
            PyAccurateRipStatus::Match => "Match",
            PyAccurateRipStatus::Mismatch => "Mismatch",
            PyAccurateRipStatus::Offsetted => "Offsetted",
            PyAccurateRipStatus::NotFound => "NotFound",
            PyAccurateRipStatus::Disabled => "Disabled",
        }
    }

    #[getter]
    fn value(&self, py: Python) -> PyResult<Py<PyAny>> {
        // Convert to original Rust enum and serialize with pythonize
        let rust_enum = match self {
            PyAccurateRipStatus::Match => AccurateRipStatus::Match,
            PyAccurateRipStatus::Mismatch => AccurateRipStatus::Mismatch,
            PyAccurateRipStatus::Offsetted => AccurateRipStatus::Offsetted,
            PyAccurateRipStatus::NotFound => AccurateRipStatus::NotFound,
            PyAccurateRipStatus::Disabled => AccurateRipStatus::Disabled,
        };
        Ok(pythonize(py, &rust_enum)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?
            .unbind())
    }

    fn __repr__(&self) -> String {
        format!("<AccurateRipStatus.{}>", self.name())
    }
}

impl From<&AccurateRipStatus> for PyAccurateRipStatus {
    fn from(status: &AccurateRipStatus) -> Self {
        match status {
            AccurateRipStatus::Match => PyAccurateRipStatus::Match,
            AccurateRipStatus::Mismatch => PyAccurateRipStatus::Mismatch,
            AccurateRipStatus::Offsetted => PyAccurateRipStatus::Offsetted,
            AccurateRipStatus::NotFound => PyAccurateRipStatus::NotFound,
            AccurateRipStatus::Disabled => PyAccurateRipStatus::Disabled,
        }
    }
}

#[pyclass(name = "EvaluatorType", eq, eq_int)]
#[derive(Clone, PartialEq)]
pub enum PyEvaluatorType {
    Cambia,
    RED,
    OPS,
}

#[pymethods]
impl PyEvaluatorType {
    #[getter]
    fn name(&self) -> &str {
        match self {
            PyEvaluatorType::Cambia => "Cambia",
            PyEvaluatorType::RED => "RED",
            PyEvaluatorType::OPS => "OPS",
        }
    }

    #[getter]
    fn value(&self, py: Python) -> PyResult<Py<PyAny>> {
        // Convert to original Rust enum and serialize with pythonize
        let rust_enum = match self {
            PyEvaluatorType::Cambia => EvaluatorType::Cambia,
            PyEvaluatorType::RED => EvaluatorType::RED,
            PyEvaluatorType::OPS => EvaluatorType::OPS,
        };
        Ok(pythonize(py, &rust_enum)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?
            .unbind())
    }

    fn __repr__(&self) -> String {
        format!("<EvaluatorType.{}>", self.name())
    }
}

impl From<&EvaluatorType> for PyEvaluatorType {
    fn from(evaluator: &EvaluatorType) -> Self {
        match evaluator {
            EvaluatorType::Cambia => PyEvaluatorType::Cambia,
            EvaluatorType::RED => PyEvaluatorType::RED,
            EvaluatorType::OPS => PyEvaluatorType::OPS,
        }
    }
}

// ============= TOC Classes =============

#[pyclass(name = "TocEntry")]
#[derive(Clone)]
pub struct PyTocEntry {
    #[pyo3(get)]
    pub track: u32,
    #[pyo3(get)]
    pub start: std::time::Duration,
    #[pyo3(get)]
    pub length: std::time::Duration,
    #[pyo3(get)]
    pub start_sector: u32,
    #[pyo3(get)]
    pub end_sector: u32,
}

impl From<&TocEntry> for PyTocEntry {
    fn from(entry: &TocEntry) -> Self {
        PyTocEntry {
            track: entry.track,
            start: time_to_duration(&entry.start),
            length: time_to_duration(&entry.length),
            start_sector: entry.start_sector,
            end_sector: entry.end_sector,
        }
    }
}

#[pymethods]
impl PyTocEntry {
    fn __repr__(&self) -> String {
        format!(
            "<TocEntry track={} start_sector={} end_sector={}>",
            self.track, self.start_sector, self.end_sector
        )
    }
}

#[pyclass(name = "TocHash")]
#[derive(Clone)]
pub struct PyTocHash {
    #[pyo3(get)]
    pub hash: String,
    #[pyo3(get)]
    pub url: String,
}

impl From<&TocHash> for PyTocHash {
    fn from(hash: &TocHash) -> Self {
        PyTocHash {
            hash: hash.hash.clone(),
            url: hash.url.clone(),
        }
    }
}

#[pymethods]
impl PyTocHash {
    fn __repr__(&self) -> String {
        format!("<TocHash hash='{}'>", self.hash)
    }
}

#[pyclass(name = "TocRaw")]
#[derive(Clone)]
pub struct PyTocRaw {
    #[pyo3(get)]
    pub entries: Vec<PyTocEntry>,
}

impl From<&TocRaw> for PyTocRaw {
    fn from(raw: &TocRaw) -> Self {
        PyTocRaw {
            entries: raw.entries.iter().map(PyTocEntry::from).collect(),
        }
    }
}

#[pymethods]
impl PyTocRaw {
    fn __repr__(&self) -> String {
        format!("<TocRaw entries={}>", self.entries.len())
    }
}

#[pyclass(name = "Toc")]
#[derive(Clone)]
pub struct PyToc {
    #[pyo3(get)]
    pub raw: PyTocRaw,
    #[pyo3(get)]
    pub freedb: PyTocHash,
    #[pyo3(get)]
    pub accurip_tocid: PyTocHash,
    #[pyo3(get)]
    pub ctdb_tocid: PyTocHash,
    #[pyo3(get)]
    pub mbz: PyTocHash,
    #[pyo3(get)]
    pub gn: PyTocHash,
    #[pyo3(get)]
    pub mcdi: PyTocHash,
}

impl From<&Toc> for PyToc {
    fn from(toc: &Toc) -> Self {
        PyToc {
            raw: PyTocRaw::from(&toc.raw),
            freedb: PyTocHash::from(&toc.freedb),
            accurip_tocid: PyTocHash::from(&toc.accurip_tocid),
            ctdb_tocid: PyTocHash::from(&toc.ctdb_tocid),
            mbz: PyTocHash::from(&toc.mbz),
            gn: PyTocHash::from(&toc.gn),
            mcdi: PyTocHash::from(&toc.mcdi),
        }
    }
}

#[pymethods]
impl PyToc {
    fn __repr__(&self) -> String {
        format!("<Toc entries={}>", self.raw.entries.len())
    }
}

// ============= Checksum and Integrity =============

#[pyclass(name = "Checksum")]
#[derive(Clone)]
pub struct PyChecksum {
    #[pyo3(get)]
    pub calculated: String,
    #[pyo3(get)]
    pub log: String,
    #[pyo3(get)]
    pub integrity: PyIntegrity,
}

impl From<&Checksum> for PyChecksum {
    fn from(checksum: &Checksum) -> Self {
        PyChecksum {
            calculated: checksum.calculated.clone(),
            log: checksum.log.clone(),
            integrity: PyIntegrity::from(&checksum.integrity),
        }
    }
}

#[pymethods]
impl PyChecksum {
    fn __repr__(&self) -> String {
        format!("<Checksum integrity={}>", self.integrity.name())
    }
}

// ============= Release Info =============

#[pyclass(name = "ReleaseInfo")]
#[derive(Clone)]
pub struct PyReleaseInfo {
    #[pyo3(get)]
    pub artist: String,
    #[pyo3(get)]
    pub title: String,
}

impl From<&ReleaseInfo> for PyReleaseInfo {
    fn from(info: &ReleaseInfo) -> Self {
        PyReleaseInfo {
            artist: info.artist.clone(),
            title: info.title.clone(),
        }
    }
}

#[pymethods]
impl PyReleaseInfo {
    fn __repr__(&self) -> String {
        format!(
            "<ReleaseInfo artist='{}' title='{}'>",
            self.artist, self.title
        )
    }
}

// ============= Track Classes =============

#[pyclass(name = "TrackErrorRange")]
#[derive(Clone)]
pub struct PyTrackErrorRange {
    #[pyo3(get)]
    pub start: std::time::Duration,
    #[pyo3(get)]
    pub length: std::time::Duration,
}

impl From<&TrackErrorRange> for PyTrackErrorRange {
    fn from(range: &TrackErrorRange) -> Self {
        PyTrackErrorRange {
            start: time_to_duration(&range.start),
            length: time_to_duration(&range.length),
        }
    }
}

#[pymethods]
impl PyTrackErrorRange {
    fn __repr__(&self) -> String {
        format!(
            "<TrackErrorRange start={:?} length={:?}>",
            self.start, self.length
        )
    }
}

#[pyclass(name = "TrackErrorData")]
#[derive(Clone)]
pub struct PyTrackErrorData {
    #[pyo3(get)]
    pub count: u32,
    #[pyo3(get)]
    pub ranges: Vec<PyTrackErrorRange>,
}

impl From<&TrackErrorData> for PyTrackErrorData {
    fn from(data: &TrackErrorData) -> Self {
        PyTrackErrorData {
            count: data.count,
            ranges: data.ranges.iter().map(PyTrackErrorRange::from).collect(),
        }
    }
}

#[pymethods]
impl PyTrackErrorData {
    fn __repr__(&self) -> String {
        format!("<TrackErrorData count={}>", self.count)
    }
}

#[pyclass(name = "TrackError")]
#[derive(Clone)]
pub struct PyTrackError {
    #[pyo3(get)]
    pub read: PyTrackErrorData,
    #[pyo3(get)]
    pub skip: PyTrackErrorData,
    #[pyo3(get)]
    pub jitter_generic: PyTrackErrorData,
    #[pyo3(get)]
    pub jitter_edge: PyTrackErrorData,
    #[pyo3(get)]
    pub jitter_atom: PyTrackErrorData,
    #[pyo3(get)]
    pub drift: PyTrackErrorData,
    #[pyo3(get)]
    pub dropped: PyTrackErrorData,
    #[pyo3(get)]
    pub duplicated: PyTrackErrorData,
    #[pyo3(get)]
    pub damaged_sectors: PyTrackErrorData,
    #[pyo3(get)]
    pub inconsistent_err_sectors: PyTrackErrorData,
    #[pyo3(get)]
    pub missing_samples: PyTrackErrorData,
}

impl From<&TrackError> for PyTrackError {
    fn from(error: &TrackError) -> Self {
        PyTrackError {
            read: PyTrackErrorData::from(&error.read),
            skip: PyTrackErrorData::from(&error.skip),
            jitter_generic: PyTrackErrorData::from(&error.jitter_generic),
            jitter_edge: PyTrackErrorData::from(&error.jitter_edge),
            jitter_atom: PyTrackErrorData::from(&error.jitter_atom),
            drift: PyTrackErrorData::from(&error.drift),
            dropped: PyTrackErrorData::from(&error.dropped),
            duplicated: PyTrackErrorData::from(&error.duplicated),
            damaged_sectors: PyTrackErrorData::from(&error.damaged_sectors),
            inconsistent_err_sectors: PyTrackErrorData::from(&error.inconsistent_err_sectors),
            missing_samples: PyTrackErrorData::from(&error.missing_samples),
        }
    }
}

#[pymethods]
impl PyTrackError {
    fn __repr__(&self) -> String {
        format!(
            "<TrackError read={} skip={} jitter={}>",
            self.read.count, self.skip.count, self.jitter_generic.count
        )
    }
}

#[pyclass(name = "AccurateRipConfidence")]
#[derive(Clone)]
pub struct PyAccurateRipConfidence {
    #[pyo3(get)]
    pub matching: Option<u32>,
    #[pyo3(get)]
    pub total: Option<u32>,
    #[pyo3(get)]
    pub offset: String,
}

impl From<&AccurateRipConfidence> for PyAccurateRipConfidence {
    fn from(conf: &AccurateRipConfidence) -> Self {
        let total = match conf.total {
            Some(AccurateRipConfidenceTotal::All(n)) => Some(n),
            Some(AccurateRipConfidenceTotal::Version(n)) => Some(n),
            None => None,
        };

        let offset = match conf.offset {
            AccurateRipOffset::Same => "Same".to_string(),
            AccurateRipOffset::Different(Some(n)) => format!("Different ({})", n),
            AccurateRipOffset::Different(None) => "Different".to_string(),
        };

        PyAccurateRipConfidence {
            matching: conf.matching,
            total,
            offset,
        }
    }
}

#[pymethods]
impl PyAccurateRipConfidence {
    fn __repr__(&self) -> String {
        format!(
            "<AccurateRipConfidence matching={:?} total={:?}>",
            self.matching, self.total
        )
    }
}

#[pyclass(name = "AccurateRipUnit")]
#[derive(Clone)]
pub struct PyAccurateRipUnit {
    #[pyo3(get)]
    pub status: PyAccurateRipStatus,
    #[pyo3(get)]
    pub confidence: Option<PyAccurateRipConfidence>,
    #[pyo3(get)]
    pub sign: String,
    #[pyo3(get)]
    pub version: Option<u8>,
}

impl From<&AccurateRipUnit> for PyAccurateRipUnit {
    fn from(ar: &AccurateRipUnit) -> Self {
        PyAccurateRipUnit {
            status: PyAccurateRipStatus::from(&ar.status),
            confidence: ar.confidence.as_ref().map(PyAccurateRipConfidence::from),
            sign: ar.sign.clone(),
            version: ar.version,
        }
    }
}

#[pymethods]
impl PyAccurateRipUnit {
    fn __repr__(&self) -> String {
        format!("<AccurateRipUnit status={}>", self.status.name())
    }
}

#[pyclass(name = "TestAndCopy")]
#[derive(Clone)]
pub struct PyTestAndCopy {
    #[pyo3(get)]
    pub test_hash: String,
    #[pyo3(get)]
    pub copy_hash: String,
    #[pyo3(get)]
    pub integrity: PyIntegrity,
}

impl From<&TestAndCopy> for PyTestAndCopy {
    fn from(tc: &TestAndCopy) -> Self {
        PyTestAndCopy {
            test_hash: tc.test_hash.clone(),
            copy_hash: tc.copy_hash.clone(),
            integrity: PyIntegrity::from(&tc.integrity),
        }
    }
}

#[pymethods]
impl PyTestAndCopy {
    fn __repr__(&self) -> String {
        format!("<TestAndCopy integrity={}>", self.integrity.name())
    }
}

#[pyclass(name = "TrackEntry")]
#[derive(Clone)]
pub struct PyTrackEntry {
    #[pyo3(get)]
    pub num: u8,
    #[pyo3(get)]
    pub is_range: bool,
    #[pyo3(get)]
    pub aborted: bool,
    #[pyo3(get)]
    pub filenames: Vec<String>,
    #[pyo3(get)]
    pub peak_level: Option<f64>,
    #[pyo3(get)]
    pub pregap_length: Option<std::time::Duration>,
    #[pyo3(get)]
    pub extraction_speed: Option<f64>,
    #[pyo3(get)]
    pub gain: Option<f64>,
    #[pyo3(get)]
    pub preemphasis: Option<bool>,
    #[pyo3(get)]
    pub test_and_copy: PyTestAndCopy,
    #[pyo3(get)]
    pub errors: PyTrackError,
    #[pyo3(get)]
    pub ar_info: Vec<PyAccurateRipUnit>,
}

impl From<&TrackEntry> for PyTrackEntry {
    fn from(entry: &TrackEntry) -> Self {
        PyTrackEntry {
            num: entry.num,
            is_range: entry.is_range,
            aborted: entry.aborted,
            filenames: entry.filenames.clone(),
            peak_level: entry.peak_level,
            pregap_length: entry.pregap_length.as_ref().map(time_to_duration),
            extraction_speed: entry.extraction_speed,
            gain: entry.gain,
            preemphasis: entry.preemphasis,
            test_and_copy: PyTestAndCopy::from(&entry.test_and_copy),
            errors: PyTrackError::from(&entry.errors),
            ar_info: entry.ar_info.iter().map(PyAccurateRipUnit::from).collect(),
        }
    }
}

#[pymethods]
impl PyTrackEntry {
    fn __repr__(&self) -> String {
        format!("<TrackEntry num={} aborted={}>", self.num, self.aborted)
    }
}

// ============= Parsed Log =============

#[pyclass(name = "ParsedLog")]
#[derive(Clone)]
pub struct PyParsedLog {
    #[pyo3(get)]
    pub ripper: PyRipper,
    #[pyo3(get)]
    pub ripper_version: String,
    #[pyo3(get)]
    pub release_info: PyReleaseInfo,
    #[pyo3(get)]
    pub language: String,
    #[pyo3(get)]
    pub read_offset: Option<i16>,
    #[pyo3(get)]
    pub combined_rw_offset: Option<i32>,
    #[pyo3(get)]
    pub drive: String,
    #[pyo3(get)]
    pub media_type: PyMediaType,
    #[pyo3(get)]
    pub accurate_stream: PyQuartet,
    #[pyo3(get)]
    pub defeat_audio_cache: PyQuartet,
    #[pyo3(get)]
    pub use_c2: PyQuartet,
    #[pyo3(get)]
    pub overread: PyQuartet,
    #[pyo3(get)]
    pub fill_silence: PyQuartet,
    #[pyo3(get)]
    pub delete_silence: PyQuartet,
    #[pyo3(get)]
    pub use_null_samples: PyQuartet,
    #[pyo3(get)]
    pub test_and_copy: PyQuartet,
    #[pyo3(get)]
    pub normalize: PyQuartet,
    #[pyo3(get)]
    pub read_mode: PyReadMode,
    #[pyo3(get)]
    pub gap_handling: PyGap,
    #[pyo3(get)]
    pub checksum: PyChecksum,
    #[pyo3(get)]
    pub toc: PyToc,
    #[pyo3(get)]
    pub tracks: Vec<PyTrackEntry>,
    #[pyo3(get)]
    pub id3_enabled: PyQuartet,
    #[pyo3(get)]
    pub audio_encoder: Vec<String>,
}

impl PyParsedLog {
    pub fn from_log(log: &ParsedLog) -> Self {
        let tracks = log.tracks.iter().map(PyTrackEntry::from).collect();

        PyParsedLog {
            ripper: PyRipper::from(&log.ripper),
            ripper_version: log.ripper_version.clone(),
            release_info: PyReleaseInfo::from(&log.release_info),
            language: log.language.clone(),
            read_offset: log.read_offset,
            combined_rw_offset: log.combined_rw_offset,
            drive: log.drive.clone(),
            media_type: PyMediaType::from(&log.media_type),
            accurate_stream: PyQuartet::from(&log.accurate_stream),
            defeat_audio_cache: PyQuartet::from(&log.defeat_audio_cache),
            use_c2: PyQuartet::from(&log.use_c2),
            overread: PyQuartet::from(&log.overread),
            fill_silence: PyQuartet::from(&log.fill_silence),
            delete_silence: PyQuartet::from(&log.delete_silence),
            use_null_samples: PyQuartet::from(&log.use_null_samples),
            test_and_copy: PyQuartet::from(&log.test_and_copy),
            normalize: PyQuartet::from(&log.normalize),
            read_mode: PyReadMode::from(&log.read_mode),
            gap_handling: PyGap::from(&log.gap_handling),
            checksum: PyChecksum::from(&log.checksum),
            toc: PyToc::from(&log.toc),
            tracks,
            id3_enabled: PyQuartet::from(&log.id3_enabled),
            audio_encoder: log.audio_encoder.clone(),
        }
    }
}

#[pymethods]
impl PyParsedLog {
    fn __repr__(&self) -> String {
        format!(
            "<ParsedLog ripper={} tracks={}>",
            self.ripper.name(),
            self.tracks.len()
        )
    }
}

// ============= Parsed Combined =============

#[pyclass(name = "ParsedLogCombined")]
#[derive(Clone)]
pub struct PyParsedLogCombined {
    #[pyo3(get)]
    pub encoding: String,
    #[pyo3(get)]
    pub parsed_logs: Vec<PyParsedLog>,
}

impl PyParsedLogCombined {
    pub fn from_combined(combined: &ParsedLogCombined) -> Self {
        let parsed_logs = combined
            .parsed_logs
            .iter()
            .map(|log| PyParsedLog::from_log(log))
            .collect();

        PyParsedLogCombined {
            encoding: combined.encoding.clone(),
            parsed_logs,
        }
    }
}

#[pymethods]
impl PyParsedLogCombined {
    fn __repr__(&self) -> String {
        format!(
            "<ParsedLogCombined encoding='{}' logs={}>",
            self.encoding,
            self.parsed_logs.len()
        )
    }
}

// ============= Evaluation Classes =============

#[pyclass(name = "EvaluationUnitScope")]
#[derive(Clone, PartialEq)]
pub enum PyEvaluationUnitScope {
    Release(),
    Track(Option<u8>),
}

#[pymethods]
impl PyEvaluationUnitScope {
    #[getter]
    fn name(&self) -> &str {
        match self {
            PyEvaluationUnitScope::Release() => "Release",
            PyEvaluationUnitScope::Track(_) => "Track",
        }
    }

    #[getter]
    fn value(&self) -> String {
        match self {
            PyEvaluationUnitScope::Release() => "Release".to_string(),
            PyEvaluationUnitScope::Track(track_num) => {
                if let Some(num) = track_num {
                    format!("Track {}", num)
                } else {
                    "Track".to_string()
                }
            }
        }
    }
}

impl From<&EvaluationUnitScope> for PyEvaluationUnitScope {
    fn from(scope: &EvaluationUnitScope) -> Self {
        match scope {
            EvaluationUnitScope::Release => PyEvaluationUnitScope::Release(),
            EvaluationUnitScope::Track(track_num) => PyEvaluationUnitScope::Track(*track_num),
        }
    }
}

#[pyclass(name = "EvaluationUnitField")]
#[derive(Clone, PartialEq)]
pub enum PyEvaluationUnitField {
    Encoding,
    RipperVersion,
    Drive,
    Ripper,
    Offset,
    Cache,
    TestAndCopy,
    Encoder,
    Checksum,
    MediaType,
    ReadMode,
    MaxRetryCount,
    AccurateStream,
    C2,
    SilentSamples,
    NullSamples,
    Gap,
    Tag,
    Gain,
    RangeSplit,
    Samples,
    SilentBlocks,
    Normalization,
    Filename,
    ReadError,
    SkipError,
    JitterGenericError,
    JitterEdgeError,
    JitterAtomError,
    DriftError,
    DroppedError,
    DuplicatedError,
    InconsistentErrorSectors,
    DamagedSector,
    Abort,
}

#[pymethods]
impl PyEvaluationUnitField {
    #[getter]
    fn name(&self) -> &str {
        match self {
            PyEvaluationUnitField::Encoding => "Encoding",
            PyEvaluationUnitField::RipperVersion => "RipperVersion",
            PyEvaluationUnitField::Drive => "Drive",
            PyEvaluationUnitField::Ripper => "Ripper",
            PyEvaluationUnitField::Offset => "Offset",
            PyEvaluationUnitField::Cache => "Cache",
            PyEvaluationUnitField::TestAndCopy => "TestAndCopy",
            PyEvaluationUnitField::Encoder => "Encoder",
            PyEvaluationUnitField::Checksum => "Checksum",
            PyEvaluationUnitField::MediaType => "MediaType",
            PyEvaluationUnitField::ReadMode => "ReadMode",
            PyEvaluationUnitField::MaxRetryCount => "MaxRetryCount",
            PyEvaluationUnitField::AccurateStream => "AccurateStream",
            PyEvaluationUnitField::C2 => "C2",
            PyEvaluationUnitField::SilentSamples => "SilentSamples",
            PyEvaluationUnitField::NullSamples => "NullSamples",
            PyEvaluationUnitField::Gap => "Gap",
            PyEvaluationUnitField::Tag => "Tag",
            PyEvaluationUnitField::Gain => "Gain",
            PyEvaluationUnitField::RangeSplit => "RangeSplit",
            PyEvaluationUnitField::Samples => "Samples",
            PyEvaluationUnitField::SilentBlocks => "SilentBlocks",
            PyEvaluationUnitField::Normalization => "Normalization",
            PyEvaluationUnitField::Filename => "Filename",
            PyEvaluationUnitField::ReadError => "ReadError",
            PyEvaluationUnitField::SkipError => "SkipError",
            PyEvaluationUnitField::JitterGenericError => "JitterGenericError",
            PyEvaluationUnitField::JitterEdgeError => "JitterEdgeError",
            PyEvaluationUnitField::JitterAtomError => "JitterAtomError",
            PyEvaluationUnitField::DriftError => "DriftError",
            PyEvaluationUnitField::DroppedError => "DroppedError",
            PyEvaluationUnitField::DuplicatedError => "DuplicatedError",
            PyEvaluationUnitField::InconsistentErrorSectors => "InconsistentErrorSectors",
            PyEvaluationUnitField::DamagedSector => "DamagedSector",
            PyEvaluationUnitField::Abort => "Abort",
        }
    }

    #[getter]
    fn value(&self, py: Python) -> PyResult<Py<PyAny>> {
        // Convert to original Rust enum and serialize with pythonize
        let rust_enum = match self {
            PyEvaluationUnitField::Encoding => EvaluationUnitField::Encoding,
            PyEvaluationUnitField::RipperVersion => EvaluationUnitField::RipperVersion,
            PyEvaluationUnitField::Drive => EvaluationUnitField::Drive,
            PyEvaluationUnitField::Ripper => EvaluationUnitField::Ripper,
            PyEvaluationUnitField::Offset => EvaluationUnitField::Offset,
            PyEvaluationUnitField::Cache => EvaluationUnitField::Cache,
            PyEvaluationUnitField::TestAndCopy => EvaluationUnitField::TestAndCopy,
            PyEvaluationUnitField::Encoder => EvaluationUnitField::Encoder,
            PyEvaluationUnitField::Checksum => EvaluationUnitField::Checksum,
            PyEvaluationUnitField::MediaType => EvaluationUnitField::MediaType,
            PyEvaluationUnitField::ReadMode => EvaluationUnitField::ReadMode,
            PyEvaluationUnitField::MaxRetryCount => EvaluationUnitField::MaxRetryCount,
            PyEvaluationUnitField::AccurateStream => EvaluationUnitField::AccurateStream,
            PyEvaluationUnitField::C2 => EvaluationUnitField::C2,
            PyEvaluationUnitField::SilentSamples => EvaluationUnitField::SilentSamples,
            PyEvaluationUnitField::NullSamples => EvaluationUnitField::NullSamples,
            PyEvaluationUnitField::Gap => EvaluationUnitField::Gap,
            PyEvaluationUnitField::Tag => EvaluationUnitField::Tag,
            PyEvaluationUnitField::Gain => EvaluationUnitField::Gain,
            PyEvaluationUnitField::RangeSplit => EvaluationUnitField::RangeSplit,
            PyEvaluationUnitField::Samples => EvaluationUnitField::Samples,
            PyEvaluationUnitField::SilentBlocks => EvaluationUnitField::SilentBlocks,
            PyEvaluationUnitField::Normalization => EvaluationUnitField::Normalization,
            PyEvaluationUnitField::Filename => EvaluationUnitField::Filename,
            PyEvaluationUnitField::ReadError => EvaluationUnitField::ReadError,
            PyEvaluationUnitField::SkipError => EvaluationUnitField::SkipError,
            PyEvaluationUnitField::JitterGenericError => EvaluationUnitField::JitterGenericError,
            PyEvaluationUnitField::JitterEdgeError => EvaluationUnitField::JitterEdgeError,
            PyEvaluationUnitField::JitterAtomError => EvaluationUnitField::JitterAtomError,
            PyEvaluationUnitField::DriftError => EvaluationUnitField::DriftError,
            PyEvaluationUnitField::DroppedError => EvaluationUnitField::DroppedError,
            PyEvaluationUnitField::DuplicatedError => EvaluationUnitField::DuplicatedError,
            PyEvaluationUnitField::InconsistentErrorSectors => {
                EvaluationUnitField::InconsistentErrorSectors
            }
            PyEvaluationUnitField::DamagedSector => EvaluationUnitField::DamagedSector,
            PyEvaluationUnitField::Abort => EvaluationUnitField::Abort,
        };
        Ok(pythonize(py, &rust_enum)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?
            .unbind())
    }

    fn __repr__(&self) -> String {
        format!("<EvaluationUnitField.{}>", self.name())
    }
}

impl From<&EvaluationUnitField> for PyEvaluationUnitField {
    fn from(field: &EvaluationUnitField) -> Self {
        match field {
            EvaluationUnitField::Encoding => PyEvaluationUnitField::Encoding,
            EvaluationUnitField::RipperVersion => PyEvaluationUnitField::RipperVersion,
            EvaluationUnitField::Drive => PyEvaluationUnitField::Drive,
            EvaluationUnitField::Ripper => PyEvaluationUnitField::Ripper,
            EvaluationUnitField::Offset => PyEvaluationUnitField::Offset,
            EvaluationUnitField::Cache => PyEvaluationUnitField::Cache,
            EvaluationUnitField::TestAndCopy => PyEvaluationUnitField::TestAndCopy,
            EvaluationUnitField::Encoder => PyEvaluationUnitField::Encoder,
            EvaluationUnitField::Checksum => PyEvaluationUnitField::Checksum,
            EvaluationUnitField::MediaType => PyEvaluationUnitField::MediaType,
            EvaluationUnitField::ReadMode => PyEvaluationUnitField::ReadMode,
            EvaluationUnitField::MaxRetryCount => PyEvaluationUnitField::MaxRetryCount,
            EvaluationUnitField::AccurateStream => PyEvaluationUnitField::AccurateStream,
            EvaluationUnitField::C2 => PyEvaluationUnitField::C2,
            EvaluationUnitField::SilentSamples => PyEvaluationUnitField::SilentSamples,
            EvaluationUnitField::NullSamples => PyEvaluationUnitField::NullSamples,
            EvaluationUnitField::Gap => PyEvaluationUnitField::Gap,
            EvaluationUnitField::Tag => PyEvaluationUnitField::Tag,
            EvaluationUnitField::Gain => PyEvaluationUnitField::Gain,
            EvaluationUnitField::RangeSplit => PyEvaluationUnitField::RangeSplit,
            EvaluationUnitField::Samples => PyEvaluationUnitField::Samples,
            EvaluationUnitField::SilentBlocks => PyEvaluationUnitField::SilentBlocks,
            EvaluationUnitField::Normalization => PyEvaluationUnitField::Normalization,
            EvaluationUnitField::Filename => PyEvaluationUnitField::Filename,
            EvaluationUnitField::ReadError => PyEvaluationUnitField::ReadError,
            EvaluationUnitField::SkipError => PyEvaluationUnitField::SkipError,
            EvaluationUnitField::JitterGenericError => PyEvaluationUnitField::JitterGenericError,
            EvaluationUnitField::JitterEdgeError => PyEvaluationUnitField::JitterEdgeError,
            EvaluationUnitField::JitterAtomError => PyEvaluationUnitField::JitterAtomError,
            EvaluationUnitField::DriftError => PyEvaluationUnitField::DriftError,
            EvaluationUnitField::DroppedError => PyEvaluationUnitField::DroppedError,
            EvaluationUnitField::DuplicatedError => PyEvaluationUnitField::DuplicatedError,
            EvaluationUnitField::InconsistentErrorSectors => {
                PyEvaluationUnitField::InconsistentErrorSectors
            }
            EvaluationUnitField::DamagedSector => PyEvaluationUnitField::DamagedSector,
            EvaluationUnitField::Abort => PyEvaluationUnitField::Abort,
        }
    }
}

#[pyclass(name = "EvaluationUnitClass")]
#[derive(Clone, PartialEq)]
pub enum PyEvaluationUnitClass {
    Critical,
    Bad,
    Neutral,
    Good,
    Perfect,
}

#[pymethods]
impl PyEvaluationUnitClass {
    #[getter]
    fn name(&self) -> &str {
        match self {
            PyEvaluationUnitClass::Critical => "Critical",
            PyEvaluationUnitClass::Bad => "Bad",
            PyEvaluationUnitClass::Neutral => "Neutral",
            PyEvaluationUnitClass::Good => "Good",
            PyEvaluationUnitClass::Perfect => "Perfect",
        }
    }

    #[getter]
    fn value(&self, py: Python) -> PyResult<Py<PyAny>> {
        // Convert to original Rust enum and serialize with pythonize
        let rust_enum = match self {
            PyEvaluationUnitClass::Critical => EvaluationUnitClass::Critical,
            PyEvaluationUnitClass::Bad => EvaluationUnitClass::Bad,
            PyEvaluationUnitClass::Neutral => EvaluationUnitClass::Neutral,
            PyEvaluationUnitClass::Good => EvaluationUnitClass::Good,
            PyEvaluationUnitClass::Perfect => EvaluationUnitClass::Perfect,
        };
        Ok(pythonize(py, &rust_enum)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?
            .unbind())
    }

    fn __repr__(&self) -> String {
        format!("<EvaluationUnitClass.{}>", self.name())
    }
}

impl From<&EvaluationUnitClass> for PyEvaluationUnitClass {
    fn from(class: &EvaluationUnitClass) -> Self {
        match class {
            EvaluationUnitClass::Critical => PyEvaluationUnitClass::Critical,
            EvaluationUnitClass::Bad => PyEvaluationUnitClass::Bad,
            EvaluationUnitClass::Neutral => PyEvaluationUnitClass::Neutral,
            EvaluationUnitClass::Good => PyEvaluationUnitClass::Good,
            EvaluationUnitClass::Perfect => PyEvaluationUnitClass::Perfect,
        }
    }
}

#[pyclass(name = "EvaluationUnitData")]
#[derive(Clone)]
pub struct PyEvaluationUnitData {
    #[pyo3(get)]
    pub scope: PyEvaluationUnitScope,
    #[pyo3(get)]
    pub field: PyEvaluationUnitField,
    #[pyo3(get)]
    pub message: String,
    #[pyo3(get, name = "classification")]
    pub classification: PyEvaluationUnitClass,
}

impl From<&EvaluationUnitData> for PyEvaluationUnitData {
    fn from(data: &EvaluationUnitData) -> Self {
        PyEvaluationUnitData {
            scope: PyEvaluationUnitScope::from(&data.scope),
            field: PyEvaluationUnitField::from(&data.field),
            message: data.message.clone(),
            classification: PyEvaluationUnitClass::from(&data.class),
        }
    }
}

#[pymethods]
impl PyEvaluationUnitData {
    fn __repr__(&self) -> String {
        format!(
            "<EvaluationUnitData field={} classification={}>",
            self.field.name(),
            self.classification.name()
        )
    }
}

#[pyclass(name = "EvaluationUnit")]
#[derive(Clone)]
pub struct PyEvaluationUnit {
    #[pyo3(get)]
    pub unit_score: String,
    #[pyo3(get)]
    pub data: PyEvaluationUnitData,
}

impl PyEvaluationUnit {
    pub fn from_unit(unit: &EvaluationUnit) -> Self {
        PyEvaluationUnit {
            unit_score: unit.unit_score.clone(),
            data: PyEvaluationUnitData::from(&unit.data),
        }
    }
}

#[pymethods]
impl PyEvaluationUnit {
    fn __repr__(&self) -> String {
        format!("<EvaluationUnit score='{}'>", self.unit_score)
    }
}

#[pyclass(name = "Evaluation")]
#[derive(Clone)]
pub struct PyEvaluation {
    #[pyo3(get)]
    pub score: String,
    #[pyo3(get)]
    pub evaluation_units: Vec<PyEvaluationUnit>,
}

impl PyEvaluation {
    pub fn from_evaluation(eval: &Evaluation) -> Self {
        let units = eval
            .evaluation_units
            .iter()
            .map(|u| PyEvaluationUnit::from_unit(u))
            .collect();

        PyEvaluation {
            score: eval.score.clone(),
            evaluation_units: units,
        }
    }
}

#[pymethods]
impl PyEvaluation {
    fn __repr__(&self) -> String {
        format!(
            "<Evaluation score='{}' units={}>",
            self.score,
            self.evaluation_units.len()
        )
    }
}

#[pyclass(name = "EvaluationCombined")]
#[derive(Clone)]
pub struct PyEvaluationCombined {
    #[pyo3(get)]
    pub evaluator: PyEvaluatorType,
    #[pyo3(get)]
    pub combined_score: String,
    #[pyo3(get)]
    pub evaluations: Vec<PyEvaluation>,
}

impl PyEvaluationCombined {
    pub fn from_combined(combined: &EvaluationCombined) -> Self {
        let evaluations = combined
            .evaluations
            .iter()
            .map(|e| PyEvaluation::from_evaluation(e))
            .collect();

        PyEvaluationCombined {
            evaluator: PyEvaluatorType::from(&combined.evaluator),
            combined_score: combined.combined_score.clone(),
            evaluations,
        }
    }
}

#[pymethods]
impl PyEvaluationCombined {
    fn __repr__(&self) -> String {
        format!(
            "<EvaluationCombined evaluator={} score='{}'>",
            self.evaluator.name(),
            self.combined_score
        )
    }
}

// ============= Main Response Classes =============

#[pyclass(name = "CambiaResponse")]
#[derive(Clone)]
pub struct PyCambiaResponse {
    pub id: Vec<u8>,
    #[pyo3(get)]
    pub parsed: PyParsedLogCombined,
    #[pyo3(get)]
    pub evaluation_combined: Vec<PyEvaluationCombined>,
}

#[pymethods]
impl PyCambiaResponse {
    #[getter]
    fn id<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.id)
    }

    fn __repr__(&self) -> String {
        format!(
            "<CambiaResponse evaluations={}>",
            self.evaluation_combined.len()
        )
    }
}

impl PyCambiaResponse {
    pub fn from_response(response: &CambiaResponse) -> Self {
        let evaluation_combined = response
            .evaluation_combined
            .iter()
            .map(|e| PyEvaluationCombined::from_combined(e))
            .collect();

        PyCambiaResponse {
            id: response.id.clone(),
            parsed: PyParsedLogCombined::from_combined(&response.parsed),
            evaluation_combined,
        }
    }
}

pub fn register_classes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Enums
    m.add_class::<PyRipper>()?;
    m.add_class::<PyMediaType>()?;
    m.add_class::<PyQuartet>()?;
    m.add_class::<PyReadMode>()?;
    m.add_class::<PyGap>()?;
    m.add_class::<PyIntegrity>()?;
    m.add_class::<PyAccurateRipStatus>()?;
    m.add_class::<PyEvaluatorType>()?;
    m.add_class::<PyEvaluationUnitScope>()?;
    m.add_class::<PyEvaluationUnitField>()?;
    m.add_class::<PyEvaluationUnitClass>()?;

    // Data classes
    m.add_class::<PyTocEntry>()?;
    m.add_class::<PyTocHash>()?;
    m.add_class::<PyTocRaw>()?;
    m.add_class::<PyToc>()?;
    m.add_class::<PyChecksum>()?;
    m.add_class::<PyReleaseInfo>()?;
    m.add_class::<PyAccurateRipConfidence>()?;
    m.add_class::<PyAccurateRipUnit>()?;
    m.add_class::<PyTestAndCopy>()?;
    m.add_class::<PyTrackErrorRange>()?;
    m.add_class::<PyTrackErrorData>()?;
    m.add_class::<PyTrackError>()?;
    m.add_class::<PyTrackEntry>()?;
    m.add_class::<PyParsedLog>()?;
    m.add_class::<PyParsedLogCombined>()?;
    m.add_class::<PyEvaluationUnitData>()?;
    m.add_class::<PyEvaluationUnit>()?;
    m.add_class::<PyEvaluation>()?;
    m.add_class::<PyEvaluationCombined>()?;
    m.add_class::<PyCambiaResponse>()?;
    Ok(())
}
