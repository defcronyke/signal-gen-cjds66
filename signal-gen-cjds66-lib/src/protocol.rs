use bitflags;
use phf::phf_map;

use std::fmt;
use std::str;

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
// Set the voltage offset in volts.
// Ex:
//   ch1 (-1.23%) = ":w27=877.\r\n"
//   ch2 (-1.23%) = ":w28=877.\r\n"
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


// -----
// Set the phase in degrees.
// Ex:
//   180.7% = ":w31=1807.\r\n"
macro_rules! WRITE_PHASE_COMMAND {() => {"31"}}
pub const WRITE_PHASE_COMMAND: &'static str = WRITE_PHASE_COMMAND!();

macro_rules! WRITE_PHASE_RES_LEN {() => {6}}
pub const WRITE_PHASE_RES_LEN: u8 = WRITE_PHASE_RES_LEN!();
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
macro_rules! WRITE_TRACKING_COMMAND {() => {"54"}}
pub const WRITE_TRACKING_COMMAND: &'static str = WRITE_TRACKING_COMMAND!();

macro_rules! TRACKING_FEATURES {() => {"The bit position meanings are as follows:
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

Turn tracking off like this: -T n"}}
pub const TRACKING_FEATURES: &'static str = TRACKING_FEATURES!();

macro_rules! TRACKING_NONE      {() => {0b00000000u8}}
macro_rules! TRACKING_FREQUENCY {() => {0b00000001u8}}
macro_rules! TRACKING_WAVEFORM  {() => {0b00000010u8}}
macro_rules! TRACKING_AMPLITUDE {() => {0b00000100u8}}
macro_rules! TRACKING_DUTYCYCLE {() => {0b00001000u8}}
macro_rules! TRACKING_OFFSET    {() => {0b00010000u8}}

pub const TRACKING_NONE: u8         = TRACKING_NONE!();
pub const TRACKING_FREQUENCY: u8    = TRACKING_FREQUENCY!();
pub const TRACKING_WAVEFORM: u8     = TRACKING_WAVEFORM!();
pub const TRACKING_AMPLITUDE: u8    = TRACKING_AMPLITUDE!();
pub const TRACKING_DUTYCYCLE: u8    = TRACKING_DUTYCYCLE!();
pub const TRACKING_OFFSET: u8       = TRACKING_OFFSET!();

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
        write!(f, "{},{},{},{},{}", 
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
        format!("{}{}{}{}{}", 
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

        let res = format!("{},{},{},{},{}", 
            one, 
            two, 
            three, 
            four, 
            five,
        ).split(",")
            .filter(|&i| i != "none")
            .collect::<Vec<&str>>()
            .join(", ");
        
        if res == "" {
            return TRACKING_ARG_REVMAP[&0].to_string();
        }

        res
    }
}

macro_rules! WRITE_TRACKING_RES_LEN {() => {6}}
pub const WRITE_TRACKING_RES_LEN: u8 = WRITE_TRACKING_RES_LEN!();
// -----
