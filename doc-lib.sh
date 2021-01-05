#!/bin/bash

pwd="$PWD"
cd signal-gen-cjds66-lib

cargo doc $@

cd "$pwd"
