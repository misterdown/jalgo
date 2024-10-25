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
use io::Write;

macro_rules! INT_FMT {
    () => {
        "\tpush qword {}\n"
    };
}
macro_rules! BIG_INT_FMT { // x > i32.max
    () => {
        "\tmov rax, {}\n\tpush rax\n"
    };
}

const ASM_CODE_BEGIN_WIN64: &str = "section .import\n\textern printf\n\textern exit\n\nsection .data\n\t@int_fmt: db \"%lli \", 0\n\t@bytes_fmt: db \"%.8s\", 0\n\t@second_stack: times 1024 dq 0\n\t@stack_size: dq 0\n\nsection .text\n\tglobal WinMain\n\nWinMain:\n\tjmp start\n\n@share_to_second_stack:\n\t; arg - rax\n\tmov rbx, @second_stack\n\tmov rcx, @stack_size\n\tmov rcx, [rcx]\n\tmov qword [rbx + rcx * 8], rax\n\tmov rcx, @stack_size\n\tinc qword [rcx]\n\tret\n\n@get_pop_second_stack:\n\t; return - rax\n\tmov rbx, @second_stack\n\tmov rcx, @stack_size\n\tmov rcx, [rcx]\n\tmov rax, qword [rbx + rcx * 8 - 8]\n\tmov rcx, @stack_size\n\tdec qword [rcx]\n\tret\n";
const ASM_CODE_BEGIN_LINUX: &str = "section .import\n\textern printf\n\textern exit\n\nsection .data\n\t@int_fmt: db \"%lli \", 0\n\t@bytes_fmt: db \"%.8s\", 0\n\t\n\t@second_stack: times 1024 dq 0\n\t@stack_size: dq 0\n\nsection .text\n\tglobal main\n\nmain:\n\tjmp start\n\n@share_to_second_stack:\n\t; arg - rax\n\tmov rbx, @second_stack\n\tmov rcx, @stack_size\n\tmov rcx, [rcx]\n\tmov qword [rbx + rcx * 8], rax\n\tmov rcx, @stack_size\n\tinc qword [rcx]\n\tret\n\n@get_pop_second_stack:\n\t; return - rax\n\tmov rbx, @second_stack\n\tmov rcx, @stack_size\n\tmov rcx, [rcx]\n\tmov rax, qword [rbx + rcx * 8 - 8]\n\tmov rcx, @stack_size\n\tdec qword [rcx]\n\tret\n";
// |
const STACK_HEAD_ASM: &str = "\tpush rsp ; stack_head\n";
// |
const READ_FROM_ASM: &str = "\tpop rax ; read_from\n\tpush qword [rax]\n";
const WRITE_TO_ASM: &str = "\tpop rax ; write_to\n\tpop rbx\n\tmov qword [rbx], rax\n";
// |
const DUP_ASM: &str = "\tpush qword [rsp] ; dup\n";
const POP_ASM: &str = "\tadd rsp, 8\n";
// |
const EQ_ASM: &str = "\tpop rax ; eq\n\tpop rbx\n\tcmp rbx, rax\n\tsete al\n\tand rax, 1\n\tpush rax\n";
const NEQ_ASM: &str = "\tpop rax ; neq\n\tpop rbx\n\tcmp rbx, rax\n\tsetne al\n\tand rax, 1\n\tpush rax\n";
// |
const MORE_ASM: &str = "\tpop rax ; more\n\tpop rbx\n\tcmp rbx, rax\n\tsetg al\n\tand rax, 1\n\tpush rax\n";
const LESS_ASM: &str = "\tpop rax ; less\n\tpop rbx\n\tcmp rbx, rax\n\tsetl al\n\tand rax, 1\n\tpush rax\n";
// |
// const INC_ASM: &str = "\tpop rax ; inc\n\tinc rax\n\tpush rax\n"; unoptimized
// const DEC_ASM: &str = "\tpop rax ; dec\n\tdec rax\n\tpush rax\n"; unoptimized
const INC_ASM: &str = "\tinc qword [rsp] ; inc\n";
const DEC_ASM: &str = "\tdec qword [rsp] ; dec\n";
// |
// const SUM_ASM: &str = "\tpop rax ; sum\n\tpop rbx\n\tadd rbx, rax\n\tpush rbx\n"; unoptimized
// const DIF_ASM: &str = "\tpop rax ; dif\n\tpop rbx\n\tsub rbx, rax\n\tpush rbx\n"; unoptimized
const SUM_ASM: &str = "\tpop rax ; sum\n\tadd qword [rsp], rax\n";
const DIF_ASM: &str = "\tpop rax ; dif\n\tsub qword [rsp], rax\n";
// |
const MUL_ASM: &str = "\tpop rax ; mul\n\tpop rbx\n\tmul rbx\n\tpush rax\n";
const DIV_ASM: &str = "\tpop rbx ; div\n\tpop rax\n\tdiv rbx\n\tpush rax\n";
// |
const PRINT_ASM_WIN64: &str = "\tlea rcx, [rel @int_fmt] ; print\n\tpop rdx\n\tsub rsp, 32\n\tcall printf\n\tadd rsp, 32\n";
const WRITE_RAW_ASM_WIN64: &str = "\tlea rcx, [rel @bytes_fmt] ; write\n\tmov rdx, rsp\n\tsub rsp, 56\n\tcall printf\n\tadd rsp, 64\n"; // "sub rsp, 56" = "add rsp, 8"(pop) + "sub rsp, 64"
const EXIT_ASM_WIN64: &str = "\tpop rcx\n\tcall exit\n";
const SUCCESFUL_EXIT_ASM_WIN64: &str = "\tadd rsp, 32\n\txor rcx, rcx\n\tcall exit\n";
// |
const PRINT_ASM_LINUX: &str = "\txor rax, rax ; print\n\tlea rdi, [rel @int_fmt]\n\tpop rsi\n\tsub rsp, 32\n\tcall printf\n\tadd rsp, 32\n";
const WRITE_RAW_ASM_LINUX: &str = "\txor rax, rax ; print\n\tlea rdi, [rel @bytes_fmt]\n\tpop rsi\n\tsub rsp, 56\n\tcall printf\n\tadd rsp, 64\n"; // "sub rsp, 56" = "add rsp, 8"(pop) + "sub rsp, 64"
const EXIT_ASM_LINUX: &str = "\tpop rdi\n\tcall exit\n";
const SUCCESFUL_EXIT_ASM_LINUX: &str = "\txor rdi, rdi\n\tcall exit\n";

