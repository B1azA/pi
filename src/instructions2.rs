use crate::VM;
use crate::allocation::*;

pub const INSTRUCTIONS: [fn(&mut VM) -> bool; 20] = [
    halt, pu1, pu2, pu4, pu8, pux, set1, set2, set4, set8, setx, off, pop1, pop2, pop4, pop8, popx, 
//  0     1    2    3    4    5    6     7     8     9     10    11   12    13    14    15    16    
    pop, cp, sett
//  17   18  19
// Types: u8, i8, u16, i16, u32, i32, u64, i64, f32, f64 
];

/// stops the machine
/// 
/// Layout: 0 - 1 byte
fn halt(vm: &mut VM) -> bool {
    true
}

/// pushes 1 byte
/// 
/// Layout: 1, data - 2 bytes
fn pu1(vm: &mut VM) -> bool {
    let ptr = allocate::<u8>();
    vm.stack.push(ptr);
    let val = vm.bytecode[vm.ip];
    set_value(ptr, val);
    vm.ip += 1;
    false
}

/// pushes 2 bytes
/// 
/// Layout: 2, data, data - 3 bytes
fn pu2(vm: &mut VM) -> bool {
    let ptr = allocate::<u16>();
    vm.stack.push(ptr);
    let bytes = vm.bytecode[vm.ip..vm.ip + 2].try_into().unwrap();
    let val = u16::from_be_bytes(bytes);
    set_value(ptr, val);
    vm.ip += 2;
    false
}

/// pushes 4 bytes
/// 
/// Layout: 3, data, data, data, data - 5 bytes
fn pu4(vm: &mut VM) -> bool {
    let ptr = allocate::<u32>();
    vm.stack.push(ptr);
    let bytes = vm.bytecode[vm.ip..vm.ip + 4].try_into().unwrap();
    let val = u32::from_be_bytes(bytes);
    set_value(ptr, val);
    vm.ip += 4;
    false
}

/// pushes 8 bytes
/// 
/// Layout: 4, data, data, data, data, data, data, data, data - 9 bytes
fn pu8(vm: &mut VM) -> bool{
    let ptr = allocate::<u64>();
    vm.stack.push(ptr);
    let bytes = vm.bytecode[vm.ip..vm.ip + 8].try_into().unwrap();
    let val = u64::from_be_bytes(bytes);
    set_value(ptr, val);
    vm.ip += 8;
    false
}

/// pushes x bytes and sets them to 0
/// 
/// Layout: 5, x, x, x, x, x, x, x, x  - 9 bytes
fn pux(vm: &mut VM) -> bool {
    let bytes = vm.bytecode[vm.ip..vm.ip + 8].try_into().unwrap();
    let x = u64::from_be_bytes(bytes);
    let ptr = allocate_size(x as usize);
    vm.stack.push(ptr);
    vm.ip += 8;
    
    false
}

/// sets 1 byte to the highest index element on the stack
/// 
/// Layout: 6, data - 2 bytes
fn set1(vm: &mut VM) -> bool {
    let ptr = vm.stack[vm.stack.len() - 1];
    let val = vm.bytecode[vm.ip];
    set_value(ptr, val);
    vm.ip += 1;
    false
}

/// sets 2 bytes to the highest index element on the stack
/// 
/// Layout: 7, data, data - 3 bytes
fn set2(vm: &mut VM) -> bool {
    let ptr = vm.stack[vm.stack.len() - 1];
    let bytes = vm.bytecode[vm.ip..vm.ip + 2].try_into().unwrap();
    let val = u16::from_be_bytes(bytes);
    set_value(ptr, val);
    vm.ip += 2;
    false
}

/// sets 4 bytes to the highest index element on the stack
/// 
/// Layout: 8, data, data, data, data - 5 bytes
fn set4(vm: &mut VM) -> bool {
    let ptr = vm.stack[vm.stack.len() - 1];
    let bytes = vm.bytecode[vm.ip..vm.ip + 4].try_into().unwrap();
    let val = u32::from_be_bytes(bytes);
    set_value(ptr, val);
    vm.ip += 4;
    false
}

/// sets 8 bytes to the highest index element on the stack
/// 
/// Layout: 9, data, data, data, data, data, data, data, data - 9 bytes
fn set8(vm: &mut VM) -> bool {
    let ptr = vm.stack[vm.stack.len() - 1];
    let bytes = vm.bytecode[vm.ip..vm.ip + 8].try_into().unwrap();
    let val = u64::from_be_bytes(bytes);
    set_value(ptr, val);
    let num = get_value::<u64>(ptr);
    vm.ip += 8;
    false
}

