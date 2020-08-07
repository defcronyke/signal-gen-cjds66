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
