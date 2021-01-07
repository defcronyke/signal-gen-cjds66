@echo off
REM Copyright © 2020-2021 Jeremy Carter - jeremy@jeremycarter.ca
REM
REM By using this software, you agree to the LICENSE TERMS 
REM outlined in the file titled LICENSE.md contained in the 
REM top-level directory of this project. If you don't agree
REM to the LICENSE TERMS, you aren't allowed to use this
REM software.

cargo test --workspace -- %*
