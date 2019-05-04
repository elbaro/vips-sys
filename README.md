# vips-sys
[![Crates.io](https://img.shields.io/crates/v/vips-sys.svg)](https://crates.io/crates/vips-sys) [![Build Status](https://travis-ci.org/elbaro/vips-sys.svg?branch=master)](https://travis-ci.org/elbaro/vips-sys) [![Code Coverage](https://codecov.io/gh/elbaro/vips-sys/branch/master/graph/badge.svg)](https://codecov.io/gh/elbaro/vips-sys)

This crate provides bindings to libvips.

[Documentation](https://elbaro.github.io/vips-sys/vips_sys/)

[ABI changes for libvips](https://abi-laboratory.pro/index.php?view=timeline&l=vips)

## Requirements
- libvips >= 8.2 (Ubuntu 16.04 libvips==8.2.2)
- Linux: `pkg-config --cflags --libs vips` runs successfully
- Mac:  `pkg-config --cflags --libs vips` runs successfully
- Windows: not supported yet

## Feature gates
- [TODO] static vs dynamic linking
