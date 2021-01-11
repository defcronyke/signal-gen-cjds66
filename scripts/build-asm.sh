#!/bin/bash
# Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>
#
# By using this software, you agree to the LICENSE TERMS 
# outlined in the file titled LICENSE.md contained in the 
# top-level directory of this project. If you don't agree
# to the LICENSE TERMS, you aren't allowed to use this
# software.

RUSTFLAGS="--emit asm -C llvm-args=-x86-asm-syntax=intel" cargo build $@
