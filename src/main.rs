use std::*;
use std::process::exit;
use collections::hash_map::HashMap;
use clap::{App, Arg};

macro_rules! INT_FMT {
    () => {
        "\tpush {}\n"
    };
}

mod windows {
    pub const ASM_CODE_BEGIN: &str = "section .import\n\textern printf\n\textern exit\n\nsection .data\n\tint_fmt: db \"%d \", 0\n\tsecond_stack: times 1024 dq 0\n\tstack_size: dq 0\n\nsection .text\n\tglobal WinMain\n\nWinMain:\n\tjmp start\n\nshare_to_second_stack:\n\t; arg - rax\n\tmov rbx, second_stack\n\tmov rcx, stack_size\n\tmov rcx, [rcx]\n\tmov qword [rbx + rcx * 8], rax\n\tmov rcx, stack_size\n\tinc qword [rcx]\n\tret\n\nget_pop_second_stack:\n\t; return - rax\n\tmov rbx, second_stack\n\tmov rcx, stack_size\n\tmov rcx, [rcx]\n\tmov rax, qword [rbx + rcx * 8 - 8]\n\tmov rcx, stack_size\n\tdec qword [rcx]\n\tret\n";
}

mod linux {
    pub const ASM_CODE_BEGIN: &str = "section .import\n\textern printf\n\textern exit\n\nsection .data\n\tint_fmt: db \"%d \", 0\n\tsecond_stack: times 1024 dq 0\n\tstack_size: dq 0\n\nsection .text\n\tglobal main\n\nmain:\n\tjmp start\n\nshare_to_second_stack:\n\t; arg - rax\n\tmov rbx, second_stack\n\tmov rcx, stack_size\n\tmov rcx, [rcx]\n\tmov qword [rbx + rcx * 8], rax\n\tmov rcx, stack_size\n\tinc qword [rcx]\n\tret\n\nget_pop_second_stack:\n\t; return - rax\n\tmov rbx, second_stack\n\tmov rcx, stack_size\n\tmov rcx, [rcx]\n\tmov rax, qword [rbx + rcx * 8 - 8]\n\tmov rcx, stack_size\n\tdec qword [rcx]\n\tret\n";
}

const PRINT_ASM: &str = "\tpop rdx ; print\n\tlea rcx, [rel int_fmt]\n\tsub rsp, 32\n\tcall printf\n\tadd rsp, 32\n";
const POP_ASM: &str = "\tpop r9\n";
const SUM_ASM: &str = "\tpop rax ; sum\n\tpop rbx\n\tadd rbx, rax\n\tpush rbx\n";
const DIF_ASM: &str = "\tpop rax ; dif\n\tpop rbx\n\tsub rbx, rax\n\tpush rbx\n";
const MUL_ASM: &str = "\tpop rax ; mul\n\tpop rbx\n\tmul rbx\n\tpush rax\n";
const DIV_ASM: &str = "\tpop rbx; div\n\tpop rax\n\txor rdx, rdx\n\tdiv rbx\n\tpush rax\n";
const DUP_ASM: &str = "\tpop rax ; dup\n\tpush rax\n\tpush rax\n";
const SWAP0_1_ASM: &str = "\tpop rax ; swap0_1\n\tpop rbx\n\tpush rax\n\tpush rbx\n";
const SWAP0_2_ASM: &str = "\tpop rax ; swap0_2\n\tpop rbx\n\tpop rcx\n\tpush rax\n\tpush rbx\n\tpush rcx\n";
const EXIT_ASM: &str = "\tcall exit\n";

#[derive(Eq, PartialEq)]
#[derive(Hash)]
enum StateType {
    Integer,
    Print,
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
            StateType::Print =>     StateType::Print,
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
    inlinable: bool,
}

fn numeric_state(str: String) -> State {
    State{name: str, state_type: StateType::Integer, deps: Vec::<State>::new(), inlinable: false }
}

