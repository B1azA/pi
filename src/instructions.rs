use crate::VM;
use crate::allocation::*;

pub const INSTRUCTIONS: [fn(&mut VM); 45] = [
    halt, pu1, pu2, pu4, pu8, pux, set, get, setx, off, cp, load1, load2, load4, load8, and, or,
//  0     1    2    3    4    5    6    7    8     9     10    11   12    13     14    15    16
    xor, lshift, rshift, add, sub, mul, div, addf, subf, mulf, divf, addd, subd, muld, divd, eq,
//  17   18      19      20   21   22   23   24    25    26    27    28    29    30    31    32
    neq, gr, sm, not, jmp, jmpif, jmpifn, ret, call, calldy, fun, fundy
//  33   34  35  36   37   38     39      40   41    42      44   45
// Types: u8, i8, u16, i16, u32, i32, u64, i64, f32 (f), f64 (d)
];

/// stops the machine
/// 
/// Layout: 0 - 1 byte
fn halt(vm: &mut VM) {
    vm.quit = true;
}

/// pushes 1 byte
/// 
/// Layout: 1, data - 2 bytes
fn pu1(vm: &mut VM) {
    let val = vm.bytecode[vm.ip];
    vm.stack.push(val as u64);
    vm.ip += 1;
}

/// pushes 2 bytes
/// 
/// Layout: 2, data, data - 3 bytes
fn pu2(vm: &mut VM) {
    let bytes = vm.bytecode[vm.ip..vm.ip + 2].try_into().unwrap();
    let val = u16::from_be_bytes(bytes);
    vm.stack.push(val as u64);
    vm.ip += 2;
}

/// pushes 4 bytes
/// 
/// Layout: 3, data, data, data, data - 5 bytes
fn pu4(vm: &mut VM) {
    let bytes = vm.bytecode[vm.ip..vm.ip + 4].try_into().unwrap();
    let val = u32::from_be_bytes(bytes);
    vm.stack.push(val as u64);
    vm.ip += 4;
}

/// pushes 8 bytes
/// 
/// Layout: 4, data, data, data, data, data, data, data, data - 9 bytes
fn pu8(vm: &mut VM){
    let bytes = vm.bytecode[vm.ip..vm.ip + 8].try_into().unwrap();
    let val = u64::from_be_bytes(bytes);
    vm.stack.push(val);
    vm.ip += 8;
}

/// pops x (u64) and pushes x bytes (allocated on the heap) and sets them to 0
/// 
/// Layout: 5 - 1 byte
fn pux(vm: &mut VM) {
    let x = vm.stack.pop().unwrap();
    let ptr = allocate_size(x as usize);
    vm.stack.push(ptr as u64);
}

/// pops value from the stack and sets it to the global at index (u8)
/// 
/// Layout: 6, index - 2 bytes
fn set(vm: &mut VM) {
    let val = vm.stack.pop().unwrap();
    let index = vm.bytecode[vm.ip];
    vm.global[index as usize] = val;
    vm.ip += 1;
}

/// gets value from the global at index (u8) and pushes it to stack
/// 
/// Layout: 7, index - 2 bytes
fn get(vm: &mut VM) {
    let index = vm.bytecode[vm.ip];
    let val = vm.global[index as usize];
    vm.stack.push(val);
    vm.ip += 1;
}

/// pops x (u64) and sets xth byte of a pointer
/// 
/// Layout: 8, data - 2 bytes
fn setx(vm: &mut VM) {
    let x = vm.stack.pop().unwrap();
    let val = vm.bytecode[vm.ip];
    let ptr = vm.stack[vm.stack.len() - 1] as *mut u8;
    set_value_size(ptr, x as isize, val);
    vm.ip += 1;
}

/// pops offset (i64) and offsets a pointer
/// 
/// Layout: 9 - 1 bytes
fn off(vm: &mut VM) {
    let bytes = vm.stack.pop().unwrap().to_be_bytes();
    let off = i64::from_be_bytes(bytes);
    let stack_len = vm.stack.len();
    println!("{:?}", vm.stack[stack_len - 1] as *mut u8);
    let ptr = offset(vm.stack.pop().unwrap() as *mut u8, off as isize);
    vm.stack.push(ptr as u64);
    println!("{:?}", vm.stack[stack_len - 1] as *mut u8);
}

