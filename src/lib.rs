use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    // TODO: Decode hex string into Vec<u8>, return error string on failure
    hex::decode(hex_str).map_err(|e| format!("Hex decoding error: {}", e))
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    // TODO: Reverse the byte order of input slice and return as Vec<u8>
    bytes.iter().rev().copied().collect()
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    // TODO: Implement conversion of bytes slice to hex string
    encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    // TODO: Implement conversion of hex string to bytes vector
    decode(hex)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    // TODO: Implement little-endian byte swap for u32
    num.to_le_bytes()
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    // TODO: Parse input string to u64, return error string if invalid
    input
        .parse::<u64>()
        .map_err(|_| format!("Invalid satoshi amount"))
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}
pub fn classify_script(script: &[u8]) -> ScriptType {
    // TODO: Match script pattern and return corresponding ScriptType
    
    if script.starts_with(&[0x76, 0xa9, 0x14]) // OP_DUP, OP_HASH160, PUSH(20)
        && (script.len() == 3 || (script.len() == 25 && script[23] == 0x88 && script[24] == 0xac))
    {
        ScriptType::P2PKH
    }
    else if script.starts_with(&[0x00, 0x14]) // OP_0, PUSH(20)
        && (script.len() == 3 || script.len() == 22)
    {
        ScriptType::P2WPKH
    } else {
        ScriptType::Unknown
    }
}

#[derive(Debug, PartialEq)]
pub struct Outpoint(pub String, pub u32);

// TODO: complete Outpoint tuple struct
// #[derive(Debug, PartialEq)]
// pub struct Outpoint(
//     pub Vec<u8>,
//     pub u32
// );

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    match classify_script(script) {
        ScriptType::P2PKH => {
            if script.len() >= 25 {
                &script[3..23] // 20 bytes after the push byte
            } else {
                &[]
            }
        }
        ScriptType::P2WPKH => {
            if script.len() >= 22 {
                &script[2..22] // starts at index 2
            } else {
                &[]
            }
        }
        _ => &[],
    }
}



pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        // TODO: Return the wallet's confirmed balance
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    // TODO: Subtract fee from mutable balance reference
    *balance = balance.saturating_sub(fee);
}

pub fn move_txid(txid: String) -> String {
    // TODO: Return formatted string including the txid for display or logging
    format!("txid: {}", txid)
}

// TODO: Add necessary derive traits
#[derive(Debug, PartialEq)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        // TODO: Implement mapping from byte to Opcode variant
        match byte {
            0xac => Ok(Opcode::OpChecksig),
            0x76 => Ok(Opcode::OpDup),
            _ => Err(format!("Invalid opcode: 0x{:02x}", byte)),
        }
    }
}

// TODO: Add necessary derive traits
#[derive(Debug, Clone, PartialEq)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    // TODO: Implement UTXO consumption logic (if any)
    utxo
}
