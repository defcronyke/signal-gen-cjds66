/* Copyright © 2020-2021 Jeremy Carter <jeremy@jeremycarter.ca>

By using this software, you agree to the LICENSE TERMS 
outlined in the file titled LICENSE.md contained in the 
top-level directory of this project. If you don't agree
to the LICENSE TERMS, you aren't allowed to use this
software.
*/

/*! The device's USB-serial communication protocol,
exposed as a bunch of constants for use with other
parts of the library.
*/

use bitflags;
use phf::phf_map;

use std::fmt;
use std::str;

// -----
macro_rules! SERIAL_TIMEOUT_MS {
	() => {
		3000
	};
}
pub const SERIAL_TIMEOUT_MS: u64 = SERIAL_TIMEOUT_MS!();

macro_rules! COMMAND_DELAY_MS {
	() => {
			50
	};
}
pub const COMMAND_DELAY_MS: u64 = COMMAND_DELAY_MS!();
// -----

// -----
macro_rules! COMMAND_BEGIN {
	() => {
			":"
	};
}
pub const COMMAND_BEGIN: &'static str = COMMAND_BEGIN!();

macro_rules! COMMAND_SEPARATOR {
	() => {
			"="
	};
}
pub const COMMAND_SEPARATOR: &'static str = COMMAND_SEPARATOR!();

macro_rules! COMMAND_ARG_SEPARATOR {
	() => {
			","
	};
}
pub const COMMAND_ARG_SEPARATOR: &'static str = COMMAND_ARG_SEPARATOR!();

macro_rules! COMMAND_STOP {
	() => {
			"."
	};
}
pub const COMMAND_STOP: &'static str = COMMAND_STOP!();

macro_rules! COMMAND_LINEBREAK {
	() => {
		"\r\n"
	};
}
pub const COMMAND_LINEBREAK: &'static str = COMMAND_LINEBREAK!();

// ".\r\n"
macro_rules! COMMAND_END {
	() => {
		concat!(COMMAND_STOP!(), COMMAND_LINEBREAK!(),)
	};
}
pub const COMMAND_END: &'static str = COMMAND_END!();
// -----

// -----
// Use this to read values from the device.
macro_rules! COMMAND_GET {
	() => {
			"r"
	};
}
pub const COMMAND_GET: &'static str = COMMAND_GET!();

// Use this to read values from the device.
macro_rules! COMMAND_SET {
	() => {
			"w"
	};
}
pub const COMMAND_SET: &'static str = COMMAND_SET!();
// -----

// -----
// Read the device's model number.
macro_rules! GET_MODEL_COMMAND {
	() => {
		"00"
	};
}
pub const GET_MODEL_COMMAND: &'static str = GET_MODEL_COMMAND!();

macro_rules! GET_MODEL_ARG1 {
	() => {
			"0"
	};
}
pub const GET_MODEL_ARG1: &'static str = GET_MODEL_ARG1!();

// command example:
// ":r00=0.\r\n"
macro_rules! GET_MODEL {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_GET!(),
			GET_MODEL_COMMAND!(),
			COMMAND_SEPARATOR!(),
			GET_MODEL_ARG1!(),
			COMMAND_END!(),
			)
	};
}
pub const GET_MODEL: &'static str = GET_MODEL!();

macro_rules! GET_MODEL_RES_LEN {
	() => {
			10
	};
}
pub const GET_MODEL_RES_LEN: u8 = GET_MODEL_RES_LEN!();
// -----

// -----
// Read the device's serial number.
macro_rules! GET_SERIAL_COMMAND {
	() => {
		"01"
	};
}
pub const GET_SERIAL_COMMAND: &'static str = GET_SERIAL_COMMAND!();

macro_rules! GET_SERIAL_ARG1 {
	() => {
			"0"
	};
}
pub const GET_SERIAL_ARG1: &'static str = GET_SERIAL_ARG1!();

macro_rules! GET_SERIAL_RES_LEN {
	() => {
			18
	};
}
pub const GET_SERIAL_RES_LEN: u8 = GET_SERIAL_RES_LEN!();

// command example:
// ":r01=0.\r\n"
macro_rules! GET_SERIAL {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_GET!(),
			GET_SERIAL_COMMAND!(),
			COMMAND_SEPARATOR!(),
			GET_SERIAL_ARG1!(),
			COMMAND_END!(),
			)
	};
}
pub const GET_SERIAL: &'static str = GET_SERIAL!();
// -----

// -----
// Read the device's model number and serial number.
macro_rules! GET_MODEL_AND_NUMBER_ARG1 {
	() => {
			"1"
	};
}
pub const GET_MODEL_AND_NUMBER_ARG1: &'static str = GET_MODEL_AND_NUMBER_ARG1!();

// command example:
// ":r00=1.\r\n"
macro_rules! GET_MODEL_AND_NUMBER {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_GET!(),
			GET_MODEL_COMMAND!(),
			COMMAND_SEPARATOR!(),
			GET_MODEL_AND_NUMBER_ARG1!(),
			COMMAND_END!(),
			)
	};
}
pub const GET_MODEL_AND_NUMBER: &'static str = GET_MODEL_AND_NUMBER!();

macro_rules! GET_MODEL_AND_NUMBER_RES_LEN {
	() => {
			28
	};
}
pub const GET_MODEL_AND_NUMBER_RES_LEN: u8 = GET_MODEL_AND_NUMBER_RES_LEN!();
// -----

// -----
// Turn output channels on or off.
macro_rules! SET_CHANNEL_OUTPUT_COMMAND {
	() => {
		"20"
	};
}
pub const SET_CHANNEL_OUTPUT_COMMAND: &'static str = SET_CHANNEL_OUTPUT_COMMAND!();

macro_rules! SET_CHANNEL_OUTPUT_ARG_CH_ON {
	() => {
			"1"
	};
}
pub const SET_CHANNEL_OUTPUT_ARG_CH_ON: &'static str = SET_CHANNEL_OUTPUT_ARG_CH_ON!();

macro_rules! SET_CHANNEL_OUTPUT_ARG_CH_OFF {
	() => {
			"0"
	};
}
pub const SET_CHANNEL_OUTPUT_ARG_CH_OFF: &'static str = SET_CHANNEL_OUTPUT_ARG_CH_OFF!();

macro_rules! SET_CHANNEL_OUTPUT_RES_LEN {
	() => {
			6
	};
}
pub const SET_CHANNEL_OUTPUT_RES_LEN: u8 = SET_CHANNEL_OUTPUT_RES_LEN!();