fn generate_swap_asm(first_argument: usize, second_argument: usize) -> String {
    format!("\tmov rax, qword [rsp + {first}] ; swap<{first_argument},{second_argument}>\n\tmov rbx, qword [rsp + {second}]\n\tmov qword [rsp + {first}], rbx\n\tmov qword [rsp + {second}], rax\n", first = first_argument * 8,  second = second_argument * 8)
}

#[derive(Eq, PartialEq)]
#[derive(Hash)]
enum StateType {
    Integer,
    StackHead,
    ReadFrom,
    WriteTo,
    Eq,
    Neq,
    More,
    Less,
    Dup,
    Pop,
    Inc,
    Dec,
    Sum,
    Dif,
    Mul,
    Div,
    If,
    Else,
    Swap,
    SelfCall,
    SelfGoto,
    Print,
    WriteRaw,
    Exit,
    Additional,
}

type StackValueType = i64;
// const STACK_VALUE_SIZE: usize = size_of::<StackValueType>();
const STACK_VALUE_SIZE_AS_SV: StackValueType = size_of::<StackValueType>() as StackValueType;

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
            StateType::Eq =>        StateType::Eq,
            StateType::Neq =>       StateType::Neq,
            StateType::More =>      StateType::More,
            StateType::Less =>      StateType::Less,
            StateType::Dup =>       StateType::Dup,
            StateType::Pop =>       StateType::Pop,
            StateType::Inc =>       StateType::Inc,
            StateType::Dec =>       StateType::Dec,
            StateType::Sum =>       StateType::Sum,
            StateType::Dif =>       StateType::Dif,
            StateType::Mul =>       StateType::Mul,
            StateType::Div =>       StateType::Div,
            StateType::If =>        StateType::If,
            StateType::Else =>      StateType::Else,
            StateType::Swap   =>    StateType::Swap,
            StateType::SelfCall =>  StateType::SelfCall,
            StateType::SelfGoto =>  StateType::SelfGoto,
            StateType::Print =>     StateType::Print,
            StateType::WriteRaw =>  StateType::WriteRaw,
            StateType::Exit =>      StateType::Exit,
            StateType::Additional =>StateType::Additional,
        }
    }
}