/// sets xth byte of the highest index element on the stack
/// 
/// Layout: 10, x, x, x, x, x, x, x, x, data - 10 bytes
fn setx(vm: &mut VM) -> bool {
    let bytes = vm.bytecode[vm.ip..vm.ip + 8].try_into().unwrap();
    let x = u64::from_be_bytes(bytes);
    vm.ip += 8;
    let val = vm.bytecode[vm.ip];
    vm.ip += 1; 
    let ptr = vm.stack[vm.stack.len() - 1];
    set_value_size(ptr, x as isize, val);
    false
}

/// offsets the highest index element's on the stack pointer
/// 
/// Layout: 11, data, data, data, data, data, data, data, data - 9 bytes 
/// (in, i64)
fn off(vm: &mut VM) -> bool {
    let bytes = vm.bytecode[vm.ip..vm.ip + 8].try_into().unwrap();
    let off = i64::from_be_bytes(bytes);
    let stack_len = vm.stack.len();
    offset(&mut vm.stack[stack_len - 1], off as isize);
    vm.ip += 8;
    false
}

/// pops the highest index element on the stack, assuming it is 1 byte
/// 
/// Layout: 12 - 1 byte 
fn pop1(vm: &mut VM) -> bool {
    let ptr = vm.stack.pop().unwrap();
    deallocate::<u8>(ptr);
    false
}

/// pops the highest index element on the stack, assuming it is 2 bytes
/// 
/// Layout: 13 - 1 byte 
fn pop2(vm: &mut VM) -> bool {
    let ptr = vm.stack.pop().unwrap();
    deallocate::<u16>(ptr);
    false
}

/// pops the highest index element on the stack, assuming it is 4 bytes
/// 
/// Layout: 14 - 1 byte 
fn pop4(vm: &mut VM) -> bool {
    let ptr = vm.stack.pop().unwrap();
    deallocate::<u32>(ptr);
    false
}

/// pops the highest index element on the stack, assuming it is 8 bytes
/// 
/// Layout: 15 - 1 byte 
fn pop8(vm: &mut VM) -> bool {
    let ptr = vm.stack.pop().unwrap();
    deallocate::<u64>(ptr);
    false
}

/// pops the highest index element on the stack, assuming it is x bytes
/// 
/// Layout: 16, x, x, x, x, x, x, x, x - 9 bytes
fn popx(vm: &mut VM) -> bool {
    let ptr = vm.stack.pop().unwrap();
    let bytes = vm.bytecode[vm.ip..vm.ip + 8].try_into().unwrap();
    let x = u64::from_be_bytes(bytes);
    deallocate_size(ptr, x as usize);
    vm.ip += 8;
    false
}

/// pops the highest index element on the stack, but does not deallocate it
/// 
/// Layout: 17 - 1 byte
fn pop(vm: &mut VM) -> bool {
    vm.stack.pop();
    false
}

/// copies the highest index element on stack and pushes it to stack, if stack length is 0 does nothing
/// 
/// Layout: 18 - 1 byte
fn cp(vm: &mut VM) -> bool {
    let stack_len = vm.stack.len();
    if stack_len > 0 {
        vm.stack.push(vm.stack[vm.stack.len() - 1]);
    }
    false
}

/// sets the global type 
/// 
/// Layout: 19, type - 2 bytes
/// 
/// types: 0 = u8, 1 = i8, 2 = u16, 3 = i16, 4 = u32, 5 = i32, 6 = u64, 7 = i64, 8 = f32, 9 = f64
fn sett(vm: &mut VM) -> bool {
    let tp = vm.bytecode[vm.ip];
    vm.data_type = tp;
    vm.ip += 1;
    false
}

/// gets two values of global type from stack and adds them, pushes result to the stack
/// 
/// Layout: 20 - 1 byte
fn add(vm: &mut VM) -> bool {
    if vm.stack.len() > 1 {
        let ptr1 = vm.stack.pop().unwrap();
        let ptr2 = vm.stack.pop().unwrap();

        match vm.data_type {
            0 => {
                let val1: u8 = get_value(ptr1);
                let val2: u8 = get_value(ptr2);
                let result = val1 + val2;
                let ptr = allocate::<u8>();
                set_value(ptr, result);
                vm.stack.push(ptr);
            }

            _ => {}
        }
        false
    } else {
        true
    }
}