// command example - both on:
// ":w20=1,1.\r\n"
macro_rules! SET_CHANNEL_OUTPUT_BOTH_ON {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_CHANNEL_OUTPUT_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_CHANNEL_OUTPUT_ARG_CH_ON!(),
			COMMAND_ARG_SEPARATOR!(),
			SET_CHANNEL_OUTPUT_ARG_CH_ON!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_CHANNEL_OUTPUT_BOTH_ON: &str = SET_CHANNEL_OUTPUT_BOTH_ON!();

// command example - both off:
// ":w20=0,0.\r\n"
macro_rules! SET_CHANNEL_OUTPUT_BOTH_OFF {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_CHANNEL_OUTPUT_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_CHANNEL_OUTPUT_ARG_CH_OFF!(),
			COMMAND_ARG_SEPARATOR!(),
			SET_CHANNEL_OUTPUT_ARG_CH_OFF!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_CHANNEL_OUTPUT_BOTH_OFF: &str = SET_CHANNEL_OUTPUT_BOTH_OFF!();

// command example - ch1 on, ch2 off:
// ":w20=1,0.\r\n"
macro_rules! SET_CHANNEL_OUTPUT_CH1_ON_CH2_OFF {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_CHANNEL_OUTPUT_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_CHANNEL_OUTPUT_ARG_CH_ON!(),
			COMMAND_ARG_SEPARATOR!(),
			SET_CHANNEL_OUTPUT_ARG_CH_OFF!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_CHANNEL_OUTPUT_CH1_ON_CH2_OFF: &str = SET_CHANNEL_OUTPUT_CH1_ON_CH2_OFF!();

// command example - ch1 off, ch2 on:
// ":w20=0,1.\r\n"
macro_rules! SET_CHANNEL_OUTPUT_CH1_OFF_CH2_ON {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_CHANNEL_OUTPUT_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_CHANNEL_OUTPUT_ARG_CH_OFF!(),
			COMMAND_ARG_SEPARATOR!(),
			SET_CHANNEL_OUTPUT_ARG_CH_ON!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_CHANNEL_OUTPUT_CH1_OFF_CH2_ON: &str = SET_CHANNEL_OUTPUT_CH1_OFF_CH2_ON!();
// -----

// -----
// Read output channels on or off state.
macro_rules! GET_CHANNEL_OUTPUT_COMMAND {
	() => {
		"20"
	};
}
pub const GET_CHANNEL_OUTPUT_COMMAND: &'static str = GET_CHANNEL_OUTPUT_COMMAND!();

macro_rules! GET_CHANNEL_OUTPUT_ARG {
	() => {
			"0"
	};
}
pub const GET_CHANNEL_OUTPUT_ARG: &'static str = GET_CHANNEL_OUTPUT_ARG!();

macro_rules! GET_CHANNEL_OUTPUT_RES_LEN {
	() => {
			11
	};
}
pub const GET_CHANNEL_OUTPUT_RES_LEN: u8 = GET_CHANNEL_OUTPUT_RES_LEN!();

// command example:
// ":r20=0.\r\n"
macro_rules! GET_CHANNEL_OUTPUT {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_GET!(),
			GET_CHANNEL_OUTPUT_COMMAND!(),
			COMMAND_SEPARATOR!(),
			GET_CHANNEL_OUTPUT_ARG!(),
			COMMAND_END!(),
			)
	};
}
pub const GET_CHANNEL_OUTPUT: &str = GET_CHANNEL_OUTPUT!();
// -----

// -----
// Set waveform preset for each channel.
// Ex:
//   ch1 preset0 (sine wave) = ":w21=00.\r\n"
//   ch2 preset1 (square wave) = ":w22=01.\r\n"
//   ch1 preset101 (arbitrary wave preset1) = ":w21=101.\r\n"
//   ch2 preset102 (arbitrary wave preset2) = ":w22=102.\r\n"
macro_rules! SET_WAVEFORM_PRESET_COMMAND_PREFIX {
	() => {
			"2"
	};
}
pub const SET_WAVEFORM_PRESET_COMMAND_PREFIX: &'static str =
	SET_WAVEFORM_PRESET_COMMAND_PREFIX!();

macro_rules! SET_WAVEFORM_PRESET_COMMAND_CH1 {
	() => {
		concat!(SET_WAVEFORM_PRESET_COMMAND_PREFIX!(), "1",)
	};
}
pub const SET_WAVEFORM_PRESET_COMMAND_CH1: &'static str = SET_WAVEFORM_PRESET_COMMAND_CH1!();

macro_rules! SET_WAVEFORM_PRESET_COMMAND_CH2 {
	() => {
		concat!(SET_WAVEFORM_PRESET_COMMAND_PREFIX!(), "2",)
	};
}
pub const SET_WAVEFORM_PRESET_COMMAND_CH2: &'static str = SET_WAVEFORM_PRESET_COMMAND_CH2!();

macro_rules! SET_WAVEFORM_PRESET_RES_LEN {
	() => {
			6
	};
}
pub const SET_WAVEFORM_PRESET_RES_LEN: u8 = SET_WAVEFORM_PRESET_RES_LEN!();

// -----
// Get waveform preset for each channel.
// Ex:
//   ch1 preset = ":r21=0.\r\n"
//   ch2 preset = ":r22=0.\r\n"
macro_rules! GET_WAVEFORM_PRESET_COMMAND_PREFIX {
	() => {
			"2"
	};
}
pub const GET_WAVEFORM_PRESET_COMMAND_PREFIX: &'static str =
	GET_WAVEFORM_PRESET_COMMAND_PREFIX!();

macro_rules! GET_WAVEFORM_PRESET_COMMAND_CH1 {
	() => {
		concat!(GET_WAVEFORM_PRESET_COMMAND_PREFIX!(), "1",)
	};
}
pub const GET_WAVEFORM_PRESET_COMMAND_CH1: &'static str = GET_WAVEFORM_PRESET_COMMAND_CH1!();

macro_rules! GET_WAVEFORM_PRESET_COMMAND_CH2 {
	() => {
		concat!(GET_WAVEFORM_PRESET_COMMAND_PREFIX!(), "2",)
	};
}
pub const GET_WAVEFORM_PRESET_COMMAND_CH2: &'static str = GET_WAVEFORM_PRESET_COMMAND_CH2!();

macro_rules! GET_WAVEFORM_PRESET_ARG {
	() => {
			0
	};
}
pub const GET_WAVEFORM_PRESET_ARG: u8 = GET_WAVEFORM_PRESET_ARG!();

macro_rules! GET_WAVEFORM_PRESET_RES_LEN {
	() => {
			11
	};
}
pub const GET_WAVEFORM_PRESET_RES_LEN: u8 = GET_WAVEFORM_PRESET_RES_LEN!();
// -----

// -----
// Set waveform frequency for each channel.
// Ex:
//   ch1 = ":w23=1,0.\r\n"
//   ch2 = ":w24=1,0.\r\n"
macro_rules! SET_FREQUENCY_COMMAND_PREFIX {
	() => {
			"2"
	};
}
pub const SET_FREQUENCY_COMMAND_PREFIX: &'static str = SET_FREQUENCY_COMMAND_PREFIX!();

macro_rules! SET_FREQUENCY_COMMAND_CH1 {
	() => {
		concat!(SET_FREQUENCY_COMMAND_PREFIX!(), "3",)
	};
}
pub const SET_FREQUENCY_COMMAND_CH1: &'static str = SET_FREQUENCY_COMMAND_CH1!();

macro_rules! SET_FREQUENCY_COMMAND_CH2 {
	() => {
		concat!(SET_FREQUENCY_COMMAND_PREFIX!(), "4",)
	};
}
pub const SET_FREQUENCY_COMMAND_CH2: &'static str = SET_FREQUENCY_COMMAND_CH2!();

macro_rules! SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ {
	() => {
			"4"
	};
}
pub const SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ: &'static str =
	SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ!();

macro_rules! SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MIN {
	() => {
			0.0
	};
}
pub const SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MIN: f64 =
	SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MIN!();

macro_rules! SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MAX {
	() => {
			80000000.0
	};
}
pub const SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MAX: f64 =
	SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MAX!();

macro_rules! SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MULTIPLIER {
	() => {
			100.0
	};
}
pub const SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MULTIPLIER: f64 =
	SET_FREQUENCY_COMMAND_UNIT_MICROHERTZ_ARG_MULTIPLIER!();

macro_rules! SET_FREQUENCY_COMMAND_UNIT_MILLIHERTZ {
	() => {
			"3"
	};
}
pub const SET_FREQUENCY_COMMAND_UNIT_MILLIHERTZ: &'static str =
	SET_FREQUENCY_COMMAND_UNIT_MILLIHERTZ!();

macro_rules! SET_FREQUENCY_COMMAND_UNIT_HERTZ {
	() => {
			"0"
	};
}
pub const SET_FREQUENCY_COMMAND_UNIT_HERTZ: &'static str = SET_FREQUENCY_COMMAND_UNIT_HERTZ!();

macro_rules! SET_FREQUENCY_COMMAND_UNIT_KILOHERTZ {
	() => {
			"1"
	};
}
pub const SET_FREQUENCY_COMMAND_UNIT_KILOHERTZ: &'static str =
	SET_FREQUENCY_COMMAND_UNIT_KILOHERTZ!();

macro_rules! SET_FREQUENCY_COMMAND_UNIT_MEGAHERTZ {
	() => {
			"2"
	};
}
pub const SET_FREQUENCY_COMMAND_UNIT_MEGAHERTZ: &'static str =
	SET_FREQUENCY_COMMAND_UNIT_MEGAHERTZ!();

macro_rules! SET_FREQUENCY_RES_LEN {
	() => {
			6
	};
}
pub const SET_FREQUENCY_RES_LEN: u8 = SET_FREQUENCY_RES_LEN!();
// -----

// -----
// Get waveform frequency for each channel.
// Ex:
//   ch1 = ":r23=0.\r\n"
//   ch2 = ":r24=0.\r\n"
macro_rules! GET_FREQUENCY_COMMAND_PREFIX {
	() => {
			"2"
	};
}
pub const GET_FREQUENCY_COMMAND_PREFIX: &'static str = GET_FREQUENCY_COMMAND_PREFIX!();

macro_rules! GET_FREQUENCY_COMMAND_CH1 {
	() => {
		concat!(GET_FREQUENCY_COMMAND_PREFIX!(), "3",)
	};
}
pub const GET_FREQUENCY_COMMAND_CH1: &'static str = GET_FREQUENCY_COMMAND_CH1!();

macro_rules! GET_FREQUENCY_COMMAND_CH2 {
	() => {
		concat!(GET_FREQUENCY_COMMAND_PREFIX!(), "4",)
	};
}
pub const GET_FREQUENCY_COMMAND_CH2: &'static str = GET_FREQUENCY_COMMAND_CH2!();

macro_rules! GET_FREQUENCY_ARG {
	() => {
			"0"
	};
}
pub const GET_FREQUENCY_ARG: &'static str = GET_FREQUENCY_ARG!();

macro_rules! GET_FREQUENCY_RES_LEN {
	() => {
			21
	};
}
pub const GET_FREQUENCY_RES_LEN: u8 = GET_FREQUENCY_RES_LEN!();
// -----

// -----
// Set the signal amplitude.
// Ex:
//   ch1 (0.01v) = ":w25=1.\r\n"
//   ch2 (0.01v) = ":w26=1.\r\n"
macro_rules! SET_AMPLITUDE_COMMAND_PREFIX {
	() => {
			"2"
	};
}
pub const SET_AMPLITUDE_COMMAND_PREFIX: &'static str = SET_AMPLITUDE_COMMAND_PREFIX!();

macro_rules! SET_AMPLITUDE_COMMAND_CH1 {
	() => {
		concat!(SET_AMPLITUDE_COMMAND_PREFIX!(), "5",)
	};
}
pub const SET_AMPLITUDE_COMMAND_CH1: &'static str = SET_AMPLITUDE_COMMAND_CH1!();

macro_rules! SET_AMPLITUDE_COMMAND_CH2 {
	() => {
		concat!(SET_AMPLITUDE_COMMAND_PREFIX!(), "6",)
	};
}
pub const SET_AMPLITUDE_COMMAND_CH2: &'static str = SET_AMPLITUDE_COMMAND_CH2!();

macro_rules! SET_AMPLITUDE_RES_LEN {
	() => {
			6
	};
}
pub const SET_AMPLITUDE_RES_LEN: u8 = SET_AMPLITUDE_RES_LEN!();
// -----

// -----
// Get the signal amplitude.
// Ex:
//   ch1 = ":r25=0.\r\n"
//   ch2 = ":r26=0.\r\n"
macro_rules! GET_AMPLITUDE_COMMAND_PREFIX {
	() => {
			"2"
	};
}
pub const GET_AMPLITUDE_COMMAND_PREFIX: &'static str = GET_AMPLITUDE_COMMAND_PREFIX!();

macro_rules! GET_AMPLITUDE_COMMAND_CH1 {
	() => {
		concat!(GET_AMPLITUDE_COMMAND_PREFIX!(), "5",)
	};
}
pub const GET_AMPLITUDE_COMMAND_CH1: &'static str = GET_AMPLITUDE_COMMAND_CH1!();

macro_rules! GET_AMPLITUDE_COMMAND_CH2 {
	() => {
		concat!(GET_AMPLITUDE_COMMAND_PREFIX!(), "6",)
	};
}
pub const GET_AMPLITUDE_COMMAND_CH2: &'static str = GET_AMPLITUDE_COMMAND_CH2!();

macro_rules! GET_AMPLITUDE_ARG {
	() => {
			0
	};
}
pub const GET_AMPLITUDE_ARG: u8 = GET_AMPLITUDE_ARG!();

macro_rules! GET_AMPLITUDE_RES_LEN {
	() => {
			13
	};
}
pub const GET_AMPLITUDE_RES_LEN: u8 = GET_AMPLITUDE_RES_LEN!();
// -----

// -----
// Set the duty cycle.
// Ex:
//   ch1 (40.1%) = ":w29=401.\r\n"
//   ch2 (40.1%) = ":w30=401.\r\n"
macro_rules! SET_DUTY_CYCLE_COMMAND_PREFIX_CH1 {
	() => {
			"2"
	};
}
pub const SET_DUTY_CYCLE_COMMAND_PREFIX_CH1: &'static str =
	SET_DUTY_CYCLE_COMMAND_PREFIX_CH1!();

macro_rules! SET_DUTY_CYCLE_COMMAND_PREFIX_CH2 {
	() => {
			"3"
	};
}
pub const SET_DUTY_CYCLE_COMMAND_PREFIX_CH2: &'static str =
	SET_DUTY_CYCLE_COMMAND_PREFIX_CH2!();

macro_rules! SET_DUTY_CYCLE_COMMAND_CH1 {
	() => {
		concat!(SET_DUTY_CYCLE_COMMAND_PREFIX_CH1!(), "9",)
	};
}
pub const SET_DUTY_CYCLE_COMMAND_CH1: &'static str = SET_DUTY_CYCLE_COMMAND_CH1!();

macro_rules! SET_DUTY_CYCLE_COMMAND_CH2 {
	() => {
		concat!(SET_DUTY_CYCLE_COMMAND_PREFIX_CH2!(), "0",)
	};
}
pub const SET_DUTY_CYCLE_COMMAND_CH2: &'static str = SET_DUTY_CYCLE_COMMAND_CH2!();

macro_rules! SET_DUTY_CYCLE_RES_LEN {
	() => {
			6
	};
}
pub const SET_DUTY_CYCLE_RES_LEN: u8 = SET_DUTY_CYCLE_RES_LEN!();
// -----

// -----
// Get the duty cycle.
// Ex:
//   ch1 = ":r29=0.\r\n"
//   ch2 = ":r30=0.\r\n"
macro_rules! GET_DUTY_CYCLE_COMMAND_PREFIX_CH1 {
	() => {
			"2"
	};
}
pub const GET_DUTY_CYCLE_COMMAND_PREFIX_CH1: &'static str = GET_DUTY_CYCLE_COMMAND_PREFIX_CH1!();

macro_rules! GET_DUTY_CYCLE_COMMAND_PREFIX_CH2 {
	() => {
			"3"
	};
}
pub const GET_DUTY_CYCLE_COMMAND_PREFIX_CH2: &'static str = GET_DUTY_CYCLE_COMMAND_PREFIX_CH2!();

macro_rules! GET_DUTY_CYCLE_COMMAND_CH1 {
	() => {
		concat!(GET_DUTY_CYCLE_COMMAND_PREFIX_CH1!(), "9",)
	};
}
pub const GET_DUTY_CYCLE_COMMAND_CH1: &'static str = GET_DUTY_CYCLE_COMMAND_CH1!();

macro_rules! GET_DUTY_CYCLE_COMMAND_CH2 {
	() => {
		concat!(GET_DUTY_CYCLE_COMMAND_PREFIX_CH2!(), "0",)
	};
}
pub const GET_DUTY_CYCLE_COMMAND_CH2: &'static str = GET_DUTY_CYCLE_COMMAND_CH2!();

macro_rules! GET_DUTY_CYCLE_ARG {
	() => {
			0
	};
}
pub const GET_DUTY_CYCLE_ARG: u8 = GET_DUTY_CYCLE_ARG!();

macro_rules! GET_DUTY_CYCLE_RES_LEN {
	() => {
			11
	};
}
pub const GET_DUTY_CYCLE_RES_LEN: u8 = GET_DUTY_CYCLE_RES_LEN!();
// -----

// -----
// Set the voltage offset in volts.
// Ex:
//   ch1 (-1.23%) = ":w27=877.\r\n"
//   ch2 (-1.23%) = ":w28=877.\r\n"
macro_rules! SET_VOLTAGE_OFFSET_COMMAND_PREFIX {
	() => {
			"2"
	};
}
pub const SET_VOLTAGE_OFFSET_COMMAND_PREFIX: &'static str =
	SET_VOLTAGE_OFFSET_COMMAND_PREFIX!();

macro_rules! SET_VOLTAGE_OFFSET_COMMAND_CH1 {
	() => {
		concat!(SET_VOLTAGE_OFFSET_COMMAND_PREFIX!(), "7",)
	};
}
pub const SET_VOLTAGE_OFFSET_COMMAND_CH1: &'static str = SET_VOLTAGE_OFFSET_COMMAND_CH1!();

macro_rules! SET_VOLTAGE_OFFSET_COMMAND_CH2 {
	() => {
		concat!(SET_VOLTAGE_OFFSET_COMMAND_PREFIX!(), "8",)
	};
}
pub const SET_VOLTAGE_OFFSET_COMMAND_CH2: &'static str = SET_VOLTAGE_OFFSET_COMMAND_CH2!();

macro_rules! SET_VOLTAGE_OFFSET_RES_LEN {
	() => {
			6
	};
}
pub const SET_VOLTAGE_OFFSET_RES_LEN: u8 = SET_VOLTAGE_OFFSET_RES_LEN!();
// -----

// -----
// Get the voltage offset in volts.
// Ex:
//   ch1 = ":r27=0.\r\n"
//   ch2 = ":r28=0.\r\n"
macro_rules! GET_VOLTAGE_OFFSET_COMMAND_PREFIX {
	() => {
			"2"
	};
}
pub const GET_VOLTAGE_OFFSET_COMMAND_PREFIX: &'static str = GET_VOLTAGE_OFFSET_COMMAND_PREFIX!();

macro_rules! GET_VOLTAGE_OFFSET_COMMAND_CH1 {
	() => {
		concat!(GET_VOLTAGE_OFFSET_COMMAND_PREFIX!(), "7",)
	};
}
pub const GET_VOLTAGE_OFFSET_COMMAND_CH1: &'static str = GET_VOLTAGE_OFFSET_COMMAND_CH1!();

macro_rules! GET_VOLTAGE_OFFSET_COMMAND_CH2 {
	() => {
		concat!(GET_VOLTAGE_OFFSET_COMMAND_PREFIX!(), "8",)
	};
}
pub const GET_VOLTAGE_OFFSET_COMMAND_CH2: &'static str = GET_VOLTAGE_OFFSET_COMMAND_CH2!();

macro_rules! GET_VOLTAGE_OFFSET_ARG {
	() => {
			0
	};
}
pub const GET_VOLTAGE_OFFSET_ARG: u8 = GET_VOLTAGE_OFFSET_ARG!();

macro_rules! GET_VOLTAGE_OFFSET_RES_LEN {
	() => {
			11
	};
}
pub const GET_VOLTAGE_OFFSET_RES_LEN: u8 = GET_VOLTAGE_OFFSET_RES_LEN!();
// -----

// -----
// Set the phase in degrees.
// Ex:
//   180.7% = ":w31=1807.\r\n"
macro_rules! SET_PHASE_COMMAND {
	() => {
		"31"
	};
}
pub const SET_PHASE_COMMAND: &'static str = SET_PHASE_COMMAND!();

macro_rules! SET_PHASE_RES_LEN {
	() => {
			6
	};
}
pub const SET_PHASE_RES_LEN: u8 = SET_PHASE_RES_LEN!();
// -----

// -----
// Get the phase in degrees.
// Ex:
//   ":r31=0.\r\n"
macro_rules! GET_PHASE_COMMAND {
	() => {
		"31"
	};
}
pub const GET_PHASE_COMMAND: &'static str = GET_PHASE_COMMAND!();

macro_rules! GET_PHASE_ARG {
	() => {
			0
	};
}
pub const GET_PHASE_ARG: u8 = GET_PHASE_ARG!();

macro_rules! GET_PHASE_RES_LEN {
	() => {
			12
	};
}
pub const GET_PHASE_RES_LEN: u8 = GET_PHASE_RES_LEN!();
// -----

// -----
// Set the tracking mode.
// Ex:
//   frequency and amplitude sync = ":w54=1,0,1,0,0.\r\n"
//
// Argument index position meanings:
//   0: frequency
//   1: waveform
//   2: amplitude
//   3: dutycycle
//   4: offset
macro_rules! SET_TRACKING_COMMAND {
	() => {
		"54"
	};
}
pub const SET_TRACKING_COMMAND: &'static str = SET_TRACKING_COMMAND!();

macro_rules! TRACKING_FEATURES {
	() => {
		"The bit position meanings are as follows:
0: frequency | freq | fq | fr | f
1: waveform | wave | wav | wv | w
2: amplitude | ampli | amp | am | a
3: dutycycle | duty | dc | du | d
4: offset | off | os | ot | o

turn off tracking: 0 | none | null | non | nil | no | n

You can also use any of the names above separated by commas to turn on
the tracking by feature name.
Ex:
frequency and amplitude sync: -T freq,amp

Or you can use the single character versions with no commas in between.
Ex:
frequency and amplitude sync: -T fa

Turn tracking off like this: -T n"
	};
}
pub const TRACKING_FEATURES: &'static str = TRACKING_FEATURES!();

macro_rules! TRACKING_NONE {
	() => {
		0b00000000u8
	};
}
macro_rules! TRACKING_FREQUENCY {
	() => {
		0b00000001u8
	};
}
macro_rules! TRACKING_WAVEFORM {
	() => {
		0b00000010u8
	};
}
macro_rules! TRACKING_AMPLITUDE {
	() => {
		0b00000100u8
	};
}
macro_rules! TRACKING_DUTYCYCLE {
	() => {
		0b00001000u8
	};
}
macro_rules! TRACKING_OFFSET {
	() => {
		0b00010000u8
	};
}

pub const TRACKING_NONE: u8 = TRACKING_NONE!();
pub const TRACKING_FREQUENCY: u8 = TRACKING_FREQUENCY!();
pub const TRACKING_WAVEFORM: u8 = TRACKING_WAVEFORM!();
pub const TRACKING_AMPLITUDE: u8 = TRACKING_AMPLITUDE!();
pub const TRACKING_DUTYCYCLE: u8 = TRACKING_DUTYCYCLE!();
pub const TRACKING_OFFSET: u8 = TRACKING_OFFSET!();

bitflags! {
	pub struct TrackingArg: u8 {
		const NONE      = TRACKING_NONE;
		const FREQUENCY = TRACKING_FREQUENCY;
		const WAVEFORM  = TRACKING_WAVEFORM;
		const AMPLITUDE = TRACKING_AMPLITUDE;
		const DUTYCYCLE = TRACKING_DUTYCYCLE;
		const OFFSET    = TRACKING_OFFSET;
	}
}

pub static TRACKING_ARG_MAP: phf::Map<&'static str, u8> = phf_map! {
	"none"      => TRACKING_NONE,
	"null"      => TRACKING_NONE,
	"non"       => TRACKING_NONE,
	"nil"       => TRACKING_NONE,
	"no"        => TRACKING_NONE,
	"n"         => TRACKING_NONE,

	"frequency" => TRACKING_FREQUENCY,
	"freq"      => TRACKING_FREQUENCY,
	"fq"        => TRACKING_FREQUENCY,
	"fr"        => TRACKING_FREQUENCY,
	"f"         => TRACKING_FREQUENCY,

	"waveform"  => TRACKING_WAVEFORM,
	"wave"      => TRACKING_WAVEFORM,
	"wav"       => TRACKING_WAVEFORM,
	"wv"        => TRACKING_WAVEFORM,
	"w"         => TRACKING_WAVEFORM,

	"amplitude" => TRACKING_AMPLITUDE,
	"ampli"     => TRACKING_AMPLITUDE,
	"amp"       => TRACKING_AMPLITUDE,
	"am"        => TRACKING_AMPLITUDE,
	"a"         => TRACKING_AMPLITUDE,

	"dutycycle" => TRACKING_DUTYCYCLE,
	"duty"      => TRACKING_DUTYCYCLE,
	"dc"        => TRACKING_DUTYCYCLE,
	"du"        => TRACKING_DUTYCYCLE,
	"d"         => TRACKING_DUTYCYCLE,

	"offset"    => TRACKING_OFFSET,
	"off"       => TRACKING_OFFSET,
	"os"        => TRACKING_OFFSET,
	"ot"        => TRACKING_OFFSET,
	"o"         => TRACKING_OFFSET,
};

pub static TRACKING_ARG_REVMAP: phf::Map<u8, &'static str> = phf_map! {
	0b00000000u8 => "none",
	0b00000001u8 => "frequency",
	0b00000010u8 => "waveform",
	0b00000100u8 => "amplitude",
	0b00001000u8 => "dutycycle",
	0b00010000u8 => "offset",
};