/// copies the highest index element on stack and pushes it to stack, if stack length is 0 does nothing
/// 
/// Layout: 10 - 1 byte
fn cp(vm: &mut VM) {
    vm.stack.push(vm.stack[vm.stack.len() - 1]);
}

/// pops pointer and pushes its 1 byte value to the stack
/// 
/// Layout: 11 - 1 byte
fn load1(vm: &mut VM) {
    let ptr = vm.stack.pop().unwrap() as *mut u8;
    let val: u8 = get_value(ptr);
    vm.stack.push(val as u64);
}

/// pops pointer and pushes its 2 byte value to the stack
/// 
/// Layout: 12 - 1 byte
fn load2(vm: &mut VM) {
    let ptr = vm.stack.pop().unwrap() as *mut u8;
    let val: u16 = get_value(ptr);
    vm.stack.push(val as u64);
}

/// pops pointer and pushes its 4 byte value to the stack
/// 
/// Layout: 13 - 1 byte
fn load4(vm: &mut VM) {
    let ptr = vm.stack.pop().unwrap() as *mut u8;
    let val: u32 = get_value(ptr);
    vm.stack.push(val as u64);
}

/// pops pointer and pushes its 8 byte value to the stack
/// 
/// Layout: 14 - 1 byte
fn load8(vm: &mut VM) {
    let ptr = vm.stack.pop().unwrap() as *mut u8;
    let val: u64 = get_value(ptr);
    vm.stack.push(val);
}

/// pops two values and pushes its AND result
/// 
/// Layout: 15 - 1 byte
fn and(vm: &mut VM) {
    let val1 = vm.stack.pop().unwrap();
    let val2 = vm.stack.pop().unwrap();
    let result = val1 & val2;
    vm.stack.push(result);
}

/// pops two values and pushes its OR result
/// 
/// Layout: 16 - 1 byte
fn or(vm: &mut VM) {
    let val1 = vm.stack.pop().unwrap();
    let val2 = vm.stack.pop().unwrap();
    let result = val1 | val2;
    vm.stack.push(result);
}

/// pops two values and pushes its XOR result
/// 
/// Layout: 17 - 1 byte
fn xor(vm: &mut VM) {
    let val1 = vm.stack.pop().unwrap();
    let val2 = vm.stack.pop().unwrap();
    let result = val1 ^ val2;
    vm.stack.push(result);
}

/// pops x (u64) and value and pushes its LEFT SHIFT by x result
/// 
/// Layout: 18 - 1 byte
fn lshift(vm: &mut VM) {
    let x = vm.stack.pop().unwrap();
    let val = vm.stack.pop().unwrap();
    let result = val << x;
    vm.stack.push(result);
}

/// pops x (u64) and value and pushes its RIGHT SHIFT by x result
/// 
/// Layout: 19 - 1 byte
fn rshift(vm: &mut VM) {
    let x = vm.stack.pop().unwrap();
    let val = vm.stack.pop().unwrap();
    let result = val >> x;
    vm.stack.push(result);
}

/// pops two numbers and pushes theirs ADDITION result
/// 
/// Layout: 20 - 1 byte
fn add(vm: &mut VM) {
    let val1 = vm.stack.pop().unwrap();
    let val2 = vm.stack.pop().unwrap();
    let result = val1 + val2;
    vm.stack.push(result);
}

/// pops two numbers and pushes theirs SUBSTRACTION result
/// 
/// Layout: 21 - 1 byte
fn sub(vm: &mut VM) {
    let val1 = vm.stack.pop().unwrap();
    let val2 = vm.stack.pop().unwrap();
    let result = val1 - val2;
    vm.stack.push(result);
}

/// pops two numbers and pushes theirs MULTIPLICATION result
/// 
/// Layout: 22 - 1 byte
fn mul(vm: &mut VM) {
    let val1 = vm.stack.pop().unwrap();
    let val2 = vm.stack.pop().unwrap();
    let result = val1 * val2;
    vm.stack.push(result);
}

/// pops two numbers and pushes theirs DIVISION result
/// 
/// Layout: 23 - 1 byte
fn div(vm: &mut VM) {
    let val1 = vm.stack.pop().unwrap();
    let val2 = vm.stack.pop().unwrap();
    let result = val1 / val2;
    vm.stack.push(result);
}

