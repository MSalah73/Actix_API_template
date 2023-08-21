# Variables
BUILD_TARGET ?= x86_64-unknown-linux-gnu 

RUSTUP ?= rustup
CARGO ?= cargo
FMT ?= cargo-fmt
CLIPPY ?= cargo-clippy
COV ?= cargo-tarpaulin
DENY ?= cargo-deny
AUDIT ?= cargo-audit


# Conditionals to check if packages exists on users machine
check-rustup:
ifeq (, $(shell which $(RUSTUP)))
	$(error " `${RUSTUP}` is not installed. Please follow the installation guide at 'https://doc.rust-lang.org/cargo/getting-started/installation.html' ")
endif

check-cargo:
ifeq (, $(shell which $(CARGO)))
	$(error " `${CARGO}` is not installed. Please follow the installation guide at 'https://doc.rust-lang.org/cargo/getting-started/installation.html' ")
endif

check-fmt:
ifeq (, $(shell which $(FMT)))
	$(error " `${FMT}` is not installed. Please try `rustup component add rustfmt` ")
endif

check-clippy:
ifeq (, $(shell which $(CLIPPY)))
	$(error " `${CLIPPY}` is not installed. Please try `rustup component add clippy` ")
endif

check-audit:
ifeq (, $(shell which $(AUDIT)))
	$(error " `${AUDIT}` is not installed. Please try `cargo install $(AUDIT)` ")
endif

check-deny:
ifeq (, $(shell which $(DENY)))
	$(error " `${DENY}` is not installed. Please try `cargo install --locked $(DENY) or make install` ")
endif

check-coverage:
ifeq (, $(shell which $(COV)))
	$(error " `${COV}` is not installed. Please try `cargo install $(COV) or make install` ")
endif


# Make Commands
.PHONY: all clean \
			check-rustup check-cargo check-fmt check-fmt check- clippy \
			check-audit check-deny check-coverage \
			install format check-format linter test test-unit test-coverage \
			check-sec audit deny

all: build

clean: check-cargo
	$(CARGO) clean

build: 

install: check-cargo
	$(CARGO) install $(DENY)
	$(CARGO) install $(COV) --locked
	
# Formatting
format: check-fmt
	$(CARGO) fmt

check-format: check-fmt
	$(CARGO) fmt -- --check

# Linter
linter: check-clippy
	$(CARGO) clippy -- -D warnings

# Testing
test: check-cargo check-coverage test-unit test-coverage

test-unit:
	$(CARGO) test

test-coverage:
	$(CARGO) tarpaulin --ignore-tests
	
# Security 
check-sec: check-cargo check-deny audit deny

audit:
	$(CARGO) audit

deny:
	$(CARGO) deny check advisories
