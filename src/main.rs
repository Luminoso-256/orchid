use raylib::prelude::*;
use std::fs;

mod machinecode;

#[derive(Clone, Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    IP,
}

#[derive(Debug)]
pub enum Instruction {
    //used internally for errors
    DUMMY,
    MOVD(Register, u32),
    MOVR(Register, Register),
    JEQ(Register, u32),
    JLT(Register, u32),

    //some instructions have two variants for diff. params internally
    //on an asm/external level, it is one instruction.
    // D for direct, R for register
    ADDD(Register, u32),
    ADDR(Register, Register),
    SUBD(Register, u32),
    SUBR(Register, Register),
    ANDD(Register, u32),
    ANDR(Register, Register),
    ORD(Register, u32),
    ORR(Register, Register),
    NOT(Register),
    LOD(Register, u32),
    SET(Register),
}

impl Instruction {
    fn from_bytes(bytes: &Vec<u8>, index: usize) -> (Instruction, u32) {
        let opcode = machinecode::get_opcode(&bytes[index]);
        let arg1 = machinecode::get_register(&bytes[index]);
        match &opcode[..] {
            "00000" => {
                let val = u32::from_le_bytes([
                    bytes[index + 1],
                    bytes[index + 2],
                    bytes[index + 3],
                    bytes[index + 4],
                ]);
                (Instruction::MOVD(arg1, val), 5)
            }
            "00001" => {
                let arg2 = machinecode::get_register(&bytes[index + 1]);
                (Instruction::MOVR(arg1, arg2), 2)
            }
            "00010" => {
                let val = u32::from_le_bytes([
                    bytes[index + 1],
                    bytes[index + 2],
                    bytes[index + 3],
                    bytes[index + 4],
                ]);
                (Instruction::JEQ(arg1, val), 5)
            }
            "00011" => {
                let val = u32::from_le_bytes([
                    bytes[index + 1],
                    bytes[index + 2],
                    bytes[index + 3],
                    bytes[index + 4],
                ]);
                (Instruction::ADDD(arg1, val), 5)
            }
            "00100" => {
                let arg2 = machinecode::get_register(&bytes[index + 1]);
                (Instruction::ADDR(arg1, arg2), 2)
            }
            "00101" => {
                let val = u32::from_le_bytes([
                    bytes[index + 1],
                    bytes[index + 2],
                    bytes[index + 3],
                    bytes[index + 4],
                ]);
                (Instruction::SUBD(arg1, val), 5)
            }
            "00110" => {
                let arg2 = machinecode::get_register(&bytes[index + 1]);
                (Instruction::SUBR(arg1, arg2), 2)
            }
            "00111" => {
                let val = u32::from_le_bytes([
                    bytes[index + 1],
                    bytes[index + 2],
                    bytes[index + 3],
                    bytes[index + 4],
                ]);
                (Instruction::ANDD(arg1, val), 5)
            }
            "01000" => {
                let arg2 = machinecode::get_register(&bytes[index + 1]);
                (Instruction::ANDR(arg1, arg2), 2)
            }
            "01001" => {
                let val = u32::from_le_bytes([
                    bytes[index + 1],
                    bytes[index + 2],
                    bytes[index + 3],
                    bytes[index + 4],
                ]);
                (Instruction::ORD(arg1, val), 5)
            }
            "01010" => {
                let arg2 = machinecode::get_register(&bytes[index + 1]);
                (Instruction::ORR(arg1, arg2), 2)
            }
            "01101" => (Instruction::NOT(arg1), 1),
            "01110" => {
                let val = u32::from_le_bytes([
                    bytes[index + 1],
                    bytes[index + 2],
                    bytes[index + 3],
                    bytes[index + 4],
                ]);
                (Instruction::LOD(arg1, val), 5)
            }
            "01111" => (Instruction::SET(arg1), 1),
            "10000" => {
                let val = u32::from_le_bytes([
                    bytes[index + 1],
                    bytes[index + 2],
                    bytes[index + 3],
                    bytes[index + 4],
                ]);
                (Instruction::JLT(arg1, val), 5)
            }
            _ => (Instruction::DUMMY, 0),
        }
    }
}

#[derive(Debug)]
struct Machine {
    //= Registers

    //general purposes
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    reg_d: u32,
    reg_e: u32,
    //instruction/return flags
    reg_f: u32,
    //instruction pointer
    reg_ip: u32,

