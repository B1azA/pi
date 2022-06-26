mod instructions;
use instructions::INSTRUCTIONS;
mod allocation;

pub struct VM<'a> {
    bytecode: &'a Vec<u8>,
    ip: usize,
    stack: Vec<u64>,
    global: Vec<u64>,
    flag: bool,
    quit: bool,
    fn_pointers: Vec<usize>,
    functions: Vec<fn(&mut VM)>,
}

pub fn run(bytecode: &mut Vec<u8>, ip: usize) -> Result<(), String> {
    let mut vm = VM {
        bytecode,
        ip,
        stack: vec![],
        global: vec![0; 255],
        flag: false,
        quit: false,
        fn_pointers: vec![],
        functions: vec![],
    };

    while vm.ip < bytecode.len() {
        let fn_index = bytecode[vm.ip] as usize;
        if fn_index < INSTRUCTIONS.len() {
            vm.ip += 1;
            INSTRUCTIONS[fn_index](&mut vm);
            if vm.quit {
                break;
            }
        } else {
            return Err(format!(
                "Tried to access unknown instruction.\n    fn_index: {fn_index}\n    ip: {ip}"));
        }
    }
    Ok(())
}