// -----
macro_rules! SERIAL_TIMEOUT_MS {() => {3000}}
pub const SERIAL_TIMEOUT_MS: u64 = SERIAL_TIMEOUT_MS!();

macro_rules! COMMAND_DELAY_MS {() => {50}}
pub const COMMAND_DELAY_MS: u64 = COMMAND_DELAY_MS!();
// -----


// -----
macro_rules! COMMAND_BEGIN {() => {":"}}
pub const COMMAND_BEGIN: &'static str = COMMAND_BEGIN!();

macro_rules! COMMAND_SEPARATOR {() => {"="}}
pub const COMMAND_SEPARATOR: &'static str = COMMAND_SEPARATOR!();

macro_rules! COMMAND_ARG_SEPARATOR {() => {","}}
pub const COMMAND_ARG_SEPARATOR: &'static str = COMMAND_ARG_SEPARATOR!();

macro_rules! COMMAND_STOP {() => {"."}}
pub const COMMAND_STOP: &'static str = COMMAND_STOP!();

macro_rules! COMMAND_LINEBREAK {() => {"\r\n"}}
pub const COMMAND_LINEBREAK: &'static str = COMMAND_LINEBREAK!();

// ".\r\n"
macro_rules! COMMAND_END {() => {concat!(
    COMMAND_STOP!(), 
    COMMAND_LINEBREAK!(),
)}}
pub const COMMAND_END: &'static str = COMMAND_END!();
// -----


// -----
// Use this to read values from the device.
macro_rules! COMMAND_READ {() => {"r"}}
pub const COMMAND_READ: &'static str = COMMAND_READ!();

// Use this to read values from the device.
macro_rules! COMMAND_WRITE {() => {"w"}}
pub const COMMAND_WRITE: &'static str = COMMAND_WRITE!();
// -----


// -----
// Read the device's model number.
macro_rules! READ_MACHINE_MODEL_COMMAND {() => {"00"}}
pub const READ_MACHINE_MODEL_COMMAND: &'static str = READ_MACHINE_MODEL_COMMAND!();

macro_rules! READ_MACHINE_MODEL_ARG1 {() => {"0"}}
pub const READ_MACHINE_MODEL_ARG1: &'static str = READ_MACHINE_MODEL_ARG1!();

macro_rules! READ_MACHINE_MODEL_RES_LEN {() => {10}}
pub const READ_MACHINE_MODEL_RES_LEN: u8 = READ_MACHINE_MODEL_RES_LEN!();

// command example:
// ":r00=0.\r\n"
macro_rules! READ_MACHINE_MODEL {() => {concat!(
    COMMAND_BEGIN!(),
    COMMAND_READ!(),
    READ_MACHINE_MODEL_COMMAND!(),
    COMMAND_SEPARATOR!(),
    READ_MACHINE_MODEL_ARG1!(),
    COMMAND_END!(),
)}}
pub const READ_MACHINE_MODEL: &'static str = READ_MACHINE_MODEL!();
// -----


// -----
// Read the device's serial number.
macro_rules! READ_MACHINE_NUMBER_COMMAND {() => {"01"}}
pub const READ_MACHINE_NUMBER_COMMAND: &'static str = READ_MACHINE_NUMBER_COMMAND!();

macro_rules! READ_MACHINE_NUMBER_ARG1 {() => {"0"}}
pub const READ_MACHINE_NUMBER_ARG1: &'static str = READ_MACHINE_NUMBER_ARG1!();

macro_rules! READ_MACHINE_NUMBER_RES_LEN {() => {18}}
pub const READ_MACHINE_NUMBER_RES_LEN: u8 = READ_MACHINE_NUMBER_RES_LEN!();

// command example:
// ":r01=0.\r\n"
macro_rules! READ_MACHINE_NUMBER {() => {concat!(
    COMMAND_BEGIN!(),
    COMMAND_READ!(),
    READ_MACHINE_NUMBER_COMMAND!(),
    COMMAND_SEPARATOR!(),
    READ_MACHINE_NUMBER_ARG1!(),
    COMMAND_END!(),
)}}
pub const READ_MACHINE_NUMBER: &'static str = READ_MACHINE_NUMBER!();
// -----


// -----
// Turn output channels on or off.
macro_rules! WRITE_CHANNEL_OUTPUT_COMMAND {() => {"20"}}
pub const WRITE_CHANNEL_OUTPUT_COMMAND: &'static str = WRITE_CHANNEL_OUTPUT_COMMAND!();

macro_rules! WRITE_CHANNEL_OUTPUT_ARG_CH_ON {() => {"1"}}
pub const WRITE_CHANNEL_OUTPUT_ARG_CH_ON: &'static str = WRITE_CHANNEL_OUTPUT_ARG_CH_ON!();

