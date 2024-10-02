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
            print!("{} ", stack.last().expect("stack is empty"));
            stack.pop();
            return Ok(true)
        } else if state.name == "sum" {
            let first_argument = stack.last().expect("stack is empty").clone();
            stack.pop();
            *stack.last_mut().expect("stack is empty") += first_argument;
            return Ok(true)
        } else if state.name == "pop" {
            stack.pop();
            return Ok(true)
        } else if state.name == "dif" {
            let first_argument = stack.last().expect("stack is empty").clone();
            stack.pop();
            *stack.last_mut().expect("stack is empty") -= first_argument;
            return Ok(true)
        } else if state.name == "mul" {
            let first_argument = stack.last().expect("stack is empty").clone();
            stack.pop();
            *stack.last_mut().expect("stack is empty") *= first_argument;
            return Ok(true)
        } else if state.name == "div" {
            let first_argument = stack.last().expect("stack is empty").clone();
            stack.pop();
            *stack.last_mut().expect("stack is empty") /= first_argument;
            return Ok(true)
        }else if state.name == "dup" {
            stack.push(*stack.last().expect("stack is empty"));
            return Ok(true)
        } else if state.name == "if" {
            let first_argument = stack.last().expect("stack is empty").clone();
            stack.pop();
            if first_argument <= 0 {
                return Ok(false)
            }
            return Ok(true)
        } else if state.name == "else" {
            let first_argument = stack.last().expect("stack is empty").clone();
            stack.pop();
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
    for i in state.deps.iter().enumerate() {
        if if_happens  {
            if i.1.name == "else" {
                if_happens = false;
            }
            continue;
        } else if i.1.name == "else" {
            break;
        } else if i.1.name == "__self__" {
            execute_statement(state, stack).expect("WROOONG");
        } else if !execute_statement(i.1, stack).expect("WROOONG") {
            if_happens = true;
        }
    }
    return Ok(true)
}
/*
    3 0
    0 3
    0 3 3
    3 3 0
    3 0 3
    3 0 3 3
    3 0 9
    3 9
    9 3
    9 3 1
    9 2
    2 9
*/
/*
    2 9
    9 2
    9 2 2
    2 2 9
    2 9 2
    2 9 2 2
    2 9 4
    2 13
    13 2
    13 2 1
    13 1
    1 13
*/
// , , ,
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

    ];
    let mut last_state = State::default();
    let code = "
        st factorial_loop : swap dup if dup swap0_2 mul swap 1 dif swap __self__ else pop ;
        st factorial : 1 factorial_loop ;

        st sum_loop : swap dup if dup swap0_2 sum swap 1 dif swap __self__ else pop ;
        st summ : 0 sum_loop ;

        st sum_squares_loop : swap dup if dup swap0_2 swap dup mul sum swap 1 dif swap __self__ else pop ;
        st sum_squares : 0 sum_squares_loop ;

        st start : 9 summ print 9 factorial print 9 sum_squares print ;
    "
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
