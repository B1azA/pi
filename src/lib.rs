mod instructions;
use instructions::INSTRUCTIONS;
mod allocation;

pub fn run(bytecode: &mut Vec<u8>, ip: usize) -> Result<(), String> {
    let mut ip = ip;
    let mut stack: Vec<*mut u8> = vec![];
    while ip < bytecode.len() {
        let fn_index = bytecode[ip] as usize;
        if fn_index < INSTRUCTIONS.len() {
            ip += 1;
            let halt = INSTRUCTIONS[fn_index](bytecode, &mut ip, &mut stack);
            if halt {
                break;
            }
        } else {
            return Err(format!(
                "Tried to access unknown instruction.\n    fn_index: {fn_index}\n    ip: {ip}"));
        }
    }
    Ok(())
}