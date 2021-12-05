use std::fs;


mod machinecode;

#[derive(Clone,Debug)]
pub enum Register{
    A,B,C,D,E,F,IP
}

#[derive(Debug)]
pub enum Instruction{
    //used internally for errors
    DUMMY,
    MOVD(Register,u32),
    MOVR(Register, Register),
    JEQ(Register),

    //some instructions have two variants for diff. params internally
    //on an asm/external level, it is one instruction.
    // D for direct, R for register

    ADDD(Register,u32),
    ADDR(Register,Register),
    SUBD(Register,u32),
    SUBR(Register, Register),
    ANDD(Register,u32),
    ANDR(Register,Register),
    ORD(Register,u32),
    ORR(Register,Register),
    NOT(Register),
    LOD(Register,u32),
    SET(Register,u32)
}

impl Instruction{
    fn from_bytes(bytes: &Vec<u8>,index:usize) -> (Instruction,u32){
        let opcode = machinecode::get_opcode(&bytes[index]);
        let arg1 = machinecode::get_register(&bytes[index]);
        match &opcode[..]{
            "0000" => {
                let val = u32::from_le_bytes([bytes[index+1],bytes[index+2],bytes[index+3],bytes[index+4]]);
                (Instruction::MOVD(arg1,val),5)
            },
            "0001" => {
                let arg2 = machinecode::get_register(&bytes[index+1]);
                (Instruction::MOVR(arg1,arg2),2)
            },
            "0010" => {
                (Instruction::JEQ(arg1),1)
            },
            "0011" => {
                let val = u32::from_le_bytes([bytes[index+1],bytes[index+2],bytes[index+3],bytes[index+4]]);
                (Instruction::ADDD(arg1,val),5)
            },
            "0100" => {
                 let arg2 = machinecode::get_register(&bytes[index+1]);
                (Instruction::ADDR(arg1,arg2),2)
            },
            "0101" => {
                let val = u32::from_le_bytes([bytes[index+1],bytes[index+2],bytes[index+3],bytes[index+4]]);
                (Instruction::SUBD(arg1,val),5)
            },
            "0110" => {
                 let arg2 = machinecode::get_register(&bytes[index+1]);
                (Instruction::SUBR(arg1,arg2),2)
            },
            "0111" => {
                let val = u32::from_le_bytes([bytes[index+1],bytes[index+2],bytes[index+3],bytes[index+4]]);
                (Instruction::ANDD(arg1,val),5)
            },
            "1000" => {
                 let arg2 = machinecode::get_register(&bytes[index+1]);
                (Instruction::ANDR(arg1,arg2),2)
            },
            "1001" => {
                let val = u32::from_le_bytes([bytes[index+1],bytes[index+2],bytes[index+3],bytes[index+4]]);
                (Instruction::ORD(arg1,val),5)
            },
            "1010" => {
                 let arg2 = machinecode::get_register(&bytes[index+1]);
                (Instruction::ORR(arg1,arg2),2)
            },
            "1101" => {
                (Instruction::NOT(arg1),1)
            },
            "1110" => {
                let val = u32::from_le_bytes([bytes[index+1],bytes[index+2],bytes[index+3],bytes[index+4]]);
                (Instruction::LOD(arg1,val),5)
            },
            "1111" => {
                let val = u32::from_le_bytes([bytes[index+1],bytes[index+2],bytes[index+3],bytes[index+4]]);
                (Instruction::SET(arg1,val),5)
            },
            _ => (Instruction::DUMMY,0)
        }
    }
}



#[derive(Debug)]
struct Machine{
    //= Registers

    //general purposes
    reg_a:u32,
    reg_b:u32,
    reg_c:u32,
    reg_d:u32,
    reg_e:u32,
    //instruction/return flags
    reg_f:u32,
    //instruction pointer
    reg_ip:u32,

    //= Not Registers
    //TODO: Expand to full 32 bit space?
    //perhapos 64MB is enough actual memory... IO can then be safely mapped higher.
    memory: Vec<u8>
}

impl Machine{