fn execute_statement(state: &State, stack: &mut Vec::<StackValueType>) -> Option<()> {
    match state.state_type {
        StateType::Integer => {
            stack.push(state.name.parse::<StackValueType>().ok()?);
        }
        StateType::Print => {
            let last = stack.pop().ok_or_else(|| "stack is empty on print".to_string()).ok()?;
            print!("{} ", last);
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
        StateType::Exit => {
            exit(0);
        }
        _ => {
            let mut if_happens = false;
            let mut iter =  state.deps.iter();
            loop {
                let io = iter.as_slice().first();
                if io.is_none() {
                    break;
                }
                let i = io.unwrap();
                
                if if_happens  {
                    if i.state_type ==  StateType::Else {
                        if_happens = false;
                    }
                } else if i.state_type ==  StateType::If {
                    let last = stack.pop()?;
                    if last <= 0 {
                        if_happens = true;
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
fn compile_statement(state: &State) -> Option<String> {
    let mut map = HashMap::new();
    map.insert(StateType::Print, PRINT_ASM);
    map.insert(StateType::Pop, POP_ASM);
    map.insert(StateType::Sum, SUM_ASM);
    map.insert(StateType::Dif, DIF_ASM);
    map.insert(StateType::Mul, MUL_ASM);
    map.insert(StateType::Div, DIV_ASM);
    map.insert(StateType::Dup, DUP_ASM);
    map.insert(StateType::Swap0_1, SWAP0_1_ASM);
    map.insert(StateType::Swap0_2, SWAP0_2_ASM);
    map.insert(StateType::Exit, EXIT_ASM);

    if let Some(asm) = map.get(&state.state_type) {
        return Some(asm.to_string());
    }
    if state.state_type == StateType::Integer {
        return Some(format!(INT_FMT!(), state.name));
    }
    let mut out = String::new();
    out += state.name.as_str();
    out += ":\n\tpop rax\n\tcall share_to_second_stack\n";
    out += state.name.as_str();
    out += "_jump_position_you_know:\n";

    let mut if_happens = false;

    let mut iter =  state.deps.iter();
    let mut if_count = 0;
    loop {
        let io = iter.as_slice().first();
        if io.is_none() {
            break;
        }
        let i = io.unwrap();
        
        if if_happens  {
            if i.state_type ==  StateType::Else {
                out += "\tcall get_pop_second_stack\n\tpush rax\n\tret\n\tpop rax\n";
                out += format!("{}_else{}:\n", state.name, if_count).as_str();
                if_happens = false;
                if_count += 1;
            }
        }
        if i.state_type ==  StateType::If {
            out += "\tpop rax\n";
            out += "\tcmp rax, 0\n";
            out += format!("\tjle {}_else{}\n", state.name, if_count).as_str();
            if_happens = true;
            
        } else if i.state_type ==  StateType::Else {
            
        } else if i.state_type == StateType::SelfCall {
            out += "\tcall ";
            out += state.name.as_str();
            out += "\n";
        } else if i.state_type == StateType::SelfGoto {
            out += "\tjmp ";
            out += state.name.as_str();
            out += "_jump_position_you_know\n";
        } else {
            if i.inlinable || i.state_type == StateType::Integer {
                out += compile_statement(i).expect("compiling error").as_str();
            } else {
                out += "\tcall ";
                out += i.name.as_str();
                out += "\n";
            }
        }
        iter.next();
    }
    if if_happens {
        out += format!("{}_else{}:\n", state.name, if_count).as_str();
    }
    out += "\tcall get_pop_second_stack\n\tpush rax\n\tret\n";
    return Some(out)
}

fn main() {
    let matches = App::new("jalgo")
        .version("1.0")
        .author("Your Name")
        .about("A simple compiler")
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
                .value_name("boolean")
                .help("sets the interprutation/compilation mode.\n\tposible values: c | i")
                .index(3),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output");
    let mode = matches.value_of("mode");

    let code = std::fs::read_to_string(input_file).expect("Unable to read file");

    let mut states: Vec<State> = vec![
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
    for i in tokens.enumerate() {
        if i.1.is_empty() {
            continue;
        }
        if state_found {
            last_state.name = i.1.to_string();
            state_found = false;
        } else if state_colon {
            if i.1 == ";" {
                state_colon = false;
                states.push(last_state.clone());
                last_state = State::default();
                continue;
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
            panic!("unknown mode. Check --help")
        }
    }

    let asm_code_begin = if cfg!(target_os = "windows") {
        windows::ASM_CODE_BEGIN
    } else {
        linux::ASM_CODE_BEGIN
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
