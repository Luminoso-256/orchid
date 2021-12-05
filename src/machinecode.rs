use crate::Register;

/*
- Every instruction starts with a byte laid out as follows:
<- Most Significant | Least Significant ->
[5 Bits: Instruction Opcode] [3 Bits: Register Specifier]
a register specifier can be ensured as all instruction's first parameter is a register specifier.
the next byte is the second parameter for the instruction. If this is a register, the register is encoded in the lower 4 bits, and that is the end of the instruction.
if it is an immediate value, then the second byte combined with the third byte form the parameter.
*/

pub fn get_opcode(byte: &u8) -> String {
    let mut s: String = "".to_string();
    //get the bits
    let mut out: Vec<bool> = vec![];
    for i in 3..8 {
        out.insert(0, byte & (1 << i) != 0);
    }
    for byte in out {
        if byte {
            s += "1"
        } else {
            s += "0"
        }
    }
    return s;
}
pub fn get_register(byte: &u8) -> Register {
    let mut s: String = "".to_string();
    //get the bits
    let mut out: Vec<bool> = vec![];
    for i in 0..3 {
        out.insert(0, byte & (1 << i) != 0);
    }
    for byte in out {
        if byte {
            s += "1"
        } else {
            s += "0"
        }
    }
    let clean_match_statement: &str = &s[..];
    match clean_match_statement {
        "000" => Register::A,
        "001" => Register::B,
        "010" => Register::C,
        "011" => Register::D,
        "100" => Register::E,
        "101" => Register::F,
        "110" => Register::IP,
        _ => Register::A,
    }
}

//generic verisons
pub fn get_fronthalf(byte: &u8) -> String {
    let mut s: String = "".to_string();
    //get the bits
    let mut out: Vec<bool> = vec![];
    for i in 4..8 {
        out.insert(0, byte & (1 << i) != 0);
    }
    for byte in out {
        if byte {
            s += "1"
        } else {
            s += "0"
        }
    }
    return s;
}
pub fn get_backhalf(byte: &u8) -> String {
    let mut s: String = "".to_string();
    //get the bits
    let mut out: Vec<bool> = vec![];
    for i in 0..4 {
        out.insert(0, byte & (1 << i) != 0);
    }
    for byte in out {
        if byte {
            s += "1"
        } else {
            s += "0"
        }
    }
    return s;
}
