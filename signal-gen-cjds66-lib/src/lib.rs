/* Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>

By using this software, you agree to the LICENSE TERMS 
outlined in the file titled LICENSE.md contained in the 
top-level directory of this project. If you don't agree
to the LICENSE TERMS, you aren't allowed to use this
software.
*/

// NOTE: Uncomment the line below to enable cargo doc linter.
// #![warn(missing_docs)]

/*! An unofficial support library which can fully remote control 
the inexpensive `"Koolertron DDS Signal Generator"` known as `"GH-CJDS66"` 
(Full name: 
`"Koolertron Upgraded 60MHz DDS Signal Generator Counter, High Precision Dual-channel Arbitrary Waveform Function Generator Frequency Meter 200MSa/s (60MHz) Model: GH-CJDS66-FU"`
). Perhaps it also works with other similar DDS Signal Generators, 
although that hasn't been tested.  
  
This has been written following the official manufacturer's USB-serial 
communication protocol specification. The library implements support for 
every working manufacturer-documented feature from the spec, and only a 
few extra convenience features mentioned at the end of the spec have been 
left out, because they didn't seem to work as advertised.  
  
The few extra missing features probably aren't really needed, but will 
happily be added anyway if someone would like to figure out how/if they 
work, and explain them to the author. You can read about what's still 
missing in 
[the commit message of this git commit](https://gitlab.com/defcronyke/signal-gen-cjds66/-/commit/713b026a4e10807d23f7436d26649dcc4c584019), 
and file an issue or open a merge request about it on GitLab if you figure 
something out or you'd like to discuss it further.  
  
Project link:  
[https://gitlab.com/defcronyke/signal-gen-cjds66](https://gitlab.com/defcronyke/signal-gen-cjds66)  
  
Manufacturer's webpage for the device:  
[https://www.koolertron.com/koolertron-upgraded-60mhz-dds-signal-generator-counterhigh-precision-dualchannel-arbitrary-waveform-function-generator-frequency-meter-200msas-60mhz-p-867.html](https://www.koolertron.com/koolertron-upgraded-60mhz-dds-signal-generator-counterhigh-precision-dualchannel-arbitrary-waveform-function-generator-frequency-meter-200msas-60mhz-p-867.html)
*/

#[macro_use]
extern crate bitflags;
extern crate clap;

pub mod command;
pub mod error;
pub mod protocol;
pub mod serial;
pub mod util;


/* Unit Test Modules */

#[cfg(test)]
mod command_test;

#[cfg(test)]
mod error_test;

#[cfg(test)]
mod serial_test;

#[cfg(test)]
mod util_test;

/* END Unit Test Modules */
