echo off

set pwd=%CD%
cd signal-gen-cjds66-lib

cargo doc %*

cd %pwd%