    //= Not Registers
    //TODO: Expand to full 32 bit space?
    //perhapos 64MB is enough actual memory... IO can then be safely mapped higher.
    memory: Vec<u8>,
    //"Video memory"/the part of mem-mapped IO responsible for the display
    vid_memory: [u8; 0x2A00],
}

impl Machine {
    fn get_register(&self, reg: Register) -> u32 {
        match reg {
            Register::A => return self.reg_a,
            Register::B => return self.reg_b,
            Register::C => return self.reg_c,
            Register::D => return self.reg_d,
            Register::E => return self.reg_e,
            Register::F => return self.reg_f,
            Register::IP => return self.reg_ip,
        }
    }
    fn set_register(&mut self, reg: Register, val: u32) {
        match reg {
            Register::A => self.reg_a = val,
            Register::B => self.reg_b = val,
            Register::C => self.reg_c = val,
            Register::D => self.reg_d = val,
            Register::E => self.reg_e = val,
            Register::F => self.reg_f = val,
            Register::IP => self.reg_ip = val,
        }
    }

    //return is if jump took place
    fn exec_instruction(&mut self, instruction: Instruction) -> bool {
        let mut jumped = false;
        match instruction {
            Instruction::MOVD(reg, val) => {
                self.set_register(reg, val);
            }
            Instruction::MOVR(target, source) => {
                self.set_register(target, self.get_register(source));
            }
            Instruction::JEQ(register, val) => {
                if self.get_register(register) == self.get_register(Register::F) {
                    self.reg_ip = val;
                    jumped = true;
                }
            }
            Instruction::JLT(register, val) => {
                if self.get_register(register) < self.get_register(Register::F) {
                    self.reg_ip = val;
                    jumped = true;
                }
            }
            Instruction::ADDD(reg, val) => {
                //thank the borrow checker and my laziness for the clone
                let new = self.get_register(reg.clone()) + val;
                self.set_register(reg, new);
            }
            Instruction::ADDR(reg, val) => {
                let new = self.get_register(reg.clone()) + self.get_register(val);
                self.set_register(reg, new);
            }
            Instruction::SUBD(reg, val) => {
                let new = self.get_register(reg.clone()) - val;
                self.set_register(reg, new);
            }
            Instruction::SUBR(reg, val) => {
                let new = self.get_register(reg.clone()) - self.get_register(val);
                self.set_register(reg, new);
            }
            Instruction::ANDD(reg, val) => {
                let new = self.get_register(reg.clone()) & val;
                self.set_register(reg, new);
            }
            Instruction::ANDR(reg, val) => {
                let new = self.get_register(reg.clone()) & self.get_register(val);
                self.set_register(reg, new);
            }
            Instruction::ORD(reg, val) => {
                let new = self.get_register(reg.clone()) | val;
                self.set_register(reg, new);
            }
            Instruction::ORR(reg, val) => {
                let new = self.get_register(reg.clone()) | self.get_register(val);
                self.set_register(reg, new);
            }
            Instruction::NOT(reg) => {
                let new = !self.get_register(reg.clone());
                self.set_register(reg, new);
            }

            Instruction::LOD(reg, addr) => {
                let val: u32;
                //check for mem-mapped IO
                if addr >= 0xFFFF0000 {
                    //we're working with mem-mapped IO
                    let rebased_addr = addr - 0xFFFF0000;
                    val = u32::from_le_bytes([
                        self.vid_memory[(rebased_addr) as usize],
                        self.vid_memory[(rebased_addr + 1) as usize],
                        self.vid_memory[(rebased_addr + 2) as usize],
                        self.vid_memory[(rebased_addr + 3) as usize],
                    ]);
                } else {
                    val = u32::from_le_bytes([
                        self.memory[(addr) as usize],
                        self.memory[(addr + 1) as usize],
                        self.memory[(addr + 2) as usize],
                        self.memory[(addr + 3) as usize],
                    ]);
                }
                self.set_register(reg, val)
            }
            Instruction::SET(reg) => {
                let bytes = self.get_register(reg).to_le_bytes();
                let val = self.get_register(Register::F);
                //check for mem-mapped IO
                if val >= 0xFFFF0000 {
                    //we're working with mem-mapped IO
                    let rebased_addr = val - 0xFFFF0000;
                    self.vid_memory[(rebased_addr as usize)] = bytes[0];
                    self.vid_memory[(rebased_addr + 1) as usize] = bytes[1];
                    self.vid_memory[(rebased_addr + 2) as usize] = bytes[2];
                    self.vid_memory[(rebased_addr + 3) as usize] = bytes[3];
                } else {
                    self.memory[(val + 1) as usize] = bytes[1];
                    self.memory[val as usize] = bytes[0];
                    self.memory[(val + 2) as usize] = bytes[2];
                    self.memory[(val + 3) as usize] = bytes[3];
                }
            }
            _ => {}
        }
        return jumped;
    }
}