impl fmt::Display for TrackingArg {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{},{},{},{},{}",
			self.bits & 1,
			(self.bits & (1 << 1)) >> 1,
			(self.bits & (1 << 2)) >> 2,
			(self.bits & (1 << 3)) >> 3,
			(self.bits & (1 << 4)) >> 4,
		)
	}
}

pub trait ToStrVal {
	fn to_str_val(&self) -> String;
	fn to_names(&self) -> String;
}

impl ToStrVal for TrackingArg {
	fn to_str_val(&self) -> String {
		format!(
			"{}{}{}{}{}",
			self.bits & 1,
			(self.bits & (1 << 1)) >> 1,
			(self.bits & (1 << 2)) >> 2,
			(self.bits & (1 << 3)) >> 3,
			(self.bits & (1 << 4)) >> 4,
		)
	}

	fn to_names(&self) -> String {
		let one = if (self.bits & 1) > 0 {
			TRACKING_ARG_REVMAP[&1]
		} else {
			TRACKING_ARG_REVMAP[&0]
		};

		let two = if (self.bits & (1 << 1)) > 0 {
			TRACKING_ARG_REVMAP[&(1 << 1)]
		} else {
			TRACKING_ARG_REVMAP[&0]
		};

		let three = if (self.bits & (1 << 2)) > 0 {
			TRACKING_ARG_REVMAP[&(1 << 2)]
		} else {
			TRACKING_ARG_REVMAP[&0]
		};

		let four = if (self.bits & (1 << 3)) > 0 {
			TRACKING_ARG_REVMAP[&(1 << 3)]
		} else {
			TRACKING_ARG_REVMAP[&0]
		};

		let five = if (self.bits & (1 << 4)) > 0 {
			TRACKING_ARG_REVMAP[&(1 << 4)]
		} else {
			TRACKING_ARG_REVMAP[&0]
		};

		let res = format!("{},{},{},{},{}", one, two, three, four, five,)
			.split(",")
			.filter(|&i| i != "none")
			.collect::<Vec<&str>>()
			.join(", ");

		if res == "" {
			return TRACKING_ARG_REVMAP[&0].to_string();
		}

		res
	}
}