    fn get_register(&self, reg:Register) -> u32{
        match reg{
            Register::A => return self.reg_a,
            Register::B => return self.reg_b,
            Register::C => return self.reg_c,
            Register::D => return self.reg_d,
            Register::E => return self.reg_e,
            Register::F => return self.reg_f,
            Register::IP => return self.reg_ip,
        }
    }
    fn set_register(&mut self, reg:Register, val: u32){
        match reg{
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
    fn exec_instruction(&mut self,instruction:Instruction) -> bool{
        let mut jumped = false;
        match instruction{
            Instruction::MOVD(reg, val) => {
                self.set_register(reg, val);
            },
            Instruction::MOVR(target, source) => {
                 self.set_register(target, self.get_register(source));
            },
            Instruction::JEQ(register) => {
                if self.get_register(register) == 0{
                    self.reg_ip = self.reg_f;
                    jumped = true;
                }
            },
            Instruction::ADDD(reg, val) => {
                //thank the borrow checker and my laziness for the clone
                let new = self.get_register(reg.clone())+val;
                self.set_register(reg,new);
            },
            Instruction::ADDR(reg, val) => {
                let new = self.get_register(reg.clone())+self.get_register(val);
                self.set_register(reg,new);
            },
            Instruction::SUBD(reg, val) => {
                let new = self.get_register(reg.clone())-val;
                self.set_register(reg,new);
            },
            Instruction::SUBR(reg, val) => {
                let new = self.get_register(reg.clone())-self.get_register(val);
                self.set_register(reg,new);
            },
            Instruction::ANDD(reg, val) => {
                 let new = self.get_register(reg.clone())&val;
                self.set_register(reg,new);
            },
            Instruction::ANDR(reg, val) => {
                let new = self.get_register(reg.clone())&self.get_register(val);
                self.set_register(reg,new);
            },
            Instruction::ORD(reg, val) => {
                 let new = self.get_register(reg.clone())|val;
                self.set_register(reg,new);
            },
            Instruction::ORR(reg, val) => {
                  let new = self.get_register(reg.clone())|self.get_register(val);
                self.set_register(reg,new);
            },
            Instruction::NOT(reg) => {
                 let new =!self.get_register(reg.clone());
                 self.set_register(reg, new);
            },
           
            Instruction::LOD(reg, addr) => {
                let val = u32::from_le_bytes([self.memory[(addr) as usize],self.memory[(addr+1) as usize],self.memory[(addr+2) as usize],self.memory[(addr+3) as usize]]);
                self.set_register(reg, val)
            },
            Instruction::SET(reg, val) => {
                let bytes = self.get_register(reg).to_le_bytes();
                self.memory[val as usize] = bytes[0];
                self.memory[(val+1) as usize] = bytes[1];
                self.memory[(val+2) as usize] = bytes[2];
                self.memory[(val+3) as usize] = bytes[3];

            },
            _ => {}
        }
        return jumped;
    }
}

fn main() {

    let binfilepath = std::env::args().nth(1).expect("No Binfile path provided!");
    let mut binfilecontents = fs::read(binfilepath).unwrap();
    let proglen = binfilecontents.len();
    let mut mem = vec![0 as u8; 65535*4];
    mem.append(&mut binfilecontents);



    let mut machine = Machine{
        reg_a: 0,
        reg_b: 0,
        reg_c: 0,
        reg_d: 0,
        reg_e: 0,
        reg_f: 0,
        reg_ip: (65535*4),
        memory: mem,
    };

    loop{
         if machine.reg_ip >= (proglen as u32)+65535*4{
            println!("End of code block reached. Halting execution to avoid executing mem-mapped IO or other non-code blocks.");
            break;
        }
        let ins = Instruction::from_bytes(&machine.memory,machine.reg_ip as usize);
        println!("Executing instruction {:?}",ins.0);
        let jmped = machine.exec_instruction(ins.0);
        println!("A: {} | B: {} | C: {} | D: {} | E: {} | F: {} | Byte: {:#b} | IP: {}",machine.reg_a,machine.reg_b,machine.reg_c,machine.reg_d,machine.reg_e,machine.reg_f,&machine.memory[machine.reg_ip as usize],machine.reg_ip);
        if !jmped{
            machine.reg_ip += ins.1;
        }
       
    }

}