macro_rules! WRITE_CHANNEL_OUTPUT_ARG_CH_OFF {() => {"0"}}
pub const WRITE_CHANNEL_OUTPUT_ARG_CH_OFF: &'static str = WRITE_CHANNEL_OUTPUT_ARG_CH_OFF!();

macro_rules! WRITE_CHANNEL_OUTPUT_RES_LEN {() => {6}}
pub const WRITE_CHANNEL_OUTPUT_RES_LEN: u8 = WRITE_CHANNEL_OUTPUT_RES_LEN!();

// command example - both on:
// ":w20=1,1.\r\n"
macro_rules! WRITE_CHANNEL_OUTPUT_BOTH_ON {() => {concat!(
    COMMAND_BEGIN!(),
    COMMAND_WRITE!(),
    WRITE_CHANNEL_OUTPUT_COMMAND!(),
    COMMAND_SEPARATOR!(),
    WRITE_CHANNEL_OUTPUT_ARG_CH_ON!(),
    COMMAND_ARG_SEPARATOR!(),
    WRITE_CHANNEL_OUTPUT_ARG_CH_ON!(),
    COMMAND_END!(),
)}}
pub const WRITE_CHANNEL_OUTPUT_BOTH_ON: &str = WRITE_CHANNEL_OUTPUT_BOTH_ON!();

// command example - both off:
// ":w20=0,0.\r\n"
macro_rules! WRITE_CHANNEL_OUTPUT_BOTH_OFF {() => {concat!(
    COMMAND_BEGIN!(),
    COMMAND_WRITE!(),
    WRITE_CHANNEL_OUTPUT_COMMAND!(),
    COMMAND_SEPARATOR!(),
    WRITE_CHANNEL_OUTPUT_ARG_CH_OFF!(),
    COMMAND_ARG_SEPARATOR!(),
    WRITE_CHANNEL_OUTPUT_ARG_CH_OFF!(),
    COMMAND_END!(),
)}}
pub const WRITE_CHANNEL_OUTPUT_BOTH_OFF: &str = WRITE_CHANNEL_OUTPUT_BOTH_OFF!();

// command example - ch1 on, ch2 off:
// ":w20=1,0.\r\n"
macro_rules! WRITE_CHANNEL_OUTPUT_CH1_ON_CH2_OFF {() => {concat!(
    COMMAND_BEGIN!(),
    COMMAND_WRITE!(),
    WRITE_CHANNEL_OUTPUT_COMMAND!(),
    COMMAND_SEPARATOR!(),
    WRITE_CHANNEL_OUTPUT_ARG_CH_ON!(),
    COMMAND_ARG_SEPARATOR!(),
    WRITE_CHANNEL_OUTPUT_ARG_CH_OFF!(),
    COMMAND_END!(),
)}}
pub const WRITE_CHANNEL_OUTPUT_CH1_ON_CH2_OFF: &str = WRITE_CHANNEL_OUTPUT_CH1_ON_CH2_OFF!();

// command example - ch1 off, ch2 on:
// ":w20=0,1.\r\n"
macro_rules! WRITE_CHANNEL_OUTPUT_CH1_OFF_CH2_ON {() => {concat!(
    COMMAND_BEGIN!(),
    COMMAND_WRITE!(),
    WRITE_CHANNEL_OUTPUT_COMMAND!(),
    COMMAND_SEPARATOR!(),
    WRITE_CHANNEL_OUTPUT_ARG_CH_OFF!(),
    COMMAND_ARG_SEPARATOR!(),
    WRITE_CHANNEL_OUTPUT_ARG_CH_ON!(),
    COMMAND_END!(),
)}}
pub const WRITE_CHANNEL_OUTPUT_CH1_OFF_CH2_ON: &str = WRITE_CHANNEL_OUTPUT_CH1_OFF_CH2_ON!();
// -----


// -----
// Set waveform preset for each channel.
// Ex:
//   ch1 preset0 (sine wave) = ":w21=00.\r\n"
//   ch2 preset1 (square wave) = ":w22=01.\r\n"
//   ch1 preset101 (arbitrary wave preset1) = ":w21=101.\r\n"
//   ch2 preset102 (arbitrary wave preset2) = ":w22=102.\r\n"
macro_rules! WRITE_WAVEFORM_PRESET_COMMAND_PREFIX {() => {"2"}}
pub const WRITE_WAVEFORM_PRESET_COMMAND_PREFIX: &'static str = WRITE_WAVEFORM_PRESET_COMMAND_PREFIX!();

