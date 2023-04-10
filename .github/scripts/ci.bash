#!/usr/bin/env bash
# Script for running check on your rust projects.
set -e
set -x
set -u

run_doc() {
    rustup component add rust-docs
    cargo doc
}

run_fmt() {
    rustup component add rustfmt
    cargo fmt --check
}

declare -A FEATURES
FEATURES=(
    "humantime"
    "reltime"
    "regex"
    "humantime reltime"
    "humantime regex"
    "reltime regex"
    "humantime reltime regex"
)


run_clippy() {
    rustup component add clippy-preview
    cargo clippy -- -D warnings
    for FEAT in "${FEATURES[@]}"
    do
        cargo clippy --no-default-features --features "$FEAT" -- -D warnings
    done
}

run_check() {
    cargo check --all-features
    cargo check --no-default-features
    for FEAT in "${FEATURES[@]}"
    do
        cargo check --no-default-features --features "$FEAT"
    done
}

run_test() {
    cargo test --all-features
    cargo test --no-default-features
    for FEAT in "${FEATURES[@]}"
    do
        cargo test --no-default-features --features "$FEAT"
    done
}

run_build() {
    cargo build --all-features
    cargo build --no-default-features
    for FEAT in "${FEATURES[@]}"
    do
        cargo build --no-default-features --features "$FEAT"
    done
}

run_build_release() {
    cargo build --release --all-features
    cargo build --release --no-default-features
    for FEAT in "${FEATURES[@]}"
    do
        cargo build --release --no-default-features --features "$FEAT"
    done
}

case $1 in
    doc)
        run_doc
        ;;
    fmt)
        run_fmt
        ;;
    clippy)
        run_clippy
        ;;
    check)
        run_check
        ;;
    test)
        run_test
        ;;
    build)
        run_build
        ;;
    build-release)
        run_build_release
        ;;
esac