fn fourbit_color(str: &str) -> Color {
    match str {
        "0000" => Color::WHITE,
        "0001" => Color::DARKGREEN,
        "0010" => Color::GREEN,
        "0011" => Color::DARKBLUE,
        "0100" => Color::BLUE,
        "0101" => Color::SKYBLUE,
        "0110" => Color::RED,
        "0111" => Color::ORANGE,
        "1000" => Color::YELLOW,
        "1001" => Color::PINK,
        "1010" => Color::LIGHTGRAY,
        "1011" => Color::BLACK,
        "1100" => Color::MAGENTA,
        _ => Color::BLACK,
    }
}

fn colors_from_byte(byte: &u8) -> (Color, Color) {
    let mut fgs = machinecode::get_fronthalf(byte);
    let mut bgs = machinecode::get_backhalf(byte);
    (fourbit_color(&fgs), fourbit_color(&bgs))
}

fn main() {
    let binfilepath = std::env::args().nth(1).expect("No Binfile path provided!");
    let dbf = std::env::args().nth(2).unwrap_or("".to_string());
    let debug: bool;
    if dbf != "" {
        debug = true;
    } else {
        debug = false;
    }
    let mut binfilecontents = fs::read(binfilepath).unwrap();
    let proglen = binfilecontents.len();
    let mut mem = vec![0 as u8; 65535 * 4];
    mem.append(&mut binfilecontents);

    let mut machine = Machine {
        reg_a: 0,
        reg_b: 0,
        reg_c: 0,
        reg_d: 0,
        reg_e: 0,
        reg_f: 0,
        reg_ip: (65535 * 4),
        memory: mem,
        vid_memory: [0 as u8; 0x2A00],
    };

    let (mut rl, thread) = raylib::init()
        .size(640, 420)
        .title("OrchidEmu")
        .resizable()
        .build();

    let mut exechalt = false;

    loop {
        //== Core Sim Logic
        if !exechalt {
            if machine.reg_ip >= (proglen as u32) + 65535 * 4 {
                println!("End of code block reached. Halting execution to avoid executing mem-mapped IO or other non-code blocks.");
                exechalt = true;
                continue;
            }
            let ins = Instruction::from_bytes(&machine.memory, machine.reg_ip as usize);
            if debug {
                println!("Executing instruction {:?}", ins.0);
            }
            let jmped = machine.exec_instruction(ins.0);
            if debug {
                println!(
                    "A: {} | B: {} | C: {} | D: {} | E: {} | F: {} | Byte: {:#b} | IP: {}",
                    machine.reg_a,
                    machine.reg_b,
                    machine.reg_c,
                    machine.reg_d,
                    machine.reg_e,
                    machine.reg_f,
                    &machine.memory[machine.reg_ip as usize],
                    machine.reg_ip
                );
            }
            if !jmped {
                machine.reg_ip += ins.1;
            }
        }

        //== make the colors light up
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        let mut x = 0;
        let mut y = 0;
        for i in (0..machine.vid_memory.len()).step_by(2) {
            if machine.vid_memory[i] != 0 as u8 {
                let (fg, bg) = colors_from_byte(&machine.vid_memory[i + 1]);
                if bg != Color::BLACK {
                    d.draw_rectangle(x, y, 5, 10, bg);
                }
                //println!("{} / {:#x}",machine.vid_memory[i] as char,machine.vid_memory[i]);
                d.draw_text(&*format!("{}", machine.vid_memory[i] as char), x, y, 8, fg);
            }
            x += 5;
            if x >= 640 {
                x = 0;
                y += 10;
            }
        }
        //d.draw_text(&*format!("{}",d.get_fps()), 5, 5, 20, Color::RED);
        if exechalt {
            //d.draw_text("[EXECUTION HALTED]", 5, 5, 20, Color::RED);
        }
    }
}