/// pops two f32 numbers and pushes theirs ADDITION result
/// 
/// Layout: 24 - 1 byte
fn addf(vm: &mut VM) {
    let bytes1 = (vm.stack.pop().unwrap() as u32).to_be_bytes();
    let bytes2 = (vm.stack.pop().unwrap() as u32).to_be_bytes();
    let val1 = f32::from_be_bytes(bytes1);
    let val2 = f32::from_be_bytes(bytes2);
    let result = val1 + val2;
    let bytes = result.to_be_bytes();
    let result = u32::from_be_bytes(bytes) as u64;
    vm.stack.push(result);
}

/// pops two f32 numbers and pushes theirs SUBSTRACTION result
/// 
/// Layout: 25 - 1 byte
fn subf(vm: &mut VM) {
    let bytes1 = (vm.stack.pop().unwrap() as u32).to_be_bytes();
    let bytes2 = (vm.stack.pop().unwrap() as u32).to_be_bytes();
    let val1 = f32::from_be_bytes(bytes1);
    let val2 = f32::from_be_bytes(bytes2);
    let result = val1 - val2;
    let bytes = result.to_be_bytes();
    let result = u32::from_be_bytes(bytes) as u64;
    vm.stack.push(result);
}

/// pops two f32 numbers and pushes theirs MULTIPLICATION result
/// 
/// Layout: 26 - 1 byte
fn mulf(vm: &mut VM) {
    let bytes1 = (vm.stack.pop().unwrap() as u32).to_be_bytes();
    let bytes2 = (vm.stack.pop().unwrap() as u32).to_be_bytes();
    let val1 = f32::from_be_bytes(bytes1);
    let val2 = f32::from_be_bytes(bytes2);
    let result = val1 * val2;
    let bytes = result.to_be_bytes();
    let result = u32::from_be_bytes(bytes) as u64;
    vm.stack.push(result);
}

/// pops two f32 numbers and pushes theirs DIVISION result
/// 
/// Layout: 27 - 1 byte
fn divf(vm: &mut VM) {
    let bytes1 = (vm.stack.pop().unwrap() as u32).to_be_bytes();
    let bytes2 = (vm.stack.pop().unwrap() as u32).to_be_bytes();
    let val1 = f32::from_be_bytes(bytes1);
    let val2 = f32::from_be_bytes(bytes2);
    let result = val1 / val2;
    let bytes = result.to_be_bytes();
    let result = u32::from_be_bytes(bytes) as u64;
    vm.stack.push(result);
}

/// pops two f64 numbers and pushes theirs ADDITION result
/// 
/// Layout: 28 - 1 byte
fn addd(vm: &mut VM) {
    let bytes1 = vm.stack.pop().unwrap().to_be_bytes();
    let bytes2 = vm.stack.pop().unwrap().to_be_bytes();
    let val1 = f64::from_be_bytes(bytes1);
    let val2 = f64::from_be_bytes(bytes2);
    let result = val1 + val2;
    let bytes = result.to_be_bytes();
    let result = u64::from_be_bytes(bytes);
    vm.stack.push(result);
}

/// pops two f64 numbers and pushes theirs SUBSTRACTION result
/// 
/// Layout: 29 - 1 byte
fn subd(vm: &mut VM) {
    let bytes1 = vm.stack.pop().unwrap().to_be_bytes();
    let bytes2 = vm.stack.pop().unwrap().to_be_bytes();
    let val1 = f64::from_be_bytes(bytes1);
    let val2 = f64::from_be_bytes(bytes2);
    let result = val1 - val2;
    let bytes = result.to_be_bytes();
    let result = u64::from_be_bytes(bytes);
    vm.stack.push(result);
}

/// pops two f64 numbers and pushes theirs MULTIPLICATION result
/// 
/// Layout: 30 - 1 byte
fn muld(vm: &mut VM) {
    let bytes1 = vm.stack.pop().unwrap().to_be_bytes();
    let bytes2 = vm.stack.pop().unwrap().to_be_bytes();
    let val1 = f64::from_be_bytes(bytes1);
    let val2 = f64::from_be_bytes(bytes2);
    let result = val1 * val2;
    let bytes = result.to_be_bytes();
    let result = u64::from_be_bytes(bytes);
    vm.stack.push(result);
}

