#!/usr/bin/env bash
# Script for building your rust projects.
set -e

source ci/common.bash

# $1 {path} = Path to cross/cargo executable
CROSS=$1
# $1 {string} = <Target Triple>
TARGET_TRIPLE=$2

required_arg $CROSS 'CROSS'
required_arg $TARGET_TRIPLE '<Target Triple>'

$CROSS check --target $TARGET_TRIPLE --no-default-features --features "regex"
$CROSS check --target $TARGET_TRIPLE --no-default-features --features "atty"
$CROSS check --target $TARGET_TRIPLE --no-default-features --features "humantime"
$CROSS check --target $TARGET_TRIPLE --no-default-features --features "regex atty"
$CROSS check --target $TARGET_TRIPLE --no-default-features --features "regex humantime"
$CROSS check --target $TARGET_TRIPLE --no-default-features --features "atty humantime"
$CROSS test --target $TARGET_TRIPLE
$CROSS test --target $TARGET_TRIPLE --all-features