macro_rules! WRITE_WAVEFORM_PRESET_COMMAND_CH1 {() => {concat!(
    WRITE_WAVEFORM_PRESET_COMMAND_PREFIX!(),
    "1",
)}}
pub const WRITE_WAVEFORM_PRESET_COMMAND_CH1: &'static str = WRITE_WAVEFORM_PRESET_COMMAND_CH1!();

macro_rules! WRITE_WAVEFORM_PRESET_COMMAND_CH2 {() => {concat!(
    WRITE_WAVEFORM_PRESET_COMMAND_PREFIX!(),
    "2",
)}}
pub const WRITE_WAVEFORM_PRESET_COMMAND_CH2: &'static str = WRITE_WAVEFORM_PRESET_COMMAND_CH2!();

macro_rules! WRITE_WAVEFORM_PRESET_RES_LEN {() => {6}}
pub const WRITE_WAVEFORM_PRESET_RES_LEN: u8 = WRITE_WAVEFORM_PRESET_RES_LEN!();

macro_rules! WAVEFORM_PRESET_NAMES {() => {"0:  sine || sin
1:  square || sq
2:  pulse || pul
3:  triangle || tri
4:  partialsine || partial-sine || parsine || par-sine || parsin || par-sin || psine || p-sine || psin || p-sin
5:  cmos || cm
6:  dc
7:  halfwave || half-wave || hw || h-w
8:  fullwave || full-wave || fw || f-w
9:  pos-ladder || posladder || pos-lad || poslad || positive-ladder || positiveladder || pl
10: neg-ladder || negladder || neg-lad || neglad || negative-ladder || negativeladder || nl
11: noise || nois || noi || no || n
12: exp-rise || exprise || e-r || er || e-rise || erise || e-ris || eris
13: exp-decay || expdecay || e-d || ed || e-decay || edecay || e-dec || edec
14: multi-tone || multitone || m-t || mt || m-tone || mtone
15: sinc || sc
16: lorenz || loren || lor || lz"}}
pub const WAVEFORM_PRESET_NAMES: &'static str = WAVEFORM_PRESET_NAMES!();
// -----


// -----
// Set waveform frequency for each channel.
// Ex:
//   ch1 = ":w23=0.01,0.\r\n"
//   ch2 = ":w24=0.01,0.\r\n"
macro_rules! WRITE_FREQUENCY_COMMAND_PREFIX {() => {"2"}}
pub const WRITE_FREQUENCY_COMMAND_PREFIX: &'static str = WRITE_FREQUENCY_COMMAND_PREFIX!();

macro_rules! WRITE_FREQUENCY_COMMAND_CH1 {() => {concat!(
    WRITE_FREQUENCY_COMMAND_PREFIX!(),
    "3",
)}}
pub const WRITE_FREQUENCY_COMMAND_CH1: &'static str = WRITE_FREQUENCY_COMMAND_CH1!();

macro_rules! WRITE_FREQUENCY_COMMAND_CH2 {() => {concat!(
    WRITE_FREQUENCY_COMMAND_PREFIX!(),
    "4",
)}}
pub const WRITE_FREQUENCY_COMMAND_CH2: &'static str = WRITE_FREQUENCY_COMMAND_CH2!();

macro_rules! WRITE_FREQUENCY_COMMAND_UNIT_MICROHERTZ {() => {"4"}}
pub const WRITE_FREQUENCY_COMMAND_UNIT_MICROHERTZ: &'static str = WRITE_FREQUENCY_COMMAND_UNIT_MICROHERTZ!();

macro_rules! WRITE_FREQUENCY_COMMAND_UNIT_MILLIHERTZ {() => {"3"}}
pub const WRITE_FREQUENCY_COMMAND_UNIT_MILLIHERTZ: &'static str = WRITE_FREQUENCY_COMMAND_UNIT_MILLIHERTZ!();

macro_rules! WRITE_FREQUENCY_COMMAND_UNIT_HERTZ {() => {"0"}}
pub const WRITE_FREQUENCY_COMMAND_UNIT_HERTZ: &'static str = WRITE_FREQUENCY_COMMAND_UNIT_HERTZ!();

macro_rules! WRITE_FREQUENCY_COMMAND_UNIT_KILOHERTZ {() => {"1"}}
pub const WRITE_FREQUENCY_COMMAND_UNIT_KILOHERTZ: &'static str = WRITE_FREQUENCY_COMMAND_UNIT_KILOHERTZ!();

macro_rules! WRITE_FREQUENCY_COMMAND_UNIT_MEGAHERTZ {() => {"2"}}
pub const WRITE_FREQUENCY_COMMAND_UNIT_MEGAHERTZ: &'static str = WRITE_FREQUENCY_COMMAND_UNIT_MEGAHERTZ!();

macro_rules! WRITE_FREQUENCY_RES_LEN {() => {6}}
pub const WRITE_FREQUENCY_RES_LEN: u8 = WRITE_FREQUENCY_RES_LEN!();
// -----


// -----
// Set the signal amplitude.
// Ex:
//   ch1 (0.01v) = ":w25=1.\r\n"
//   ch2 (0.01v) = ":w26=1.\r\n"
macro_rules! WRITE_AMPLITUDE_COMMAND_PREFIX {() => {"2"}}
pub const WRITE_AMPLITUDE_COMMAND_PREFIX: &'static str = WRITE_AMPLITUDE_COMMAND_PREFIX!();

macro_rules! WRITE_AMPLITUDE_COMMAND_CH1 {() => {concat!(
    WRITE_AMPLITUDE_COMMAND_PREFIX!(),
    "5",
)}}
pub const WRITE_AMPLITUDE_COMMAND_CH1: &'static str = WRITE_AMPLITUDE_COMMAND_CH1!();

macro_rules! WRITE_AMPLITUDE_COMMAND_CH2 {() => {concat!(
    WRITE_AMPLITUDE_COMMAND_PREFIX!(),
    "6",
)}}
pub const WRITE_AMPLITUDE_COMMAND_CH2: &'static str = WRITE_AMPLITUDE_COMMAND_CH2!();

macro_rules! WRITE_AMPLITUDE_RES_LEN {() => {6}}
pub const WRITE_AMPLITUDE_RES_LEN: u8 = WRITE_AMPLITUDE_RES_LEN!();
// -----


// -----
// Set the duty cycle.
// Ex:
//   ch1 (40.1%) = ":w29=401.\r\n"
//   ch2 (40.1%) = ":w30=401.\r\n"
macro_rules! WRITE_DUTY_CYCLE_COMMAND_PREFIX_CH1 {() => {"2"}}
pub const WRITE_DUTY_CYCLE_COMMAND_PREFIX_CH1: &'static str = WRITE_DUTY_CYCLE_COMMAND_PREFIX_CH1!();

macro_rules! WRITE_DUTY_CYCLE_COMMAND_PREFIX_CH2 {() => {"3"}}
pub const WRITE_DUTY_CYCLE_COMMAND_PREFIX_CH2: &'static str = WRITE_DUTY_CYCLE_COMMAND_PREFIX_CH2!();

macro_rules! WRITE_DUTY_CYCLE_COMMAND_CH1 {() => {concat!(
    WRITE_DUTY_CYCLE_COMMAND_PREFIX_CH1!(),
    "9",
)}}
pub const WRITE_DUTY_CYCLE_COMMAND_CH1: &'static str = WRITE_DUTY_CYCLE_COMMAND_CH1!();

macro_rules! WRITE_DUTY_CYCLE_COMMAND_CH2 {() => {concat!(
    WRITE_DUTY_CYCLE_COMMAND_PREFIX_CH2!(),
    "0",
)}}
pub const WRITE_DUTY_CYCLE_COMMAND_CH2: &'static str = WRITE_DUTY_CYCLE_COMMAND_CH2!();

macro_rules! WRITE_DUTY_CYCLE_RES_LEN {() => {6}}
pub const WRITE_DUTY_CYCLE_RES_LEN: u8 = WRITE_DUTY_CYCLE_RES_LEN!();
// -----


// -----
// Set the offset in volts.
// Ex:
//   ch1 (40.1%) = ":w29=401.\r\n"
//   ch2 (40.1%) = ":w30=401.\r\n"
macro_rules! WRITE_VOLTAGE_OFFSET_COMMAND_PREFIX {() => {"2"}}
pub const WRITE_VOLTAGE_OFFSET_COMMAND_PREFIX: &'static str = WRITE_VOLTAGE_OFFSET_COMMAND_PREFIX!();

macro_rules! WRITE_VOLTAGE_OFFSET_COMMAND_CH1 {() => {concat!(
    WRITE_VOLTAGE_OFFSET_COMMAND_PREFIX!(),
    "7",
)}}
pub const WRITE_VOLTAGE_OFFSET_COMMAND_CH1: &'static str = WRITE_VOLTAGE_OFFSET_COMMAND_CH1!();

macro_rules! WRITE_VOLTAGE_OFFSET_COMMAND_CH2 {() => {concat!(
    WRITE_VOLTAGE_OFFSET_COMMAND_PREFIX!(),
    "8",
)}}
pub const WRITE_VOLTAGE_OFFSET_COMMAND_CH2: &'static str = WRITE_VOLTAGE_OFFSET_COMMAND_CH2!();

macro_rules! WRITE_VOLTAGE_OFFSET_RES_LEN {() => {6}}
pub const WRITE_VOLTAGE_OFFSET_RES_LEN: u8 = WRITE_VOLTAGE_OFFSET_RES_LEN!();
// -----
