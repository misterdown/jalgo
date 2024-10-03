/*
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
use std::result::*;

#[derive(PartialEq)]
enum StateType {
    Integer,
    Primary,
    Additional,
}
impl Default for StateType {
    fn default() -> Self {
        StateType::Additional
    }
}
impl Clone for StateType {
    fn clone(&self) -> Self {
        match *self {
            StateType::Primary => StateType::Primary,
            StateType::Additional => StateType::Additional,
            StateType::Integer => StateType::Integer,
            _ => StateType::Additional,
        }
    }
}

#[derive(Clone)]
#[derive(Default)]
struct State {
    name: String,
    state_type: StateType,
    deps: Vec<State>,
}

fn numeric_state(str: String) -> State {
    State{name: str, state_type: StateType::Integer, deps: Vec::<State>::new() }
}

fn execute_statement(state: &State, stack: &mut Vec::<i32>) -> Result::<bool, String> {
    if state.state_type == StateType::Integer {
        stack.push(state.name.parse::<i32>().expect("invalid integer"));
        return Ok(true)
    }
    if state.state_type == StateType::Primary {
        if state.name == "print" {
            let last = stack.pop().expect("stack is empty on print");
            print!("{} ", last);
            return Ok(true)
        } else if state.name == "sum" {
            let first_argument = stack.pop().expect("stack is empty on sum");
            *stack.last_mut().expect("stack is empty on sum") += first_argument;
            return Ok(true)
        } else if state.name == "pop" {
            stack.pop();
            return Ok(true)
        } else if state.name == "dif" {
            let first_argument = stack.pop().expect("stack is empty on dif");
            *stack.last_mut().expect("stack is empty on dif") -= first_argument;
            return Ok(true)
        } else if state.name == "mul" {
            let first_argument = stack.pop().expect("stack is empty on mul");
            *stack.last_mut().expect("stack is empty on mul") *= first_argument;
            return Ok(true)
        } else if state.name == "div" {
            let first_argument = stack.pop().expect("stack is empty on div").clone();
            *stack.last_mut().expect("stack is empty on div") /= first_argument;
            return Ok(true)
        }else if state.name == "dup" {
            stack.push(*stack.last().expect("stack is empty on dup"));
            return Ok(true)
        } else if state.name == "if" {
            let first_argument = stack.pop().expect("stack is empty on if").clone();
            if first_argument <= 0 {
                return Ok(false)
            }
            return Ok(true)
        } else if (state.name == "swap") || (state.name == "swap0_1") {
            let end_index = stack.len() - 1;
            let end_prev_index = stack.len() - 2;
            stack.swap(end_index, end_prev_index);
            return Ok(true)
        } else if state.name == "swap0_2" {
            let end_index = stack.len() - 1;
            let end_prev_prev_index = stack.len() - 3;
            stack.swap(end_index, end_prev_prev_index);
            return Ok(true)
        } else {
            return Err("unknown primary command ".to_string() + &state.name)
        }
    }
    let mut if_happens = false;

    let mut iter =  state.deps.iter();
    loop {

        let io = iter.as_slice().first();
        if io.is_none() {
            break;
        }
        let i = io.expect("");
        
        if if_happens  {
            if i.name == "else" {
                if_happens = false;
            }
        } else if i.name == "else" {
            break;
        } else if i.name == "__self__" {
            execute_statement(state, stack).expect("WROOONG");
        } else if i.name == "__self__goto__" {
            iter =  state.deps.iter();
            continue;
        } else if !execute_statement(i, stack).expect("WROOONG") {
            if_happens = true;
        }
        iter.next();
    }
    return Ok(true)
}

fn main() {
    let mut states: Vec<State> = vec![
        State{name: "print".to_string(), state_type: StateType::Primary, deps: Vec::<State>::new()},
        State{name: "pop".to_string(), state_type: StateType::Primary, deps: Vec::<State>::new()},
        State{name: "sum".to_string(), state_type: StateType::Primary, deps: Vec::<State>::new()},
        State{name: "dif".to_string(), state_type: StateType::Primary, deps: Vec::<State>::new()},
        State{name: "mul".to_string(), state_type: StateType::Primary, deps: Vec::<State>::new()},
        State{name: "div".to_string(), state_type: StateType::Primary, deps: Vec::<State>::new()},
        State{name: "dup".to_string(), state_type: StateType::Primary, deps: Vec::<State>::new()},
        State{name: "if".to_string(), state_type: StateType::Primary, deps: Vec::<State>::new()},
        State{name: "else".to_string(), state_type: StateType::Primary, deps: Vec::<State>::new()},
        State{name: "swap".to_string(), state_type: StateType::Primary, deps: Vec::<State>::new()},
        State{name: "swap0_1".to_string(), state_type: StateType::Primary, deps: Vec::<State>::new()},
        State{name: "swap0_2".to_string(), state_type: StateType::Primary, deps: Vec::<State>::new()},
        State{name: "__self__".to_string(), state_type: StateType::Primary, deps: Vec::<State>::new()},
        State{name: "__self__goto__".to_string(), state_type: StateType::Primary, deps: Vec::<State>::new()},

    ];
    /*
        3 0 1
        1 0 3
        1 0 2
        2 0 1
        2 0 1 1
        2 1 1 0
        2 1 1
     */
    let mut last_state = State::default();
    let code = "
        st sum_of_loop : swap dup if dup swap0_2 sum swap 1 dif swap __self__goto__ else pop ;
        st sum_of : 0 sum_of_loop ;

        st factorial_loop : swap dup if dup swap0_2 mul swap 1 dif swap __self__goto__ else pop ;
        st factorial : 1 factorial_loop ;

        st sum_squares_loop : swap dup if dup swap0_2 swap dup mul sum swap 1 dif swap __self__goto__ else pop ;
        st sum_squares : 0 sum_squares_loop ;

        st is_negative : if 0 else 1 ;

        st fibonacci_recursion : dup 1 dif is_negative if pop 0 else dup 2 dif is_negative if pop 1 else 1 dif dup 1 dif __self__ swap __self__ sum ;

        st fibonacci_iteration_loop : swap0_2 dup if 1 dif swap0_2 dup swap0_2 sum __self__goto__ else pop swap pop ;
        st fibonacci_iteration : dup 1 dif is_negative if pop 0 else 1 dif 0 1 fibonacci_iteration_loop ;

        st start : 9 sum_of print 9 factorial print 9 sum_squares print 46 fibonacci_iteration print 30 fibonacci_recursion print ;
        
    "

    // 3 2 2 1 
    //st start : 9 sum_of print 9 factorial print 9 sum_squares print 4 fibonacci_recursion print ;
    .to_string();
    let tokens = code.split(|x: char| x.is_whitespace());

    let mut state_colon = false; 
    let mut state_found = false; 
    for i in tokens.enumerate() {
        if state_found {
            last_state.name = i.1.to_string();
            state_found = false;
        } if state_colon {
            if i.1 == ";" {
                state_colon = false;
                states.push(last_state.clone());
                last_state = State::default();
                continue;
            }
            if i.1.parse::<i32>().is_ok()  {
                last_state.deps.push(numeric_state(i.1.to_string()));
                continue;
            }
            last_state.deps.push(
                states.iter().
                enumerate().
                find(|x| x.1.name == i.1.to_string()).
                expect(("invalid statement \"".to_string() + i.1 + "\"").as_str()).1.
                clone());
        }
            
        else if i.1 == "st" {
            state_found = true;
        } else if i.1 == ":" {
            state_colon = true;
        }
    }

    let mut stack = Vec::<i32>::new();
    for i in states.iter().enumerate() {
        if i.1.name == "start" {
            execute_statement(i.1, &mut stack).expect("Something wrong. . .");
        }
    }
}