macro_rules! SET_TRACKING_RES_LEN {
	() => {
			6
	};
}
pub const SET_TRACKING_RES_LEN: u8 = SET_TRACKING_RES_LEN!();
// -----

// -----
// Set the extended function.
macro_rules! SET_EXTENDED_FUNCTION_COMMAND {
	() => {
		"32"
	};
}
pub const SET_EXTENDED_FUNCTION_COMMAND: &'static str = SET_EXTENDED_FUNCTION_COMMAND!();

// Measurement starting - counting, sweep, frequency, pulse, burst stopping.
macro_rules! START_MEASURING_ARG {
	() => {
		"0,0,0,0"
	};
}
pub const START_MEASURING_ARG: &'static str =
	START_MEASURING_ARG!();

macro_rules! START_COUNTING_ARG {
	() => {
		"1,0,0,0"
	};
}
pub const START_COUNTING_ARG: &'static str =
	START_COUNTING_ARG!();

macro_rules! START_SWEEPING_ARG {
	() => {
		"0,1,0,0"
	};
}
pub const START_SWEEPING_ARG: &'static str =
	START_SWEEPING_ARG!();

macro_rules! START_PULSING_ARG {
	() => {
		"1,0,1,1"
	};
}
pub const START_PULSING_ARG: &'static str =
	START_PULSING_ARG!();

macro_rules! START_BURSTING_ARG {
	() => {
		"1,0,0,1"
	};
}
pub const START_BURSTING_ARG: &'static str =
	START_BURSTING_ARG!();

// command example - measurement starting:
// ":w32=0,0,0,0.\r\n"
macro_rules! START_MEASURING {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_EXTENDED_FUNCTION_COMMAND!(),
			COMMAND_SEPARATOR!(),
			START_MEASURING_ARG!(),
			COMMAND_END!(),
			)
	};
}
pub const START_MEASURING: &str =
	START_MEASURING!();

// command example - counting starting:
// ":w32=1,0,0,0.\r\n"
macro_rules! START_COUNTING {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_EXTENDED_FUNCTION_COMMAND!(),
			COMMAND_SEPARATOR!(),
			START_COUNTING_ARG!(),
			COMMAND_END!(),
			)
	};
}
pub const START_COUNTING: &str =
	START_COUNTING!();

// command example - sweep starting:
// ":w32=0,1,0,0.\r\n"
macro_rules! START_SWEEPING {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_EXTENDED_FUNCTION_COMMAND!(),
			COMMAND_SEPARATOR!(),
			START_SWEEPING_ARG!(),
			COMMAND_END!(),
			)
	};
}
pub const START_SWEEPING: &str = START_SWEEPING!();

// command example - pulse starting:
// ":w32=1,0,1,1.\r\n"
macro_rules! START_PULSING {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_EXTENDED_FUNCTION_COMMAND!(),
			COMMAND_SEPARATOR!(),
			START_PULSING_ARG!(),
			COMMAND_END!(),
			)
	};
}
pub const START_PULSING: &str = START_PULSING!();

// command example - bursting starting:
// ":w32=1,0,0,1.\r\n"
macro_rules! START_BURSTING {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_EXTENDED_FUNCTION_COMMAND!(),
			COMMAND_SEPARATOR!(),
			START_BURSTING_ARG!(),
			COMMAND_END!(),
			)
	};
}
pub const START_BURSTING: &str =
	START_BURSTING!();

macro_rules! SET_EXTENDED_FUNCTION_RES_LEN {
	() => {
			6
	};
}
pub const SET_EXTENDED_FUNCTION_RES_LEN: u8 = SET_EXTENDED_FUNCTION_RES_LEN!();
// -----

// -----
// Set the extended function.
macro_rules! SWITCH_FUNCTION_PANEL_COMMAND {
	() => {
		"33"
	};
}
pub const SWITCH_FUNCTION_PANEL_COMMAND: &'static str =
	SWITCH_FUNCTION_PANEL_COMMAND!();

macro_rules! SWITCH_FUNCTION_PANEL_ARG_MAIN_CH1 {
	() => {
			"0"
	};
}
pub const SWITCH_FUNCTION_PANEL_ARG_MAIN_CH1: &'static str =
	SWITCH_FUNCTION_PANEL_ARG_MAIN_CH1!();

macro_rules! SWITCH_FUNCTION_PANEL_ARG_MAIN_CH2 {
	() => {
			"1"
	};
}
pub const SWITCH_FUNCTION_PANEL_ARG_MAIN_CH2: &'static str =
	SWITCH_FUNCTION_PANEL_ARG_MAIN_CH2!();

macro_rules! SWITCH_FUNCTION_PANEL_ARG_SYS {
	() => {
			"2"
	};
}
pub const SWITCH_FUNCTION_PANEL_ARG_SYS: &'static str =
	SWITCH_FUNCTION_PANEL_ARG_SYS!();

macro_rules! SWITCH_FUNCTION_PANEL_ARG_MEASUREMENT {
	() => {
			"4"
	};
}
pub const SWITCH_FUNCTION_PANEL_ARG_MEASUREMENT: &'static str =
	SWITCH_FUNCTION_PANEL_ARG_MEASUREMENT!();

macro_rules! SWITCH_FUNCTION_PANEL_ARG_COUNTING {
	() => {
			"5"
	};
}
pub const SWITCH_FUNCTION_PANEL_ARG_COUNTING: &'static str =
	SWITCH_FUNCTION_PANEL_ARG_COUNTING!();

macro_rules! SWITCH_FUNCTION_PANEL_ARG_SWEEP_CH1 {
	() => {
			"6"
	};
}
pub const SWITCH_FUNCTION_PANEL_ARG_SWEEP_CH1: &'static str =
	SWITCH_FUNCTION_PANEL_ARG_SWEEP_CH1!();

macro_rules! SWITCH_FUNCTION_PANEL_ARG_SWEEP_CH2 {
	() => {
			"7"
	};
}
pub const SWITCH_FUNCTION_PANEL_ARG_SWEEP_CH2: &'static str =
	SWITCH_FUNCTION_PANEL_ARG_SWEEP_CH2!();

macro_rules! SWITCH_FUNCTION_PANEL_ARG_PULSE {
	() => {
			"8"
	};
}
pub const SWITCH_FUNCTION_PANEL_ARG_PULSE: &'static str =
	SWITCH_FUNCTION_PANEL_ARG_PULSE!();

macro_rules! SWITCH_FUNCTION_PANEL_ARG_BURST {
	() => {
			"9"
	};
}
pub const SWITCH_FUNCTION_PANEL_ARG_BURST: &'static str =
	SWITCH_FUNCTION_PANEL_ARG_BURST!();

// command example - switch to main ch1:
// ":w33=0.\r\n"
macro_rules! SWITCH_FUNCTION_PANEL_MAIN_CH1 {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SWITCH_FUNCTION_PANEL_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SWITCH_FUNCTION_PANEL_ARG_MAIN_CH1!(),
			COMMAND_END!(),
			)
	};
}
pub const SWITCH_FUNCTION_PANEL_MAIN_CH1: &str = SWITCH_FUNCTION_PANEL_MAIN_CH1!();

// command example - switch to main ch2:
// ":w33=1.\r\n"
macro_rules! SWITCH_FUNCTION_PANEL_MAIN_CH2 {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SWITCH_FUNCTION_PANEL_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SWITCH_FUNCTION_PANEL_ARG_MAIN_CH2!(),
			COMMAND_END!(),
			)
	};
}
pub const SWITCH_FUNCTION_PANEL_MAIN_CH2: &str = SWITCH_FUNCTION_PANEL_MAIN_CH2!();

