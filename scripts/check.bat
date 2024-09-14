@echo on
REM This script runs various CI-like checks in a convenient way.

setlocal enabledelayedexpansion
set script_path=%~dp0
cd /d "%script_path%\.."
set RUSTDOCFLAGS=-D warnings

REM Installing typos-cli if not already installed
cargo install typos-cli --quiet

REM Running various Rust checks
typos 
cargo fmt --all -- --check
cargo clippy --quiet --all-features -- -D warnings 
cargo check --quiet --all-targets
cargo test --quiet --all-targets
cargo test --quiet --doc

echo All checks passed.
