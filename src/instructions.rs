pub const INSTRUCTIONS: [fn(&mut Vec<u8>, &mut usize, &mut Vec<*mut u8>) -> bool; 19] = [
    halt, pu1, pu2, pu4, pu8, pux, set1, set2, set4, set8, setx, off, pop1, pop2, pop4, pop8, popx, pop, cp,
//  0     1    2    3    4    5    6     7     8     9     10    11   12    13    14    15    16    17   18
];
use crate::allocation::*;
// Types: u8, i8, u16, i16, u32, i32, u64, i64, f32, f64 


/// stops the machine
/// Layout: 0 - 1 byte
fn halt(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    true
}

/// pushes 1 byte
/// Layout: 1, data - 2 bytes
fn pu1(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let ptr = allocate::<u8>();
    stack.push(ptr);
    let val = bytecode[*ip];
    set_value(ptr, val);
    *ip += 1;
    false
}

/// pushes 2 bytes
/// Layout: 2, data, data - 3 bytes
fn pu2(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let ptr = allocate::<u16>();
    stack.push(ptr);
    let bytes = bytecode[*ip..*ip + 2].try_into().unwrap();
    let val = u16::from_be_bytes(bytes);
    set_value(ptr, val);
    *ip += 2;
    false
}

/// pushes 4 bytes
/// Layout: 3, data, data, data, data - 5 bytes
fn pu4(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let ptr = allocate::<u32>();
    stack.push(ptr);
    let bytes = bytecode[*ip..*ip + 4].try_into().unwrap();
    let val = u32::from_be_bytes(bytes);
    set_value(ptr, val);
    *ip += 4;
    false
}

/// pushes 8 bytes
/// Layout: 4, data, data, data, data, data, data, data, data - 9 bytes
fn pu8(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool{
    let ptr = allocate::<u64>();
    stack.push(ptr);
    let bytes = bytecode[*ip..*ip + 8].try_into().unwrap();
    let val = u64::from_be_bytes(bytes);
    set_value(ptr, val);
    *ip += 8;
    false
}

/// pushes x bytes and sets them to 0
/// Layout: 5, x, x, x, x, x, x, x, x  - 9 bytes
fn pux(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let bytes = bytecode[*ip..*ip + 8].try_into().unwrap();
    let x = u64::from_be_bytes(bytes);
    let ptr = allocate_size(x as usize);
    stack.push(ptr);
    *ip += 8;
    
    false
}

/// sets 1 byte to the highest index element on the stack
/// Layout: 6, data - 2 bytes
fn set1(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let ptr = stack[stack.len() - 1];
    let val = bytecode[*ip];
    set_value(ptr, val);
    *ip += 1;
    false
}

/// sets 2 bytes to the highest index element on the stack
/// Layout: 7, data, data - 3 bytes
fn set2(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let ptr = stack[stack.len() - 1];
    let bytes = bytecode[*ip..*ip + 2].try_into().unwrap();
    let val = u16::from_be_bytes(bytes);
    set_value(ptr, val);
    *ip += 2;
    false
}

/// sets 4 bytes to the highest index element on the stack
/// Layout: 8, data, data, data, data - 5 bytes
fn set4(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let ptr = stack[stack.len() - 1];
    let bytes = bytecode[*ip..*ip + 4].try_into().unwrap();
    let val = u32::from_be_bytes(bytes);
    set_value(ptr, val);
    *ip += 4;
    false
}

/// sets 8 bytes to the highest index element on the stack
/// Layout: 9, data, data, data, data, data, data, data, data - 9 bytes
fn set8(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let ptr = stack[stack.len() - 1];
    let bytes = bytecode[*ip..*ip + 8].try_into().unwrap();
    let val = u64::from_be_bytes(bytes);
    set_value(ptr, val);
    let num = get_value::<u64>(ptr);
    *ip += 8;
    false
}

/// sets xth byte of the highest index element on the stack
/// Layout: 10, x, x, x, x, x, x, x, x, data - 10 bytes
fn setx(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let bytes = bytecode[*ip..*ip + 8].try_into().unwrap();
    let x = u64::from_be_bytes(bytes);
    *ip += 8;
    let val = bytecode[*ip];
    *ip += 1; 
    let ptr = stack[stack.len() - 1];
    set_value_size(ptr, x as isize, val);
    false
}

/// offsets the highest index element's on the stack pointer
/// Layout: 11, data, data, data, data, data, data, data, data - 9 bytes 
/// (in, i64)
fn off(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let bytes = bytecode[*ip..*ip + 8].try_into().unwrap();
    let off = i64::from_be_bytes(bytes);
    let stack_len = stack.len();
    offset(&mut stack[stack_len - 1], off as isize);
    *ip += 8;
    false
}

/// pops the highest index element on the stack, assuming it is 1 byte
/// Layout: 12 - 1 byte 
fn pop1(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let ptr = stack.pop().unwrap();
    deallocate::<u8>(ptr);
    false
}

/// pops the highest index element on the stack, assuming it is 2 bytes
/// Layout: 13 - 1 byte 
fn pop2(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let ptr = stack.pop().unwrap();
    deallocate::<u16>(ptr);
    false
}

/// pops the highest index element on the stack, assuming it is 4 bytes
/// Layout: 14 - 1 byte 
fn pop4(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let ptr = stack.pop().unwrap();
    deallocate::<u32>(ptr);
    false
}

/// pops the highest index element on the stack, assuming it is 8 bytes
/// Layout: 15 - 1 byte 
fn pop8(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let ptr = stack.pop().unwrap();
    deallocate::<u64>(ptr);
    false
}

/// pops the highest index element on the stack, assuming it is x bytes
/// Layout: 16, x, x, x, x, x, x, x, x - 9 bytes
fn popx(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let ptr = stack.pop().unwrap();
    let bytes = bytecode[*ip..*ip + 8].try_into().unwrap();
    let x = u64::from_be_bytes(bytes);
    deallocate_size(ptr, x as usize);
    *ip += 8;
    false
}

/// pops the highest index element on the stack, but does not deallocate it
/// Layout: 17 - 1 byte
fn pop(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    stack.pop();
    false
}

/// copies the highest index element on stack and pushes it to stack, if stack length is 0 does nothing
/// Layout: 18 - 1 byte
fn cp(bytecode: &mut Vec<u8>, ip: &mut usize, stack: &mut Vec<*mut u8>) -> bool {
    let stack_len = stack.len();
    if stack_len > 0 {
        stack.push(stack[stack.len() - 1]);
    }
    false
}