// command example - switch to system settings:
// ":w33=2.\r\n"
macro_rules! SWITCH_FUNCTION_PANEL_SYS {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SWITCH_FUNCTION_PANEL_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SWITCH_FUNCTION_PANEL_ARG_SYS!(),
			COMMAND_END!(),
			)
	};
}
pub const SWITCH_FUNCTION_PANEL_SYS: &str = SWITCH_FUNCTION_PANEL_SYS!();

// command example - switch to measurement:
// ":w33=4.\r\n"
macro_rules! SWITCH_FUNCTION_PANEL_MEASUREMENT {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SWITCH_FUNCTION_PANEL_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SWITCH_FUNCTION_PANEL_ARG_MEASUREMENT!(),
			COMMAND_END!(),
			)
	};
}
pub const SWITCH_FUNCTION_PANEL_MEASUREMENT: &str =
	SWITCH_FUNCTION_PANEL_MEASUREMENT!();

// command example - switch to counting:
// ":w33=5.\r\n"
macro_rules! SWITCH_FUNCTION_PANEL_COUNTING {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SWITCH_FUNCTION_PANEL_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SWITCH_FUNCTION_PANEL_ARG_COUNTING!(),
			COMMAND_END!(),
			)
	};
}
pub const SWITCH_FUNCTION_PANEL_COUNTING: &str = SWITCH_FUNCTION_PANEL_COUNTING!();

// command example - switch to sweep channel 1:
// ":w33=6.\r\n"
macro_rules! SWITCH_FUNCTION_PANEL_SWEEP_CH1 {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SWITCH_FUNCTION_PANEL_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SWITCH_FUNCTION_PANEL_ARG_SWEEP_CH1!(),
			COMMAND_END!(),
			)
	};
}
pub const SWITCH_FUNCTION_PANEL_SWEEP_CH1: &str = SWITCH_FUNCTION_PANEL_SWEEP_CH1!();

// command example - switch to sweep channel 2:
// ":w33=7.\r\n"
macro_rules! SWITCH_FUNCTION_PANEL_SWEEP_CH2 {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SWITCH_FUNCTION_PANEL_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SWITCH_FUNCTION_PANEL_ARG_SWEEP_CH2!(),
			COMMAND_END!(),
			)
	};
}
pub const SWITCH_FUNCTION_PANEL_SWEEP_CH2: &str = SWITCH_FUNCTION_PANEL_SWEEP_CH2!();

// command example - switch to pulse:
// ":w33=8.\r\n"
macro_rules! SWITCH_FUNCTION_PANEL_PULSE {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SWITCH_FUNCTION_PANEL_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SWITCH_FUNCTION_PANEL_ARG_PULSE!(),
			COMMAND_END!(),
			)
	};
}
pub const SWITCH_FUNCTION_PANEL_PULSE: &str = SWITCH_FUNCTION_PANEL_PULSE!();

// command example - switch to burst:
// ":w33=9.\r\n"
macro_rules! SWITCH_FUNCTION_PANEL_BURST {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SWITCH_FUNCTION_PANEL_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SWITCH_FUNCTION_PANEL_ARG_BURST!(),
			COMMAND_END!(),
			)
	};
}
pub const SWITCH_FUNCTION_PANEL_BURST: &str = SWITCH_FUNCTION_PANEL_BURST!();

macro_rules! SWITCH_FUNCTION_PANEL_RES_LEN {
	() => {
			6
	};
}
pub const SWITCH_FUNCTION_PANEL_RES_LEN: u8 = SWITCH_FUNCTION_PANEL_RES_LEN!();
// -----

// -----
// Set measurement coupling.
macro_rules! SET_MEASUREMENT_COUPLING_COMMAND {
	() => {
		"36"
	};
}
pub const SET_MEASUREMENT_COUPLING_COMMAND: &'static str = SET_MEASUREMENT_COUPLING_COMMAND!();

macro_rules! SET_MEASUREMENT_COUPLING_ARG_AC {
	() => {
			"0"
	};
}
pub const SET_MEASUREMENT_COUPLING_ARG_AC: &'static str = SET_MEASUREMENT_COUPLING_ARG_AC!();

macro_rules! SET_MEASUREMENT_COUPLING_ARG_DC {
	() => {
			"1"
	};
}
pub const SET_MEASUREMENT_COUPLING_ARG_DC: &'static str = SET_MEASUREMENT_COUPLING_ARG_DC!();

// command example - set AC measurement coupling:
// ":w36=0.\r\n"
macro_rules! SET_MEASUREMENT_COUPLING_AC {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_MEASUREMENT_COUPLING_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_MEASUREMENT_COUPLING_ARG_AC!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_MEASUREMENT_COUPLING_AC: &str = SET_MEASUREMENT_COUPLING_AC!();

// command example - set DC measurement coupling:
// ":w36=1.\r\n"
macro_rules! SET_MEASUREMENT_COUPLING_DC {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_MEASUREMENT_COUPLING_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_MEASUREMENT_COUPLING_ARG_DC!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_MEASUREMENT_COUPLING_DC: &str = SET_MEASUREMENT_COUPLING_DC!();

macro_rules! SET_MEASUREMENT_COUPLING_RES_LEN {
	() => {
			6
	};
}
pub const SET_MEASUREMENT_COUPLING_RES_LEN: u8 = SET_MEASUREMENT_COUPLING_RES_LEN!();
// -----

// -----
// Set measurement gate time.
// Ex.
//  gate time = 1 second: ":w37=100"
macro_rules! SET_MEASUREMENT_GATE_TIME_COMMAND {
	() => {
		"37"
	};
}
pub const SET_MEASUREMENT_GATE_TIME_COMMAND: &'static str =
	SET_MEASUREMENT_GATE_TIME_COMMAND!();

macro_rules! SET_MEASUREMENT_GATE_TIME_RES_LEN {
	() => {
			6
	};
}
pub const SET_MEASUREMENT_GATE_TIME_RES_LEN: u8 = SET_MEASUREMENT_GATE_TIME_RES_LEN!();
// -----

// -----
// Set measurement mode (count frequency or counting period).
macro_rules! SET_MEASUREMENT_MODE_COMMAND {
	() => {
		"38"
	};
}
pub const SET_MEASUREMENT_MODE_COMMAND: &'static str = SET_MEASUREMENT_MODE_COMMAND!();

macro_rules! SET_MEASUREMENT_MODE_ARG_COUNT_FREQUENCY {
	() => {
			"0"
	};
}
pub const SET_MEASUREMENT_MODE_ARG_COUNT_FREQUENCY: &'static str =
	SET_MEASUREMENT_MODE_ARG_COUNT_FREQUENCY!();

macro_rules! SET_MEASUREMENT_MODE_ARG_COUNTING_PERIOD {
	() => {
			"1"
	};
}
pub const SET_MEASUREMENT_MODE_ARG_COUNTING_PERIOD: &'static str =
	SET_MEASUREMENT_MODE_ARG_COUNTING_PERIOD!();

// command example:
// ":w38=0.\r\n"
macro_rules! SET_MEASUREMENT_MODE_COUNT_FREQUENCY {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_MEASUREMENT_MODE_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_MEASUREMENT_MODE_ARG_COUNT_FREQUENCY!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_MEASUREMENT_MODE_COUNT_FREQUENCY: &'static str =
	SET_MEASUREMENT_MODE_COUNT_FREQUENCY!();

// command example:
// ":w38=1.\r\n"
macro_rules! SET_MEASUREMENT_MODE_COUNTING_PERIOD {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_MEASUREMENT_MODE_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_MEASUREMENT_MODE_ARG_COUNTING_PERIOD!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_MEASUREMENT_MODE_COUNTING_PERIOD: &'static str =
	SET_MEASUREMENT_MODE_COUNTING_PERIOD!();

macro_rules! SET_MEASUREMENT_MODE_RES_LEN {
	() => {
			6
	};
}
pub const SET_MEASUREMENT_MODE_RES_LEN: u8 = SET_MEASUREMENT_MODE_RES_LEN!();
// -----

// -----
// Get measurement count value.
macro_rules! GET_MEASUREMENT_COUNT_COMMAND {
	() => {
		"80"
	};
}
pub const GET_MEASUREMENT_COUNT_COMMAND: &'static str = GET_MEASUREMENT_COUNT_COMMAND!();

macro_rules! GET_MEASUREMENT_COUNT_ARG_VAL {
	() => {
			"0"
	};
}
pub const GET_MEASUREMENT_COUNT_ARG_VAL: &'static str = GET_MEASUREMENT_COUNT_ARG_VAL!();

// command example:
// ":r80=0.\r\n"
macro_rules! GET_MEASUREMENT_COUNT {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_GET!(),
			GET_MEASUREMENT_COUNT_COMMAND!(),
			COMMAND_SEPARATOR!(),
			GET_MEASUREMENT_COUNT_ARG_VAL!(),
			COMMAND_END!(),
			)
	};
}
pub const GET_MEASUREMENT_COUNT: &'static str = GET_MEASUREMENT_COUNT!();

macro_rules! GET_MEASUREMENT_COUNT_RES_LEN {
	() => {
			18
	};
}
pub const GET_MEASUREMENT_COUNT_RES_LEN: u8 = GET_MEASUREMENT_COUNT_RES_LEN!();
// -----

// -----
// Get measurement frequency value in frequency mode.
macro_rules! GET_MEASUREMENT_FREQUENCY_COMMAND {
	() => {
		"81"
	};
}
pub const GET_MEASUREMENT_FREQUENCY_COMMAND: &'static str = GET_MEASUREMENT_FREQUENCY_COMMAND!();

macro_rules! GET_MEASUREMENT_FREQUENCY_ARG_VAL {
	() => {
			"0"
	};
}
pub const GET_MEASUREMENT_FREQUENCY_ARG_VAL: &'static str = GET_MEASUREMENT_FREQUENCY_ARG_VAL!();

// command example:
// ":r81=0.\r\n"
macro_rules! GET_MEASUREMENT_FREQUENCY {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_GET!(),
			GET_MEASUREMENT_FREQUENCY_COMMAND!(),
			COMMAND_SEPARATOR!(),
			GET_MEASUREMENT_FREQUENCY_ARG_VAL!(),
			COMMAND_END!(),
			)
	};
}
pub const GET_MEASUREMENT_FREQUENCY: &'static str = GET_MEASUREMENT_FREQUENCY!();

macro_rules! GET_MEASUREMENT_FREQUENCY_RES_LEN {
	() => {
			16
	};
}
pub const GET_MEASUREMENT_FREQUENCY_RES_LEN: u8 = GET_MEASUREMENT_FREQUENCY_RES_LEN!();
// -----

// -----
// Get measurement frequency value in period mode.
macro_rules! GET_MEASUREMENT_FREQUENCY_PERIOD_COMMAND {
	() => {
		"82"
	};
}
pub const GET_MEASUREMENT_FREQUENCY_PERIOD_COMMAND: &'static str =
	GET_MEASUREMENT_FREQUENCY_PERIOD_COMMAND!();

