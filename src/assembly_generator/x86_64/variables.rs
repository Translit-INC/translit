use std::{collections::HashMap, ops::Add};
use spin::Mutex;

use super::helper;
use crate::Type;

// please forgive me......
static IDX: Mutex<u64> = Mutex::new(0);
static STACK_SIZE: Mutex<u64> = Mutex::new(0);
lazy_static::lazy_static! {
    // first - index of variable, second - stack size after the value is pushed
    static ref VAR_MAP: Mutex<HashMap<u64, u64>> = Mutex::new(HashMap::new());
}

pub fn create_var(write_buffer: &mut String, var_type: Type, value: u64) -> VariableIndex {
    let reg = helper::get_register(var_type);
    let var_size = helper::get_size(var_type);
    let current_idx = *IDX.lock();
    let current_stack_size = *STACK_SIZE.lock() + var_size;

    VAR_MAP.lock().insert(current_idx, current_stack_size);

    *write_buffer += create_var_asm(var_type, value).as_str();

    *IDX.lock() += 1;
    *STACK_SIZE.lock() += var_size;

    VariableIndex(var_type, current_idx)
}

pub fn load_variable(register: &str, var: VariableIndex) -> String {
    // we wanna get the size of variables pushed after the variable, for that we can do:
    // stack size then - current stack size
    let relative_position = *STACK_SIZE.lock() - VAR_MAP.lock().get(&var.into()).unwrap();

    let type_name = helper::get_type_name(var.0);

    format!("\tmov {register}, {type_name} [rsp-{}]\n", relative_position)
}

/// Must be called at the end of function
pub fn free_all() -> String {
    format!("\tadd rsp, {}\n", *STACK_SIZE.lock())
}

fn create_var_asm(typ: Type, value: u64) -> String {
    let type_name = helper::get_type_name(typ);
    let var_size = helper::get_size(typ);

    format!("\tsub rsp, {var_size}\n\tmov {type_name} rsp, {value}\n")
}

#[derive(Clone, Copy)]
pub struct VariableIndex(Type, u64);

impl From<VariableIndex> for u64 {
    fn from(value: VariableIndex) -> Self {
        value.1
    }
}
