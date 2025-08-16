# PyCambia Python Package Makefile

.PHONY: help build install test clean dev-install release publish

help:
	@echo "PyCambia Python Package - Available commands:"
	@echo ""
	@echo "  build        - Build wheel files"
	@echo "  build-release - Build release version wheel files"
	@echo "  dev-install  - Install in development mode"
	@echo "  install      - Install from wheel files"
	@echo "  test         - Run tests"
	@echo "  clean        - Clean build files"
	@echo "  publish      - Publish to PyPI"
	@echo ""

build:
	maturin build

build-release:
	maturin build --release

dev-install:
	maturin develop

install: build-release
	pip install target/wheels/pycambia-0.1.0-*.whl --force-reinstall

test:
	pytest

clean:
	cargo clean
	rm -rf target/wheels/
	rm -rf python/cambia/__pycache__/

publish:
	maturin publish

# Windows compatible version
clean-win:
	cargo clean
	if exist target\wheels rmdir /s /q target\wheels
	if exist python\cambia\__pycache__ rmdir /s /q python\cambia\__pycache__