macro_rules! GET_MEASUREMENT_FREQUENCY_PERIOD_ARG_VAL {
	() => {
			"0"
	};
}
pub const GET_MEASUREMENT_FREQUENCY_PERIOD_ARG_VAL: &'static str =
	GET_MEASUREMENT_FREQUENCY_PERIOD_ARG_VAL!();

// command example:
// ":r82=0.\r\n"
macro_rules! GET_MEASUREMENT_FREQUENCY_PERIOD {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_GET!(),
			GET_MEASUREMENT_FREQUENCY_PERIOD_COMMAND!(),
			COMMAND_SEPARATOR!(),
			GET_MEASUREMENT_FREQUENCY_PERIOD_ARG_VAL!(),
			COMMAND_END!(),
			)
	};
}
pub const GET_MEASUREMENT_FREQUENCY_PERIOD: &'static str = GET_MEASUREMENT_FREQUENCY_PERIOD!();

macro_rules! GET_MEASUREMENT_FREQUENCY_PERIOD_RES_LEN {
	() => {
			16
	};
}
pub const GET_MEASUREMENT_FREQUENCY_PERIOD_RES_LEN: u8 =
	GET_MEASUREMENT_FREQUENCY_PERIOD_RES_LEN!();
// -----

// -----
// Get measurement pulse width (positive).
macro_rules! GET_MEASUREMENT_PULSE_WIDTH_POSITIVE_COMMAND {
	() => {
		"83"
	};
}
pub const GET_MEASUREMENT_PULSE_WIDTH_POSITIVE_COMMAND: &'static str =
	GET_MEASUREMENT_PULSE_WIDTH_POSITIVE_COMMAND!();

macro_rules! GET_MEASUREMENT_PULSE_WIDTH_POSITIVE_ARG_VAL {
	() => {
			"0"
	};
}
pub const GET_MEASUREMENT_PULSE_WIDTH_POSITIVE_ARG_VAL: &'static str =
	GET_MEASUREMENT_PULSE_WIDTH_POSITIVE_ARG_VAL!();

// command example:
// ":r83=0.\r\n"
macro_rules! GET_MEASUREMENT_PULSE_WIDTH_POSITIVE {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_GET!(),
			GET_MEASUREMENT_PULSE_WIDTH_POSITIVE_COMMAND!(),
			COMMAND_SEPARATOR!(),
			GET_MEASUREMENT_PULSE_WIDTH_POSITIVE_ARG_VAL!(),
			COMMAND_END!(),
			)
	};
}
pub const GET_MEASUREMENT_PULSE_WIDTH_POSITIVE: &'static str = GET_MEASUREMENT_PULSE_WIDTH_POSITIVE!();

// POSSIBLE BUG: Not sure if this is the correct response length.
macro_rules! GET_MEASUREMENT_PULSE_WIDTH_POSITIVE_RES_LEN {
	() => {
			12
	};
}
pub const GET_MEASUREMENT_PULSE_WIDTH_POSITIVE_RES_LEN: u8 =
	GET_MEASUREMENT_PULSE_WIDTH_POSITIVE_RES_LEN!();
// -----

// -----
// Get measurement pulse width (negative).
macro_rules! GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE_COMMAND {
	() => {
		"84"
	};
}
pub const GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE_COMMAND: &'static str =
	GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE_COMMAND!();

macro_rules! GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE_ARG_VAL {
	() => {
			"0"
	};
}
pub const GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE_ARG_VAL: &'static str =
	GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE_ARG_VAL!();

// command example:
// ":r84=0.\r\n"
macro_rules! GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_GET!(),
			GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE_COMMAND!(),
			COMMAND_SEPARATOR!(),
			GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE_ARG_VAL!(),
			COMMAND_END!(),
			)
	};
}
pub const GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE: &'static str = GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE!();

// POSSIBLE BUG: Not sure if this is the correct response length.
macro_rules! GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE_RES_LEN {
	() => {
			12
	};
}
pub const GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE_RES_LEN: u8 =
	GET_MEASUREMENT_PULSE_WIDTH_NEGATIVE_RES_LEN!();
// -----

// -----
// Get measurement period.
macro_rules! GET_MEASUREMENT_PERIOD_COMMAND {
	() => {
		"85"
	};
}
pub const GET_MEASUREMENT_PERIOD_COMMAND: &'static str =
	GET_MEASUREMENT_PERIOD_COMMAND!();

macro_rules! GET_MEASUREMENT_PERIOD_ARG_VAL {
	() => {
			"0"
	};
}
pub const GET_MEASUREMENT_PERIOD_ARG_VAL: &'static str =
	GET_MEASUREMENT_PERIOD_ARG_VAL!();

// command example:
// ":r85=0.\r\n"
macro_rules! GET_MEASUREMENT_PERIOD {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_GET!(),
			GET_MEASUREMENT_PERIOD_COMMAND!(),
			COMMAND_SEPARATOR!(),
			GET_MEASUREMENT_PERIOD_ARG_VAL!(),
			COMMAND_END!(),
			)
	};
}
pub const GET_MEASUREMENT_PERIOD: &'static str = GET_MEASUREMENT_PERIOD!();

// POSSIBLE BUG: Not sure if this is the correct response length.
macro_rules! GET_MEASUREMENT_PERIOD_RES_LEN {
	() => {
			12
	};
}
pub const GET_MEASUREMENT_PERIOD_RES_LEN: u8 =
	GET_MEASUREMENT_PERIOD_RES_LEN!();
// -----

// -----
// Get measurement duty cycle.
macro_rules! GET_MEASUREMENT_DUTY_CYCLE_COMMAND {
	() => {
		"86"
	};
}
pub const GET_MEASUREMENT_DUTY_CYCLE_COMMAND: &'static str =
	GET_MEASUREMENT_DUTY_CYCLE_COMMAND!();

macro_rules! GET_MEASUREMENT_DUTY_CYCLE_ARG_VAL {
	() => {
			"0"
	};
}
pub const GET_MEASUREMENT_DUTY_CYCLE_ARG_VAL: &'static str =
	GET_MEASUREMENT_DUTY_CYCLE_ARG_VAL!();

// command example:
// ":r86=0.\r\n"
macro_rules! GET_MEASUREMENT_DUTY_CYCLE {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_GET!(),
			GET_MEASUREMENT_DUTY_CYCLE_COMMAND!(),
			COMMAND_SEPARATOR!(),
			GET_MEASUREMENT_DUTY_CYCLE_ARG_VAL!(),
			COMMAND_END!(),
			)
	};
}
pub const GET_MEASUREMENT_DUTY_CYCLE: &'static str = GET_MEASUREMENT_DUTY_CYCLE!();

// POSSIBLE BUG: Not sure if this is the correct response length.
macro_rules! GET_MEASUREMENT_DUTY_CYCLE_RES_LEN {
	() => {
			12
	};
}
pub const GET_MEASUREMENT_DUTY_CYCLE_RES_LEN: u8 =
	GET_MEASUREMENT_DUTY_CYCLE_RES_LEN!();
// -----

// -----
// Set measurement count clear.
macro_rules! SET_MEASUREMENT_COUNT_CLEAR_COMMAND {
	() => {
		"39"
	};
}
pub const SET_MEASUREMENT_COUNT_CLEAR_COMMAND: &'static str =
	SET_MEASUREMENT_COUNT_CLEAR_COMMAND!();

macro_rules! SET_MEASUREMENT_COUNT_CLEAR_ARG {
	() => {
			"0"
	};
}
pub const SET_MEASUREMENT_COUNT_CLEAR_ARG: &'static str = SET_MEASUREMENT_COUNT_CLEAR_ARG!();

// command example:
// ":w39=0.\r\n"
macro_rules! SET_MEASUREMENT_COUNT_CLEAR {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_MEASUREMENT_COUNT_CLEAR_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_MEASUREMENT_COUNT_CLEAR_ARG!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_MEASUREMENT_COUNT_CLEAR: &'static str = SET_MEASUREMENT_COUNT_CLEAR!();

macro_rules! SET_MEASUREMENT_COUNT_CLEAR_RES_LEN {
	() => {
			6
	};
}
pub const SET_MEASUREMENT_COUNT_CLEAR_RES_LEN: u8 = SET_MEASUREMENT_COUNT_CLEAR_RES_LEN!();
// -----

// -----
// Set burst pulse number.
// command example:
// ":w49=5.\r\n"
macro_rules! SET_BURST_PULSE_NUMBER_COMMAND {
	() => {
		"49"
	};
}
pub const SET_BURST_PULSE_NUMBER_COMMAND: &'static str = SET_BURST_PULSE_NUMBER_COMMAND!();

macro_rules! SET_BURST_PULSE_NUMBER_RES_LEN {
	() => {
			6
	};
}
pub const SET_BURST_PULSE_NUMBER_RES_LEN: u8 = SET_BURST_PULSE_NUMBER_RES_LEN!();
// -----

// -----
// Set burst pulse once.
macro_rules! START_BURST_PULSE_ONCE_COMMAND {
	() => {
		"59"
	};
}
pub const START_BURST_PULSE_ONCE_COMMAND: &'static str = START_BURST_PULSE_ONCE_COMMAND!();

macro_rules! START_BURST_PULSE_ONCE_ARG {
	() => {
			"1"
	};
}
pub const START_BURST_PULSE_ONCE_ARG: &'static str = START_BURST_PULSE_ONCE_ARG!();

// command example:
// ":w59=1.\r\n"
macro_rules! START_BURST_PULSE_ONCE {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			START_BURST_PULSE_ONCE_COMMAND!(),
			COMMAND_SEPARATOR!(),
			START_BURST_PULSE_ONCE_ARG!(),
			COMMAND_END!(),
			)
	};
}
pub const START_BURST_PULSE_ONCE: &'static str = START_BURST_PULSE_ONCE!();

macro_rules! START_BURST_PULSE_ONCE_RES_LEN {
	() => {
			6
	};
}
pub const START_BURST_PULSE_ONCE_RES_LEN: u8 = START_BURST_PULSE_ONCE_RES_LEN!();
// -----

// -----
// Set burst mode.
macro_rules! SET_BURST_MODE_COMMAND {
	() => {
		"50"
	};
}
pub const SET_BURST_MODE_COMMAND: &'static str = SET_BURST_MODE_COMMAND!();

macro_rules! SET_BURST_MODE_ARG_MANUAL_TRIGGER {
	() => {
			"0"
	};
}
pub const SET_BURST_MODE_ARG_MANUAL_TRIGGER: &'static str =
	SET_BURST_MODE_ARG_MANUAL_TRIGGER!();

macro_rules! SET_BURST_MODE_ARG_CH2_BURST {
	() => {
			"1"
	};
}
pub const SET_BURST_MODE_ARG_CH2_BURST: &'static str = SET_BURST_MODE_ARG_CH2_BURST!();