#[derive(Clone)]
#[derive(Default)]
struct State {
    name: String,
    deps: Vec<usize>, // deps id`s
    state_type: StateType,
    inlinable: bool, // inlinable states cannot contain 'if', 'else', '__self__', '__self__goto__'
    template_arguments_index: usize,
}

fn numeric_state(str: String) -> State {
    State{name: str, state_type: StateType::Integer, deps: Vec::<usize>::new(), inlinable: false, template_arguments_index: 0 }
}
fn separate_template_and_name(str: &String) -> (String, String) {
    if let Some(start_pos) = str.bytes().enumerate().position(|x| x.1 == '<' as u8) {
       (str[..start_pos].to_string(), str[start_pos + 1..str.len() - 1].to_string())
    } else  {
        (str.clone(), "".to_string())
    }
    
}
fn execute_statement(states: &Vec<State>, template_arguments: &Vec<Vec<String>>, state: &State, stack: &mut Vec::<StackValueType>) -> Option<()> {
    match state.state_type {
        StateType::Integer => {
            stack.push(state.name.parse::<StackValueType>().ok()?);
        }
        StateType::StackHead => {
            let stack_len = stack.len();
            stack.push((stack_len as StackValueType) * STACK_VALUE_SIZE_AS_SV);
        }
        StateType::ReadFrom => {
            panic!("read_from not allowed in interpriter mode");
        }
        StateType::WriteTo => {
            panic!("write_to not allowed in interpriter mode");
        }
        StateType::Eq => {
            let first_argument = stack.pop().expect("stack is empty on eq");
            let second_argument = stack.pop().expect("stack is empty on eq");
            stack.push((second_argument == first_argument) as StackValueType);
        }
        StateType::Neq => {
            let first_argument = stack.pop().expect("stack is empty on neq");
            let second_argument = stack.pop().expect("stack is empty on neq");
            stack.push((second_argument == first_argument) as StackValueType);
        }
        StateType::More => {
            let first_argument = stack.pop().expect("stack is empty on more");
            let second_argument = stack.pop().expect("stack is empty on more");
            stack.push((second_argument > first_argument) as StackValueType);
        }
        StateType::Less => {
            let first_argument = stack.pop().expect("stack is empty on less");
            let second_argument = stack.pop().expect("stack is empty on less");
            stack.push((second_argument < first_argument) as StackValueType);
        }
        StateType::Dup => {
            let last = stack.last().ok_or_else(|| "stack is empty on dup".to_string()).ok()?.clone();
            if state.template_arguments_index == 0 {
                stack.push(last);
            } else {
                let my_template_arguments = &template_arguments[state.template_arguments_index];
                let dup_count = my_template_arguments[0].parse::<StackValueType>().ok()?;
                for _i in 0..dup_count {
                    stack.push(last);
                }
            }
        }
        StateType::Pop => {
            if state.template_arguments_index == 0 {
                stack.pop().expect("stack is empty on pop");
            } else {
                let my_template_arguments = &template_arguments[state.template_arguments_index];
                let pop_count = my_template_arguments[0].parse::<StackValueType>().ok()?;
                for _i in 0..pop_count {
                    stack.pop().expect("stack is empty on pop");
                }
            }
        }
        StateType::Inc => {
            *stack.last_mut().expect("stack is empty on inc") += 1;
        }
        StateType::Dec => {
            *stack.last_mut().expect("stack is empty on dec") -= 1;
        }
        StateType::Sum => {
            let first_argument = stack.pop().expect("stack is empty on sum");
            let second_argument = stack.pop().expect("stack is empty on sum");
            stack.push(second_argument + first_argument);
        }
        StateType::Dif => {
            let first_argument = stack.pop().expect("stack is empty on dif");
            let second_argument = stack.pop().expect("stack is empty on dif");
            stack.push(second_argument - first_argument);
        }
        StateType::Mul => {
            let first_argument = stack.pop().expect("stack is empty on mul");
            let second_argument = stack.pop().expect("stack is empty on mul");
            stack.push(second_argument * first_argument);
        }
        StateType::Div => {
            let first_argument = stack.pop().expect("stack is empty on div");
            let second_argument = stack.pop().expect("stack is empty on div");
            stack.push(second_argument / first_argument);
        }
        StateType::Swap => {
            if state.template_arguments_index == 0 {
                let end_index = stack.len() - 1;
                let end_prev_index = stack.len() - 2;
                stack.swap(end_index, end_prev_index);
            } else {
                let my_template_arguments = &template_arguments[state.template_arguments_index];
                let first_index = stack.len() - 1 - my_template_arguments[0].parse::<usize>().ok()?;
                let second_index = stack.len() - 1 - my_template_arguments[1].parse::<usize>().ok()?;
                stack.swap(first_index, second_index);
            }
        }
        StateType::Print => {
            let last = stack.pop().expect("stack is empty on print");
            print!("{} ", last);
        }
        StateType::WriteRaw => {
            let value = stack.pop().expect("stack is empty on write_raw");
            std::io::stdout().write(value.to_be_bytes().as_slice()).expect("error on write_raw");
        }
        StateType::Exit => {
            let value = stack.pop().expect("stack is empty on exit");
            exit(value as i32);
        }
        _ => {
            let mut else_count = 0;
            let mut if_count = 0;
            for i in state.deps.iter() {
                let dep =  states.get(*i).expect("invalid statement index");
                if dep.state_type == StateType::Else {
                    else_count += 1;
                } else if dep.state_type == StateType::If {
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
                let dep =  states.get(*i).expect("invalid statement index");
                if dep.state_type ==  StateType::If {
                    let last = stack.pop()?;
                    if last <= 0 {
                        if_happens = true;
                    }
                }
                if if_happens  {
                    if dep.state_type ==  StateType::Else {
                        if_happens = false;
                    }
                } else if dep.state_type ==  StateType::Else {
                    break;
                } else if dep.state_type == StateType::SelfCall {
                    execute_statement(states, template_arguments, state, stack)?;
                } else if dep.state_type == StateType::SelfGoto {
                    iter =  state.deps.iter();
                    continue;
                } else {
                    execute_statement(states, template_arguments, dep, stack)?;
                }
                iter.next();
            }
        }
    }
    Some(())
}
fn is_inlinable(states: &Vec<State>, state: &State) -> bool {
    if state.name == "start" {
        return false
    }
    for i in state.deps.iter().enumerate() {
        let dep =  states.get(*i.1).expect("invalid statement index");
        if  (dep.state_type == StateType::If) || 
            (dep.state_type == StateType::Else) || 
            (dep.state_type == StateType::SelfCall) ||
            (dep.state_type == StateType::SelfGoto) {
                return false
        }
    }
    return true
}
fn compile_statement(states: &Vec<State>, template_arguments: &Vec<Vec<String>>, state: &State) -> Option<String> {
    let mut map = HashMap::new();
    map.insert(StateType::StackHead,    STACK_HEAD_ASM);
    map.insert(StateType::ReadFrom,     READ_FROM_ASM);
    map.insert(StateType::WriteTo,      WRITE_TO_ASM);
    map.insert(StateType::Eq,           EQ_ASM);
    map.insert(StateType::Neq,          NEQ_ASM);
    map.insert(StateType::More,         MORE_ASM);
    map.insert(StateType::Less,         LESS_ASM);
    map.insert(StateType::Inc,          INC_ASM);
    map.insert(StateType::Dec,          DEC_ASM);
    map.insert(StateType::Sum,          SUM_ASM);
    map.insert(StateType::Dif,          DIF_ASM);
    map.insert(StateType::Mul,          MUL_ASM);
    map.insert(StateType::Div,          DIV_ASM);

    if cfg!(target_os = "windows") {
        map.insert(StateType::Print,        PRINT_ASM_WIN64);
        map.insert(StateType::WriteRaw,     WRITE_RAW_ASM_WIN64);
        map.insert(StateType::Exit,         EXIT_ASM_WIN64);
    } else {
        map.insert(StateType::Print,        PRINT_ASM_LINUX);
        map.insert(StateType::WriteRaw,     WRITE_RAW_ASM_LINUX);
        map.insert(StateType::Exit,         EXIT_ASM_LINUX);
    }

    // template commands check
    match state.state_type {
        StateType::Pop => {
            if state.template_arguments_index == 0 {
                return Some(POP_ASM.to_string());
            } else {
                let my_template_arguments = &template_arguments[state.template_arguments_index];
                let pop_count = my_template_arguments[0].parse::<usize>().expect("template argement in pop must be integer");
                return Some(POP_ASM.repeat(pop_count));
            }
        }, StateType::Dup => {
            if state.template_arguments_index == 0 {
                return Some(DUP_ASM.to_string());
            } else {
                let my_template_arguments = &template_arguments[state.template_arguments_index];
                let dup_count = my_template_arguments[0].parse::<usize>().expect("template argement in pop must be integer");
                return Some(DUP_ASM.repeat(dup_count));
            }
        }, StateType::Swap => {
            if state.template_arguments_index == 0 {
                return Some(generate_swap_asm(0, 1));
            } else {
                let my_template_arguments = &template_arguments[state.template_arguments_index];
                let first_index = my_template_arguments.get(0).expect("too few template arguments in swap").parse::<usize>().ok()?;
                let second_index = my_template_arguments.get(1).expect("too few template arguments in swap").parse::<usize>().ok()?;
                return Some(generate_swap_asm(first_index, second_index));
            }
        },
        _ => { },
    }

    if let Some(asm) = map.get(&state.state_type) {
        return Some(asm.to_string());
    }
    if state.state_type == StateType::Integer {
        let integer = state.name.parse::<i64>().expect("invalid integer");
        if integer > i32::MAX as i64 {
            return Some(format!(BIG_INT_FMT!(), state.name));
        } else {
            return Some(format!(INT_FMT!(), state.name));
        }
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
            let dep = states.get(*i).expect("invalid statement index");
            
            if dep.inlinable || dep.state_type == StateType::Integer {
                out += compile_statement(states, template_arguments, dep).expect("compiling error").as_str();
            } else {
                out += "\tcall ";
                out += dep.name.as_str();
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
                let dep = states.get(*i).expect("invalid statement index");
                if dep.state_type == StateType::Else {
                    else_count += 1;
                } else if dep.state_type == StateType::If {
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
            let dep = states.get(*i).expect("invalid statement index");
            
            if dep.state_type ==  StateType::If {
                if_count += 1;
                if_stack.push(if_count);
                out += "\tpop rax\n";
                out += "\tcmp rax, 0\n";
                out += format!("\tjle @{}_else{}\n", state.name, if_count).as_str();
                
            } else if dep.state_type ==  StateType::Else {
                out += format!("{}@{}_else{}:\n", statement_exit, state.name, if_stack.pop().expect("unexpected else")).as_str();

            } else if dep.state_type == StateType::SelfCall {
                out += "\tcall ";
                out += state.name.as_str();
                out += "\n";

            } else if dep.state_type == StateType::SelfGoto {
                out += "\tjmp @";
                out += state.name.as_str();
                out += "_jump_position_you_know\n";

            } else {
                if dep.inlinable || dep.state_type == StateType::Integer {
                    out += compile_statement(states, template_arguments, dep).expect("compilation error").as_str();
                } else {
                    out += "\tcall ";
                    out += dep.name.as_str();
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
        State{name: "stack_head".to_string(),       state_type: StateType::StackHead,   deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "read_from".to_string(),        state_type: StateType::ReadFrom,    deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "write_to".to_string(),         state_type: StateType::WriteTo,     deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "print".to_string(),            state_type: StateType::Print,       deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "write_raw".to_string(),        state_type: StateType::WriteRaw,    deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "eq".to_string(),               state_type: StateType::Eq,          deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "neq".to_string(),              state_type: StateType::Neq,         deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "more".to_string(),             state_type: StateType::More,        deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "less".to_string(),             state_type: StateType::Less,        deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "dup".to_string(),              state_type: StateType::Dup,         deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "pop".to_string(),              state_type: StateType::Pop,         deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "sum".to_string(),              state_type: StateType::Sum,         deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "dif".to_string(),              state_type: StateType::Dif,         deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "mul".to_string(),              state_type: StateType::Mul,         deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "div".to_string(),              state_type: StateType::Div,         deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "inc".to_string(),              state_type: StateType::Inc,         deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "dec".to_string(),              state_type: StateType::Dec,         deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "if".to_string(),               state_type: StateType::If,          deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "else".to_string(),             state_type: StateType::Else,        deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "swap".to_string(),             state_type: StateType::Swap,        deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "__self__".to_string(),         state_type: StateType::SelfCall,    deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "__self__goto__".to_string(),   state_type: StateType::SelfGoto,    deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
        State{name: "exit".to_string(),             state_type: StateType::Exit,        deps: Vec::new(), inlinable: true, template_arguments_index: 0 },
    ];
    let mut template_arguments: Vec<Vec<String>> = vec![vec![String::new()]];
    let mut last_state = State::default();
    let tokens = code.split_whitespace();

    let mut state_colon = false;
    let mut in_state = false;
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
        
        if in_state {
            last_state.name = i.1.to_string();
            if last_state.name.get(0..1).expect("too small statement name") == "@" {
                panic!("the first character of the state name cannot be '@'");
            }
            in_state = false;
        } else if state_colon {
            if i.1 == ";" {
                state_colon = false;
                // inline-check
                if is_inlinable(&states, &last_state) {
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
                last_state.deps.push(states.len());
                states.push(numeric_state(num.to_string()));
                continue;
            }
            let template_separated_name = separate_template_and_name(&i.1.to_string());
            if template_separated_name.1.is_empty() {
                last_state.deps.push(
                    states
                        .iter()
                        .position (|x| x.name == i.1 )
                        .expect(&format!("invalid statement \"{}\"", i.1)))
            } else {
                if let Some(template_statement) =
                    states
                    .iter()
                    .find(|x| x.name == template_separated_name.0){
                    
                    last_state.deps.push(states.len());

                    let current_template_arguments_index = template_arguments.len();
                    template_arguments.push(template_separated_name.1.split(',').map(|x| x.to_string()).collect::<Vec<_>>());

                    states.push(State{name: template_separated_name.0, state_type: template_statement.state_type.clone(), deps: template_statement.deps.clone(), inlinable: template_statement.inlinable, template_arguments_index: current_template_arguments_index });                                      
                } else {
                    panic!("{} statement doenst exist", template_separated_name.0);
                }
            }

        } else if i.1 == "st" {
            in_state = true;
        } else if i.1 == ":" {
            state_colon = true;
        }
    }

    if let Some(mode) = mode {
        if mode == "i" {
            let mut stack = Vec::<StackValueType>::new();
            execute_statement(&states, &template_arguments, states.iter().enumerate().find(|x| x.1.name == "start").expect("entry point \"start\" doesnt exist").1, &mut stack);
            return;
        } else if mode != "c" {
            panic!("unknown mode. check --help")
        }
    }

    let asm_code_begin;
    if cfg!(target_os = "windows") {
        asm_code_begin = ASM_CODE_BEGIN_WIN64
    } else {
        asm_code_begin = ASM_CODE_BEGIN_LINUX
    };

    let mut compiled_code = format!("{}",asm_code_begin);
    for i in states.iter().filter(|x| !x.inlinable).enumerate() {
        compiled_code += compile_statement(&states, &template_arguments, i.1).expect("compilation error").as_str();
    }

    if let Some(output_file) = output_file {
        let output_nasm_file_string = output_file.to_string() + ".nasm";
        let output_obj_file_string = output_file.to_string() + ".o";
        let output_result_file_string;
        if cfg!(target_os = "windows") {
            output_result_file_string = output_file.to_string() + ".exe";
        } else {
            output_result_file_string = output_file.to_string();
        };
        std::fs::write(output_nasm_file_string.clone(), compiled_code).expect("unable to write file");
        if cfg!(target_os = "windows") {
            if let Err(error_nasm) = std::process::Command::
                new("nasm").
                arg("-fwin64").
                arg(output_nasm_file_string).
                arg("-o").
                arg(output_obj_file_string.clone()).
                spawn() {
                    eprintln!("{}", error_nasm);
                }
                
            if let Err(error_gcc) = std::process::Command::
                new("gcc").
                arg(output_obj_file_string).
                arg("-o").
                arg(output_result_file_string).
                spawn() {
                    eprintln!("{}", error_gcc);
                }
        } else {
            if let Err(error_nasm) = std::process::Command::
                new("nasm").
                arg("-felf64").
                arg(output_nasm_file_string).
                arg("-o").
                arg(output_obj_file_string.clone()).
                spawn() {
                    eprintln!("{}", error_nasm);
                }
                
            if let Err(error_gcc) = std::process::Command::
                new("gcc").
                arg("-no-pie").
                arg(output_obj_file_string).
                arg("-o").
                arg(output_result_file_string).
                spawn() {
                    eprintln!("{}", error_gcc);
                }
        }

    } else {
        println!("{}", compiled_code);
    }
}
