/*  main.rs
    MIT License

    Copyright (c) 2024 Aidar Shigapov

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

use std::*;
use std::process::exit;
use collections::hash_map::HashMap;
use clap::{App, Arg};

macro_rules! INT_FMT {
    () => {
        "\tpush qword {}\n"
    };
}

pub const ASM_CODE_BEGIN_WIN64: &str = "section .import\n\textern printf\n\textern exit\n\nsection .data\n\t@int_fmt: db \"%lli \", 0\n\t@second_stack: times 1024 dq 0\n\t@stack_size: dq 0\n\nsection .text\n\tglobal WinMain\n\nWinMain:\n\tjmp start\n\n@share_to_second_stack:\n\t; arg - rax\n\tmov rbx, @second_stack\n\tmov rcx, @stack_size\n\tmov rcx, [rcx]\n\tmov qword [rbx + rcx * 8], rax\n\tmov rcx, @stack_size\n\tinc qword [rcx]\n\tret\n\n@get_pop_second_stack:\n\t; return - rax\n\tmov rbx, @second_stack\n\tmov rcx, @stack_size\n\tmov rcx, [rcx]\n\tmov rax, qword [rbx + rcx * 8 - 8]\n\tmov rcx, @stack_size\n\tdec qword [rcx]\n\tret\n";
pub const ASM_CODE_BEGIN_LINUX: &str = "section .import\n\textern printf\n\textern exit\n\nsection .data\n\t@int_fmt: db \"%lli \", 0\n\t@second_stack: times 1024 dq 0\n\t@stack_size: dq 0\n\nsection .text\n\tglobal main\n\nmain:\n\tjmp start\n\n@share_to_second_stack:\n\t; arg - rax\n\tmov rbx, @second_stack\n\tmov rcx, @stack_size\n\tmov rcx, [rcx]\n\tmov qword [rbx + rcx * 8], rax\n\tmov rcx, @stack_size\n\tinc qword [rcx]\n\tret\n\n@get_pop_second_stack:\n\t; return - rax\n\tmov rbx, @second_stack\n\tmov rcx, @stack_size\n\tmov rcx, [rcx]\n\tmov rax, qword [rbx + rcx * 8 - 8]\n\tmov rcx, @stack_size\n\tdec qword [rcx]\n\tret\n";

const STACK_HEAD_ASM: &str = "\tpush rsp ; stack_head\n";
const READ_FROM_ASM: &str = "\tpop rax ; read_from\n\tpush qword [rax]\n";
const WRITE_TO_ASM: &str = "\tpop rax ; write_to\n\tpop rbx\n\tmov qword [rbx], rax\n";
const POP_ASM: &str = "\tadd rsp, 8\n";
const SUM_ASM: &str = "\tpop rax ; sum\n\tpop rbx\n\tadd rbx, rax\n\tpush rbx\n";
const DIF_ASM: &str = "\tpop rax ; dif\n\tpop rbx\n\tsub rbx, rax\n\tpush rbx\n";
const MUL_ASM: &str = "\tpop rax ; mul\n\tpop rbx\n\tmul rbx\n\tpush rax\n";
const DIV_ASM: &str = "\tpop rbx; div\n\tpop rax\n\txor rdx, rdx\n\tdiv rbx\n\tpush rax\n";
const DUP_ASM: &str = "\tpush qword [rsp] ; dup\n";
const SWAP0_1_ASM: &str = "\tpop rax ; swap0_1\n\tpop rbx\n\tpush rax\n\tpush rbx\n";
const SWAP0_2_ASM: &str = "\tpop rax ; swap0_2\n\tpop rbx\n\tpop rcx\n\tpush rax\n\tpush rbx\n\tpush rcx\n";

const PRINT_ASM_WIN64: &str = "\tlea rcx, [rel @int_fmt]\n\tpop rdx ; print\n\tsub rsp, 32\n\tcall printf\n\tadd rsp, 32\n";
const EXIT_ASM_WIN64: &str = "\tpop rcx\n\tcall exit\n";
const SUCCESFUL_EXIT_ASM_WIN64: &str = "\txor rcx, rcx\n\tcall exit\n";

const PRINT_ASM_LINUX: &str = "\tlea rdi, [rel @int_fmt]\n\tpop rsi ; print\n\tsub rsp, 32\n\tcall printf\n\tadd rsp, 32\n";
const EXIT_ASM_LINUX: &str = "\tpop rdi\n\tcall exit\n";
const SUCCESFUL_EXIT_ASM_LINUX: &str = "\txor rdi, rdi\n\tcall exit\n";

#[derive(Eq, PartialEq)]
#[derive(Hash)]
enum StateType {
    Integer,
    StackHead,
    ReadFrom,
    WriteTo,
    Pop,
    Sum,
    Dif,
    Mul,
    Div,
    Dup,
    If,
    Else,
    Swap0_1,
    Swap0_2,
    SelfCall,
    SelfGoto,
    Print,
    Exit,
    Additional,
}

type StackValueType = i32;

impl Default for StateType {
    fn default() -> Self {
        StateType::Additional
    }
}
impl Clone for StateType {
    fn clone(&self) -> Self {
        match *self {
            StateType::Integer =>   StateType::Integer,
            StateType::StackHead => StateType::StackHead,
            StateType::ReadFrom =>  StateType::ReadFrom,
            StateType::WriteTo =>   StateType::WriteTo,
            StateType::Pop =>       StateType::Pop,
            StateType::Sum =>       StateType::Sum,
            StateType::Dif =>       StateType::Dif,
            StateType::Mul =>       StateType::Mul,
            StateType::Div =>       StateType::Div,
            StateType::Dup =>       StateType::Dup,
            StateType::If =>        StateType::If,
            StateType::Else =>      StateType::Else,
            StateType::Swap0_1 =>   StateType::Swap0_1,
            StateType::Swap0_2 =>   StateType::Swap0_2,
            StateType::SelfCall =>  StateType::SelfCall,
            StateType::SelfGoto =>  StateType::SelfGoto,
            StateType::Print =>     StateType::Print,
            StateType::Exit =>      StateType::Exit,
            StateType::Additional =>StateType::Additional,
        }
    }
}

#[derive(Clone)]
#[derive(Default)]
struct State {
    name: String,
    deps: Vec<State>,
    state_type: StateType,
    inlinable: bool, // inlinable states cannot contain 'if', 'else', '__self__', '__self__goto__'
}

fn numeric_state(str: String) -> State {
    State{name: str, state_type: StateType::Integer, deps: Vec::<State>::new(), inlinable: false }
}

fn execute_statement(state: &State, stack: &mut Vec::<StackValueType>) -> Option<()> {
    match state.state_type {
        StateType::Integer => {
            stack.push(state.name.parse::<StackValueType>().ok()?);
        }
        StateType::StackHead => {
            panic!("stack_head not allowed in interpriter mode");
        }
        StateType::ReadFrom => {
            panic!("read_from not allowed in interpriter mode");
        }
        StateType::WriteTo => {
            panic!("write_to not allowed in interpriter mode");
        }
        StateType::Pop => {
            stack.pop().ok_or_else(|| "stack is empty on pop".to_string()).ok()?;
        }
        StateType::Sum => {
            let first_argument = stack.pop().ok_or_else(|| "stack is empty on sum".to_string()).ok()?;
            let second_argument = stack.pop().ok_or_else(|| "stack is empty on sum".to_string()).ok()?;
            stack.push(second_argument + first_argument);
        }
        StateType::Dif => {
            let first_argument = stack.pop().ok_or_else(|| "stack is empty on dif".to_string()).ok()?;
            let second_argument = stack.pop().ok_or_else(|| "stack is empty on dif".to_string()).ok()?;
            stack.push(second_argument - first_argument);
        }
        StateType::Mul => {
            let first_argument = stack.pop().ok_or_else(|| "stack is empty on mul".to_string()).ok()?;
            let second_argument = stack.pop().ok_or_else(|| "stack is empty on mul".to_string()).ok()?;
            stack.push(second_argument * first_argument);
        }
        StateType::Div => {
            let first_argument = stack.pop().ok_or_else(|| "stack is empty on div".to_string()).ok()?;
            let second_argument = stack.pop().ok_or_else(|| "stack is empty on div".to_string()).ok()?;
            stack.push(second_argument / first_argument);
        }
        StateType::Dup => {
            let last = stack.last().ok_or_else(|| "stack is empty on dup".to_string()).ok()?;
            stack.push(*last);
        }
        StateType::Swap0_1 => {
            let end_index = stack.len() - 1;
            let end_prev_index = stack.len() - 2;
            stack.swap(end_index, end_prev_index);
        }
        StateType::Swap0_2 => {
            let end_index = stack.len() - 1;
            let end_prev_prev_index = stack.len() - 3;
            stack.swap(end_index, end_prev_prev_index);
        }
        StateType::Print => {
            let last = stack.pop().ok_or_else(|| "stack is empty on print".to_string()).ok()?;
            print!("{} ", last);
        }
        StateType::Exit => {
            exit(0);
        }
        _ => {
            let mut else_count = 0;
            let mut if_count = 0;
            for i in state.deps.iter() {
                if i.state_type == StateType::Else {
                    else_count += 1;
                } else if i.state_type == StateType::If {
                    if_count += 1;
                }
            }
            assert_eq!(if_count, else_count);

            let mut if_happens = false;
            let mut iter =  state.deps.iter();
            loop {
                let io = iter.as_slice().first();
                if io.is_none() {
                    break;
                }
                let i = io.unwrap();
                if i.state_type ==  StateType::If {
                    let last = stack.pop()?;
                    if last <= 0 {
                        if_happens = true;
                    }
                }
                if if_happens  {
                    if i.state_type ==  StateType::Else {
                        if_happens = false;
                    }
                } else if i.state_type ==  StateType::Else {
                    break;
                } else if i.state_type == StateType::SelfCall {
                    execute_statement(state, stack)?;
                } else if i.state_type == StateType::SelfGoto {
                    iter =  state.deps.iter();
                    continue;
                } else {
                    execute_statement(i, stack)?;
                }
                iter.next();
            }
        }
    }
    Some(())
}
fn is_inlinable(state: &State) -> bool {
    if state.name == "start" {
        return false
    }
    for i in state.deps.iter().enumerate() {
        if  (i.1.state_type == StateType::If) || 
            (i.1.state_type == StateType::Else) || 
            (i.1.state_type == StateType::SelfCall) ||
            (i.1.state_type == StateType::SelfGoto) {
                return false
        }
    }
    return true
}
fn compile_statement(state: &State) -> Option<String> {
    let mut map = HashMap::new();
    map.insert(StateType::StackHead,    STACK_HEAD_ASM);
    map.insert(StateType::ReadFrom,     READ_FROM_ASM);
    map.insert(StateType::WriteTo,      WRITE_TO_ASM);
    map.insert(StateType::Pop,          POP_ASM);
    map.insert(StateType::Sum,          SUM_ASM);
    map.insert(StateType::Dif,          DIF_ASM);
    map.insert(StateType::Mul,          MUL_ASM);
    map.insert(StateType::Div,          DIV_ASM);
    map.insert(StateType::Dup,          DUP_ASM);
    map.insert(StateType::Swap0_1,      SWAP0_1_ASM);
    map.insert(StateType::Swap0_2,      SWAP0_2_ASM);

    if cfg!(target_os = "windows") {
        map.insert(StateType::Print,        PRINT_ASM_WIN64);
        map.insert(StateType::Exit,         EXIT_ASM_WIN64);
    } else {
        map.insert(StateType::Print,        PRINT_ASM_LINUX);
        map.insert(StateType::Exit,         EXIT_ASM_LINUX);
    }

    if let Some(asm) = map.get(&state.state_type) {
        return Some(asm.to_string());
    }
    if state.state_type == StateType::Integer {
        return Some(format!(INT_FMT!(), state.name));
    }
    let mut out = String::new();
    if state.inlinable {
        let mut iter =  state.deps.iter();
        loop {
            let io = iter.as_slice().first();
            if io.is_none() {
                break;
            }
            let i = io.unwrap();
            
            if i.inlinable || i.state_type == StateType::Integer {
                out += compile_statement(i).expect("compiling error").as_str();
            } else {
                out += "\tcall ";
                out += i.name.as_str();
                out += "\n";
            }
            iter.next();
        }
    } else {
        let statement_exit = if state.name == "start" { 
            if cfg!(target_os = "windows") {
                SUCCESFUL_EXIT_ASM_WIN64
            } else {
                SUCCESFUL_EXIT_ASM_LINUX
            }
        } else {
            "\tcall @get_pop_second_stack\n\tpush rax\n\tret\n\tpop rax\n"
        };

        out += state.name.as_str();
        out += ":\n\tpop rax\n\tcall @share_to_second_stack\n";
        out += "@";
        out += state.name.as_str();
        out += "_jump_position_you_know:\n";
        
        {
            let mut else_count = 0;
            let mut if_count = 0;
            for i in state.deps.iter() {
                if i.state_type == StateType::Else {
                    else_count += 1;
                } else if i.state_type == StateType::If {
                    if_count += 1;
                }
            }
            assert!(if_count == else_count, "for every `if`, there should be a corresponding `else`. check {} statement", state.name);
        }
        let mut iter =  state.deps.iter();
        let mut if_count = 0;
        let mut if_stack = Vec::new();
        loop {
            let io = iter.as_slice().first();
            if io.is_none() {
                break;
            }
            let i = io.unwrap();
            
            if i.state_type ==  StateType::If {
                if_count += 1;
                if_stack.push(if_count);
                out += "\tpop rax\n";
                out += "\tcmp rax, 0\n";
                out += format!("\tjle @{}_else{}\n", state.name, if_count).as_str();
                
            } else if i.state_type ==  StateType::Else {

                out += format!("{}@{}_else{}:\n", statement_exit, state.name, if_stack.pop().expect("unexpected else")).as_str();
            } else if i.state_type == StateType::SelfCall {
                out += "\tcall ";
                out += state.name.as_str();
                out += "\n";
            } else if i.state_type == StateType::SelfGoto {
                out += "\tjmp @";
                out += state.name.as_str();
                out += "_jump_position_you_know\n";
            } else {
                if i.inlinable || i.state_type == StateType::Integer {
                    out += compile_statement(i).expect("compilation error").as_str();
                } else {
                    out += "\tcall ";
                    out += i.name.as_str();
                    out += "\n";
                }
            }
            iter.next();
        }
        out += statement_exit;
    }
    return Some(out)
}

fn main() {
    let matches = App::new("jalgo")
        .version("1.0")
        .author("Aidar Shigapov")
        .about("compiler/interpriter for jango language")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("sets the output file to use")
                .index(2),
        )
        .arg(
            Arg::with_name("mode")
                .short("m")
                .long("interprutation/compilation mode")
                .value_name("String")
                .help("sets the interprutation/compilation mode.\n\tposible values: c | i")
                .index(3),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output");
    let mode = matches.value_of("mode");

    let code = std::fs::read_to_string(input_file).expect("Unable to read file");

    let mut states: Vec<State> = vec![
        State{name: "stack_head".to_string(),       state_type: StateType::StackHead,   deps: Vec::<State>::default(), inlinable: true },
        State{name: "read_from".to_string(),        state_type: StateType::ReadFrom,    deps: Vec::<State>::default(), inlinable: true },
        State{name: "write_to".to_string(),         state_type: StateType::WriteTo,     deps: Vec::<State>::default(), inlinable: true },
        State{name: "print".to_string(),            state_type: StateType::Print,       deps: Vec::<State>::default(), inlinable: true },
        State{name: "pop".to_string(),              state_type: StateType::Pop,         deps: Vec::<State>::default(), inlinable: true },
        State{name: "sum".to_string(),              state_type: StateType::Sum,         deps: Vec::<State>::default(), inlinable: true },
        State{name: "dif".to_string(),              state_type: StateType::Dif,         deps: Vec::<State>::default(), inlinable: true },
        State{name: "mul".to_string(),              state_type: StateType::Mul,         deps: Vec::<State>::default(), inlinable: true },
        State{name: "div".to_string(),              state_type: StateType::Div,         deps: Vec::<State>::default(), inlinable: true },
        State{name: "dup".to_string(),              state_type: StateType::Dup,         deps: Vec::<State>::default(), inlinable: true },
        State{name: "if".to_string(),               state_type: StateType::If,          deps: Vec::<State>::default(), inlinable: true },
        State{name: "else".to_string(),             state_type: StateType::Else,        deps: Vec::<State>::default(), inlinable: true },
        State{name: "swap".to_string(),             state_type: StateType::Swap0_1,     deps: Vec::<State>::default(), inlinable: true },
        State{name: "swap0_1".to_string(),          state_type: StateType::Swap0_1,     deps: Vec::<State>::default(), inlinable: true },
        State{name: "swap0_2".to_string(),          state_type: StateType::Swap0_2,     deps: Vec::<State>::default(), inlinable: true },
        State{name: "__self__".to_string(),         state_type: StateType::SelfCall,    deps: Vec::<State>::default(), inlinable: true },
        State{name: "__self__goto__".to_string(),   state_type: StateType::SelfGoto,    deps: Vec::<State>::default(), inlinable: true },
        State{name: "exit".to_string(),             state_type: StateType::Exit,        deps: Vec::<State>::default(), inlinable: true },
    ];
    let mut last_state = State::default();
    let tokens = code.split_whitespace();

    let mut state_colon = false;
    let mut state_found = false;
    let mut in_comment = false;
    for i in tokens.enumerate() {
        if i.1.is_empty() {
            continue;
        }
        if in_comment { 
            if i.1 == "*/" {
                in_comment = false
            }
            continue;
        }
        if i.1 == "/*" {
            in_comment = true;
            continue;
        }
        
        if state_found {
            last_state.name = i.1.to_string();
            if last_state.name.get(0..1).expect("too small statement name") == "@" {
                panic!("the first character of the state name cannot be '@'");
            }
            state_found = false;
        } else if state_colon {
            if i.1 == ";" {
                state_colon = false;
                // inline-check
                if is_inlinable(&last_state) {
                    last_state.inlinable = true;
                }

                states.push(last_state.clone());
                last_state = State::default();
                continue;
            }
            if i.1.get(0..1).expect("too small statement name") == "@" {
                panic!("the first character of the state name cannot be '@'");
            }
            if let Ok(num) = i.1.parse::<StackValueType>() {
                last_state.deps.push(numeric_state(num.to_string()));
                continue;
            }

            last_state.deps.push(
                states
                    .iter()
                    .find_map(|x| if x.name == i.1 { Some(x.clone()) } else { None })
                    .expect(&format!("invalid statement \"{}\"", i.1)),
            );
        } else if i.1 == "st" {
            state_found = true;
        } else if i.1 == ":" {
            state_colon = true;
        }
    }

    if let Some(mode) = mode {
        if mode == "i" {
            let mut stack = Vec::<StackValueType>::new();
            execute_statement(states.iter().enumerate().find(|x| x.1.name == "start").expect("entry point \"start\" doesnt exist").1, &mut stack);
            return;
        } else if mode != "c" {
            panic!("unknown mode. check --help")
        }
    }

    let asm_code_begin = if cfg!(target_os = "windows") {
        ASM_CODE_BEGIN_WIN64
    } else {
        ASM_CODE_BEGIN_LINUX
    };

    let mut compiled_code = format!("{}",asm_code_begin);
    for i in states.iter().filter(|x| !x.inlinable).enumerate() {
        compiled_code += compile_statement(i.1).expect("compilation error").as_str();
    }

    if let Some(output_file) = output_file {
        std::fs::write(output_file, compiled_code).expect("unable to write file");
    } else {
        println!("{}", compiled_code);
    }
}