macro_rules! SET_BURST_MODE_ARG_EXTERNAL_BURST_AC {
	() => {
			"2"
	};
}
pub const SET_BURST_MODE_ARG_EXTERNAL_BURST_AC: &'static str =
	SET_BURST_MODE_ARG_EXTERNAL_BURST_AC!();

macro_rules! SET_BURST_MODE_ARG_EXTERNAL_BURST_DC {
	() => {
			"3"
	};
}
pub const SET_BURST_MODE_ARG_EXTERNAL_BURST_DC: &'static str =
	SET_BURST_MODE_ARG_EXTERNAL_BURST_DC!();

// command example:
// ":w50=0.\r\n"
macro_rules! SET_BURST_MODE_MANUAL_TRIGGER {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_BURST_MODE_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_BURST_MODE_ARG_MANUAL_TRIGGER!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_BURST_MODE_MANUAL_TRIGGER: &'static str = SET_BURST_MODE_MANUAL_TRIGGER!();

// command example:
// ":w50=1.\r\n"
macro_rules! SET_BURST_MODE_CH2_BURST {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_BURST_MODE_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_BURST_MODE_ARG_CH2_BURST!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_BURST_MODE_CH2_BURST: &'static str = SET_BURST_MODE_CH2_BURST!();

// command example:
// ":w50=2.\r\n"
macro_rules! SET_BURST_MODE_EXTERNAL_BURST_AC {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_BURST_MODE_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_BURST_MODE_ARG_EXTERNAL_BURST_AC!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_BURST_MODE_EXTERNAL_BURST_AC: &'static str = SET_BURST_MODE_EXTERNAL_BURST_AC!();

// command example:
// ":w50=3.\r\n"
macro_rules! SET_BURST_MODE_EXTERNAL_BURST_DC {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_BURST_MODE_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_BURST_MODE_ARG_EXTERNAL_BURST_DC!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_BURST_MODE_EXTERNAL_BURST_DC: &'static str = SET_BURST_MODE_EXTERNAL_BURST_DC!();

macro_rules! SET_BURST_MODE_RES_LEN {
	() => {
			6
	};
}
pub const SET_BURST_MODE_RES_LEN: u8 = SET_BURST_MODE_RES_LEN!();
// -----

// -----
// Set sweep starting frequency.
// command example:
// 10Hz:
//   ":w40=1000.\r\n"
macro_rules! SET_SWEEP_STARTING_FREQUENCY_COMMAND {
	() => {
		"40"
	};
}
pub const SET_SWEEP_STARTING_FREQUENCY_COMMAND: &'static str =
	SET_SWEEP_STARTING_FREQUENCY_COMMAND!();

macro_rules! SET_SWEEP_STARTING_FREQUENCY_RES_LEN {
	() => {
			6
	};
}
pub const SET_SWEEP_STARTING_FREQUENCY_RES_LEN: u8 = SET_SWEEP_STARTING_FREQUENCY_RES_LEN!();
// -----

// -----
// Set sweep termination frequency.
// command example:
// 10Hz:
//   ":w41=1000.\r\n"
macro_rules! SET_SWEEP_END_FREQUENCY_COMMAND {
	() => {
		"41"
	};
}
pub const SET_SWEEP_END_FREQUENCY_COMMAND: &'static str =
	SET_SWEEP_END_FREQUENCY_COMMAND!();

macro_rules! SET_SWEEP_END_FREQUENCY_RES_LEN {
	() => {
			6
	};
}
pub const SET_SWEEP_END_FREQUENCY_RES_LEN: u8 =
	SET_SWEEP_END_FREQUENCY_RES_LEN!();
// -----

// -----
// Set sweep time.
// command example:
// 1 second:
//   ":w42=10.\r\n"
macro_rules! SET_SWEEP_TIME_COMMAND {
	() => {
		"42"
	};
}
pub const SET_SWEEP_TIME_COMMAND: &'static str = SET_SWEEP_TIME_COMMAND!();

macro_rules! SET_SWEEP_TIME_RES_LEN {
	() => {
			6
	};
}
pub const SET_SWEEP_TIME_RES_LEN: u8 = SET_SWEEP_TIME_RES_LEN!();
// -----

// -----
// Set sweep direction.
macro_rules! SET_SWEEP_DIRECTION_COMMAND {
	() => {
		"43"
	};
}
pub const SET_SWEEP_DIRECTION_COMMAND: &'static str = SET_SWEEP_DIRECTION_COMMAND!();

macro_rules! SET_SWEEP_DIRECTION_ARG_RISE {
	() => {
			"0"
	};
}
pub const SET_SWEEP_DIRECTION_ARG_RISE: &'static str = SET_SWEEP_DIRECTION_ARG_RISE!();

macro_rules! SET_SWEEP_DIRECTION_ARG_FALL {
	() => {
			"1"
	};
}
pub const SET_SWEEP_DIRECTION_ARG_FALL: &'static str = SET_SWEEP_DIRECTION_ARG_FALL!();

macro_rules! SET_SWEEP_DIRECTION_ARG_RISE_FALL {
	() => {
			"2"
	};
}
pub const SET_SWEEP_DIRECTION_ARG_RISE_FALL: &'static str =
	SET_SWEEP_DIRECTION_ARG_RISE_FALL!();

// command example:
// ":w43=0.\r\n"
macro_rules! SET_SWEEP_DIRECTION_RISE {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_SWEEP_DIRECTION_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_SWEEP_DIRECTION_ARG_RISE!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_SWEEP_DIRECTION_RISE: &'static str = SET_SWEEP_DIRECTION_RISE!();

// command example:
// ":w43=1.\r\n"
macro_rules! SET_SWEEP_DIRECTION_FALL {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_SWEEP_DIRECTION_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_SWEEP_DIRECTION_ARG_FALL!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_SWEEP_DIRECTION_FALL: &'static str = SET_SWEEP_DIRECTION_FALL!();

// command example:
// ":w43=2.\r\n"
macro_rules! SET_SWEEP_DIRECTION_RISE_FALL {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_SWEEP_DIRECTION_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_SWEEP_DIRECTION_ARG_RISE_FALL!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_SWEEP_DIRECTION_RISE_FALL: &'static str = SET_SWEEP_DIRECTION_RISE_FALL!();

macro_rules! SET_SWEEP_DIRECTION_RES_LEN {
	() => {
			6
	};
}
pub const SET_SWEEP_DIRECTION_RES_LEN: u8 = SET_SWEEP_DIRECTION_RES_LEN!();
// -----

// -----
// Set sweep mode.
macro_rules! SET_SWEEP_MODE_COMMAND {
	() => {
		"44"
	};
}
pub const SET_SWEEP_MODE_COMMAND: &'static str = SET_SWEEP_MODE_COMMAND!();

macro_rules! SET_SWEEP_MODE_ARG_LINEAR {
	() => {
			"0"
	};
}
pub const SET_SWEEP_MODE_ARG_LINEAR: &'static str = SET_SWEEP_MODE_ARG_LINEAR!();

macro_rules! SET_SWEEP_MODE_ARG_LOGARITHM {
	() => {
			"1"
	};
}
pub const SET_SWEEP_MODE_ARG_LOGARITHM: &'static str = SET_SWEEP_MODE_ARG_LOGARITHM!();

// command example:
// ":w44=0.\r\n"
macro_rules! SET_SWEEP_MODE_LINEAR {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_SWEEP_MODE_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_SWEEP_MODE_ARG_LINEAR!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_SWEEP_MODE_LINEAR: &'static str = SET_SWEEP_MODE_LINEAR!();

// command example:
// ":w44=1.\r\n"
macro_rules! SET_SWEEP_MODE_LOGARITHM {
	() => {
		concat!(
			COMMAND_BEGIN!(),
			COMMAND_SET!(),
			SET_SWEEP_MODE_COMMAND!(),
			COMMAND_SEPARATOR!(),
			SET_SWEEP_MODE_ARG_LOGARITHM!(),
			COMMAND_END!(),
			)
	};
}
pub const SET_SWEEP_MODE_LOGARITHM: &'static str = SET_SWEEP_MODE_LOGARITHM!();

macro_rules! SET_SWEEP_MODE_RES_LEN {
	() => {
			6
	};
}
pub const SET_SWEEP_MODE_RES_LEN: u8 = SET_SWEEP_MODE_RES_LEN!();
// -----

// -----
// Set pulse width.
macro_rules! SET_PULSE_WIDTH_COMMAND {
	() => {
		"45"
	};
}
pub const SET_PULSE_WIDTH_COMMAND: &'static str = SET_PULSE_WIDTH_COMMAND!();

// command example:
// 1000 nanoseconds:
//   ":w45=1000,0.\r\n"
macro_rules! SET_PULSE_WIDTH_ARG_NANOSECONDS {
	() => {
			"0"
	};
}
pub const SET_PULSE_WIDTH_ARG_NANOSECONDS: &'static str = SET_PULSE_WIDTH_ARG_NANOSECONDS!();

macro_rules! SET_PULSE_WIDTH_ARG_NANOSECONDS_MIN {
	() => {
		25.0
	};
}
pub const SET_PULSE_WIDTH_ARG_NANOSECONDS_MIN: f64 = SET_PULSE_WIDTH_ARG_NANOSECONDS_MIN!();

macro_rules! SET_PULSE_WIDTH_ARG_NANOSECONDS_MAX {
	() => {
		4000000000.0
	};
}
pub const SET_PULSE_WIDTH_ARG_NANOSECONDS_MAX: f64 = SET_PULSE_WIDTH_ARG_NANOSECONDS_MAX!();

// command example:
// 1000 microseconds:
//   ":w45=1000,1.\r\n"
macro_rules! SET_PULSE_WIDTH_ARG_MICROSECONDS {
	() => {
			"1"
	};
}
pub const SET_PULSE_WIDTH_ARG_MICROSECONDS: &'static str = SET_PULSE_WIDTH_ARG_MICROSECONDS!();

macro_rules! SET_PULSE_WIDTH_ARG_MICROSECONDS_MIN {
	() => {
			1.0
	};
}
pub const SET_PULSE_WIDTH_ARG_MICROSECONDS_MIN: f64 = SET_PULSE_WIDTH_ARG_MICROSECONDS_MIN!();

macro_rules! SET_PULSE_WIDTH_ARG_MICROSECONDS_MAX {
	() => {
		4000000000.0
	};
}
pub const SET_PULSE_WIDTH_ARG_MICROSECONDS_MAX: f64 = SET_PULSE_WIDTH_ARG_MICROSECONDS_MAX!();

macro_rules! SET_PULSE_WIDTH_RES_LEN {
	() => {
			6
	};
}
pub const SET_PULSE_WIDTH_RES_LEN: u8 = SET_PULSE_WIDTH_RES_LEN!();
// -----

// -----
// Set pulse period.
macro_rules! SET_PULSE_PERIOD_COMMAND {
	() => {
		"46"
	};
}
pub const SET_PULSE_PERIOD_COMMAND: &'static str = SET_PULSE_PERIOD_COMMAND!();

// command example:
// 1000 nanoseconds:
//   ":w46=1000,0.\r\n"
macro_rules! SET_PULSE_PERIOD_ARG_NANOSECONDS {
	() => {
			"0"
	};
}
pub const SET_PULSE_PERIOD_ARG_NANOSECONDS: &'static str = SET_PULSE_PERIOD_ARG_NANOSECONDS!();

macro_rules! SET_PULSE_PERIOD_ARG_NANOSECONDS_MIN {
	() => {
		25.0
	};
}
pub const SET_PULSE_PERIOD_ARG_NANOSECONDS_MIN: f64 = SET_PULSE_PERIOD_ARG_NANOSECONDS_MIN!();

macro_rules! SET_PULSE_PERIOD_ARG_NANOSECONDS_MAX {
	() => {
		4000000000.0
	};
}
pub const SET_PULSE_PERIOD_ARG_NANOSECONDS_MAX: f64 = SET_PULSE_PERIOD_ARG_NANOSECONDS_MAX!();

// command example:
// 1000 microseconds:
//   ":w45=1000,1.\r\n"
macro_rules! SET_PULSE_PERIOD_ARG_MICROSECONDS {
	() => {
			"1"
	};
}
pub const SET_PULSE_PERIOD_ARG_MICROSECONDS: &'static str =
	SET_PULSE_PERIOD_ARG_MICROSECONDS!();

macro_rules! SET_PULSE_PERIOD_ARG_MICROSECONDS_MIN {
	() => {
			1.0
	};
}
pub const SET_PULSE_PERIOD_ARG_MICROSECONDS_MIN: f64 = SET_PULSE_PERIOD_ARG_MICROSECONDS_MIN!();

macro_rules! SET_PULSE_PERIOD_ARG_MICROSECONDS_MAX {
	() => {
		4000000000.0
	};
}
pub const SET_PULSE_PERIOD_ARG_MICROSECONDS_MAX: f64 = SET_PULSE_PERIOD_ARG_MICROSECONDS_MAX!();

macro_rules! SET_PULSE_PERIOD_RES_LEN {
	() => {
			6
	};
}
pub const SET_PULSE_PERIOD_RES_LEN: u8 = SET_PULSE_PERIOD_RES_LEN!();
// -----

// -----
// Set pulse offset in percent.
macro_rules! SET_PULSE_OFFSET_COMMAND {
	() => {
		"47"
	};
}
pub const SET_PULSE_OFFSET_COMMAND: &'static str = SET_PULSE_OFFSET_COMMAND!();

// command example:
// 100 percent:
//   ":w47=100.\r\n"
macro_rules! SET_PULSE_OFFSET_ARG_PERCENT_MIN {
	() => {
			0.0
	};
}
pub const SET_PULSE_OFFSET_ARG_PERCENT_MIN: f64 = SET_PULSE_OFFSET_ARG_PERCENT_MIN!();

macro_rules! SET_PULSE_OFFSET_ARG_PERCENT_MAX {
	() => {
		100.0
	};
}
pub const SET_PULSE_OFFSET_ARG_PERCENT_MAX: f64 = SET_PULSE_OFFSET_ARG_PERCENT_MAX!();

macro_rules! SET_PULSE_OFFSET_RES_LEN {
	() => {
			6
	};
}
pub const SET_PULSE_OFFSET_RES_LEN: u8 = SET_PULSE_OFFSET_RES_LEN!();
// -----

// -----
// Set pulse amplitude in volts.
macro_rules! SET_PULSE_AMPLITUDE_COMMAND {
	() => {
		"48"
	};
}
pub const SET_PULSE_AMPLITUDE_COMMAND: &'static str = SET_PULSE_AMPLITUDE_COMMAND!();

// command example:
// 5 volts:
//   ":w48=500.\r\n"
macro_rules! SET_PULSE_AMPLITUDE_ARG_VOLTS_MIN {
	() => {
			0.0
	};
}
pub const SET_PULSE_AMPLITUDE_ARG_VOLTS_MIN: f64 = SET_PULSE_AMPLITUDE_ARG_VOLTS_MIN!();

macro_rules! SET_PULSE_AMPLITUDE_ARG_VOLTS_MAX {
	() => {
		10.0
	};
}
pub const SET_PULSE_AMPLITUDE_ARG_VOLTS_MAX: f64 = SET_PULSE_AMPLITUDE_ARG_VOLTS_MAX!();

macro_rules! SET_PULSE_AMPLITUDE_RES_LEN {
	() => {
			6
	};
}
pub const SET_PULSE_AMPLITUDE_RES_LEN: u8 = SET_PULSE_AMPLITUDE_RES_LEN!();
// -----

// -----
// Save all values as a numbered preset.
macro_rules! SAVE_PRESET_COMMAND {
	() => {
		"70"
	};
}
pub const SAVE_PRESET_COMMAND: &'static str = SAVE_PRESET_COMMAND!();

// command example:
// save as preset 5:
//   ":w70=5.\r\n"
macro_rules! SAVE_PRESET_ARG_NUM_MIN {
	() => {
			0.0
	};
}
pub const SAVE_PRESET_ARG_NUM_MIN: f64 = SAVE_PRESET_ARG_NUM_MIN!();

macro_rules! SAVE_PRESET_ARG_NUM_MAX {
	() => {
		99.0
	};
}
pub const SAVE_PRESET_ARG_NUM_MAX: f64 = SAVE_PRESET_ARG_NUM_MAX!();

macro_rules! SAVE_PRESET_RES_LEN {
	() => {
			6
	};
}
pub const SAVE_PRESET_RES_LEN: u8 = SAVE_PRESET_RES_LEN!();
// -----

// -----
// Recall all values from a numbered preset.
macro_rules! LOAD_PRESET_COMMAND {
	() => {
		"71"
	};
}
pub const LOAD_PRESET_COMMAND: &'static str = LOAD_PRESET_COMMAND!();

// command example:
// recall preset 5:
//   ":w71=5.\r\n"
macro_rules! LOAD_PRESET_ARG_NUM_MIN {
	() => {
			0.0
	};
}
pub const LOAD_PRESET_ARG_NUM_MIN: f64 = LOAD_PRESET_ARG_NUM_MIN!();

macro_rules! LOAD_PRESET_ARG_NUM_MAX {
	() => {
		99.0
	};
}
pub const LOAD_PRESET_ARG_NUM_MAX: f64 = LOAD_PRESET_ARG_NUM_MAX!();

macro_rules! LOAD_PRESET_RES_LEN {
	() => {
			6
	};
}
pub const LOAD_PRESET_RES_LEN: u8 = LOAD_PRESET_RES_LEN!();
// -----

// -----
// Clear a numbered preset.
//
// NOTE: It doesn't work. The spec must be wrong.
macro_rules! SET_CLEAR_PRESET_COMMAND {
	() => {
		"72"
	};
}
pub const SET_CLEAR_PRESET_COMMAND: &'static str = SET_CLEAR_PRESET_COMMAND!();

// command example:
// clear preset 5:
//   ":w72=5.\r\n"
macro_rules! SET_CLEAR_PRESET_ARG_NUM_MIN {
	() => {
			0.0
	};
}
pub const SET_CLEAR_PRESET_ARG_NUM_MIN: f64 = SET_CLEAR_PRESET_ARG_NUM_MIN!();

macro_rules! SET_CLEAR_PRESET_ARG_NUM_MAX {
	() => {
		99.0
	};
}
pub const SET_CLEAR_PRESET_ARG_NUM_MAX: f64 = SET_CLEAR_PRESET_ARG_NUM_MAX!();

macro_rules! SET_CLEAR_PRESET_RES_LEN {
	() => {
			6
	};
}
pub const SET_CLEAR_PRESET_RES_LEN: u8 = SET_CLEAR_PRESET_RES_LEN!();
// -----

// -----
// Write an arbitrary wave to the device.
macro_rules! SET_ARBITRARY_WAVE_COMMAND {
	() => {
			"a"
	};
}
pub const SET_ARBITRARY_WAVE_COMMAND: &'static str = SET_ARBITRARY_WAVE_COMMAND!();

// command example:
// write arbitrary wave to slot 1:
//   ":a01=2048,2048,...2048.\r\n"
macro_rules! SET_ARBITRARY_WAVE_ARG_NUM_MIN {
	() => {
			1.0
	};
}
pub const SET_ARBITRARY_WAVE_ARG_NUM_MIN: f64 = SET_ARBITRARY_WAVE_ARG_NUM_MIN!();

macro_rules! SET_ARBITRARY_WAVE_ARG_NUM_MAX {
	() => {
		60.0
	};
}
pub const SET_ARBITRARY_WAVE_ARG_NUM_MAX: f64 = SET_ARBITRARY_WAVE_ARG_NUM_MAX!();

macro_rules! SET_ARBITRARY_WAVE_RES_LEN {
	() => {
			6
	};
}
pub const SET_ARBITRARY_WAVE_RES_LEN: u8 = SET_ARBITRARY_WAVE_RES_LEN!();
// -----

// -----
// Read an arbitrary wave from the device.
macro_rules! GET_ARBITRARY_WAVE_COMMAND {
	() => {
			"b"
	};
}
pub const GET_ARBITRARY_WAVE_COMMAND: &'static str = GET_ARBITRARY_WAVE_COMMAND!();

// command example:
// read arbitrary wave from slot 1:
//   ":b01=0.\r\n"
macro_rules! GET_ARBITRARY_WAVE_ARG_NUM_MIN {
	() => {
			1.0
	};
}
pub const GET_ARBITRARY_WAVE_ARG_NUM_MIN: f64 = GET_ARBITRARY_WAVE_ARG_NUM_MIN!();

macro_rules! GET_ARBITRARY_WAVE_ARG_NUM_MAX {
	() => {
		60.0
	};
}
pub const GET_ARBITRARY_WAVE_ARG_NUM_MAX: f64 = GET_ARBITRARY_WAVE_ARG_NUM_MAX!();

macro_rules! GET_ARBITRARY_WAVE_ARG2 {
	() => {
			0
	};
}
pub const GET_ARBITRARY_WAVE_ARG2: u8 = GET_ARBITRARY_WAVE_ARG2!();

macro_rules! GET_ARBITRARY_WAVE_RES_LEN {
	() => {
		10247
	};
}
pub const GET_ARBITRARY_WAVE_RES_LEN: u32 = GET_ARBITRARY_WAVE_RES_LEN!();
// -----
