Control the GH-CJDS66 60MHz Signal Generator  
--------------------------------------------  
  
[![docs](https://docs.rs/signal-gen-cjds66-lib/badge.svg)](https://docs.rs/signal-gen-cjds66-lib) [![crate](https://img.shields.io/crates/v/signal_gen_cjds66_lib)](https://crates.io/crates/signal-gen-cjds66-lib) [![pipeline status](https://gitlab.com/defcronyke/signal-gen-cjds66/badges/master/pipeline.svg)](https://gitlab.com/defcronyke/signal-gen-cjds66/-/commits/master) [![coverage report](https://gitlab.com/defcronyke/signal-gen-cjds66/badges/master/coverage.svg)](https://gitlab.com/defcronyke/signal-gen-cjds66/-/commits/master)  
  
https://gitlab.com/defcronyke/signal-gen-cjds66  
  
Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>  
  
By using this software, you agree to the LICENSE TERMS 
outlined in the file titled LICENSE.md contained in the 
top-level directory of this project. If you don't agree
to the LICENSE TERMS, you aren't allowed to use this
software.  
  
Purpose:  
-------  
This is an unofficial project which fully implements the official 
communication spec for the DDS Signal Generator and Counter device
known as the "Koolertron Upgraded 60MHz DDS Signal Generator Counter, 
High Precision Dual-channel Arbitrary Waveform Function Generator 
Frequency Meter 200MSa/s (60MHz) Model: GH-CJDS66-FU" (less a few spec 
errata, which you can read about in a commit message here:  
[713b026a4e10807d23f7436d26649dcc4c584019](https://gitlab.com/defcronyke/signal-gen-cjds66/-/commit/713b026a4e10807d23f7436d26649dcc4c584019))  
  
Device and USB Interface info:  
-----------------------------  
Manufacturer page with info on where to buy it:  
[https://www.koolertron.com/koolertron-upgraded-60mhz-dds-signal-generator-counterhigh-precision-dualchannel-arbitrary-waveform-function-generator-frequency-meter-200msas-60mhz-p-867.html](https://www.koolertron.com/koolertron-upgraded-60mhz-dds-signal-generator-counterhigh-precision-dualchannel-arbitrary-waveform-function-generator-frequency-meter-200msas-60mhz-p-867.html)  
  
Linux `lsusb` output:  
```shell
ID 1a86:7523 QinHeng Electronics CH340 serial converter
```
  
