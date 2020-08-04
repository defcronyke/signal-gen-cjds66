pub const SERIAL_TIMEOUT_MS: u64 = 3000;
pub const COMMAND_DELAY_MS: u64 = 50;

pub const GET_MACHINE_MODEL: &str = ":r00=0.\r\n";
pub const GET_MACHINE_MODEL_RES_LEN: u8 = 10;

pub const GET_MACHINE_NUMBER: &str = ":r01=0.\r\n";
pub const GET_MACHINE_NUMBER_RES_LEN: u8 = 18;