/// pops two f64 numbers and pushes theirs DIVISION result
/// 
/// Layout: 31 - 1 byte
fn divd(vm: &mut VM) {
    let bytes1 = vm.stack.pop().unwrap().to_be_bytes();
    let bytes2 = vm.stack.pop().unwrap().to_be_bytes();
    let val1 = f64::from_be_bytes(bytes1);
    let val2 = f64::from_be_bytes(bytes2);
    let result = val1 / val2;
    let bytes = result.to_be_bytes();
    let result = u64::from_be_bytes(bytes);
    vm.stack.push(result);
}

/// pops 2 values and sets the flag to true if they are EQUAL
/// 
/// Layout: 32 - 1 byte 
fn eq(vm: &mut VM) {
    let val1 = vm.stack.pop().unwrap();
    let val2 = vm.stack.pop().unwrap();
    let result = val1 == val2;
    vm.flag = result;
}

/// pops 2 values and sets the flag to true if they are NOT EQUAL
/// 
/// Layout: 33 - 1 byte 
fn neq(vm: &mut VM) {
    let val1 = vm.stack.pop().unwrap();
    let val2 = vm.stack.pop().unwrap();
    let result = val1 != val2;
    vm.flag = result;
}

/// pops 2 values and sets the flag to true if val1 is GREATER than val2
/// 
/// Layout: 34 - 1 byte 
fn gr(vm: &mut VM) {
    let val1 = vm.stack.pop().unwrap();
    let val2 = vm.stack.pop().unwrap();
    let result = val1 > val2;
    vm.flag = result;
}

/// pops 2 values and sets the flag to true if val1 is SMALLER than val2 
/// 
/// Layout: 35 - 1 byte 
fn sm(vm: &mut VM) {
    let val1 = vm.stack.pop().unwrap();
    let val2 = vm.stack.pop().unwrap();
    let result = val1 < val2;
    vm.flag = result;
}

/// uses NOT on the flag
/// 
/// Layout: 36 - 1 byte 
fn not(vm: &mut VM) {
    vm.flag = !vm.flag;
}


/// pops index (u64) and jumps to its location
/// 
/// Layout: 37 - 1 byte
fn jmp(vm: &mut VM) {
    let index = vm.stack.pop().unwrap();
    vm.ip = index as usize;
}

/// pops index (u64), jumps to index location if flag is TRUE
/// 
/// Layout: 38 - 1 byte
fn jmpif(vm: &mut VM) {
    if vm.flag {
        jmp(vm);
    }
}

/// pops index (u64), jumps to index location if flag is FALSE
/// 
/// Layout: 39 - 1 byte
fn jmpifn(vm: &mut VM) {
    if !vm.flag {
        jmp(vm);
    }
}

/// returns from function call, jumps to last function pointer
/// 
/// Layout: 40 - 1 byte
fn ret(vm: &mut VM) {
    vm.ip = vm.fn_pointers.pop().unwrap();
}

/// calls a function at index (u64)
/// 
/// Layout: 41, index, index, index, index, index, index, index, index - 9 bytes
fn call(vm: &mut VM) {
    let bytes = vm.bytecode[vm.ip..vm.ip + 8].try_into().unwrap();
    let index = u64::from_be_bytes(bytes);
    vm.ip += 8;
    vm.fn_pointers.push(vm.ip);
    vm.ip = index as usize;
}

/// pops index (u64) and calls a function at it
/// 
/// Layout: 42 - 1 byte
fn calldy(vm: &mut VM) {
    let index = vm.stack.pop().unwrap();
    vm.fn_pointers.push(vm.ip);
    vm.ip = index as usize;
}

/// calls an extern function at index
/// 
/// Layout: 43, index, index, index, index, index, index, index, index - 9 bytes
fn fun(vm: &mut VM) {
    let bytes = vm.bytecode[vm.ip..vm.ip + 8].try_into().unwrap();
    let index = u64::from_be_bytes(bytes);
    vm.ip += 8;
    vm.functions[index as usize](vm);
}

/// pops index and calls an extern function at index
/// 
/// Layout: 43, index, index, index, index, index, index, index, index - 9 bytes
fn fundy(vm: &mut VM) {
    let index = vm.stack.pop().unwrap();
    vm.functions[index as usize](vm);
}