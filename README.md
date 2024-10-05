# jalgo
A WEIRD stack-based programming language

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
- [Command-Line Interface](#command-line-interface)
- [Examples](#examples)
  - [Sum of Numbers](#sum-of-numbers)
  - [Factorial](#factorial)
  - [Sum of Squares](#sum-of-squares)
  - [Negative Check](#negative-check)
  - [Fibonacci Sequence](#fibonacci-sequence)
    - [Recursive](#recursive)
    - [Iterative](#iterative)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Welcome to jalgo, a stack-based programming language that's designed to make you question your life choices. With jalgo, recursion is handled by the magical `__self__` keyword, and you can't call a function that hasn't been defined yet. Because who needs forward declarations, right?

**Disclaimer:** I'm not even sure if this language is Turing complete. But who cares about such trivial details?

## Features

- **Stack-Based**: Everything is a stack. Because why not?
- **Recursion**: You can call yourself with `__self__`. Isn't that cute?
- **Iteration**: You can restart the current expression with `__self__goto__`. Because who needs loops?
- **Conditional Statements**: `if` and `else` are here to make your life a living hell.
- **Basic Operations**: `print`, `pop`, `sum`, `dif`, `mul`, `div`, `dup`, `swap`, `swap0_2`. Because who needs more?

## Getting Started

To get started, you'll need to write some code. Here's a simple example to calculate the factorial of a number:

```
st factorial_loop : swap dup if dup swap0_2 mul swap 1 dif swap __self__goto__ else pop ;
st factorial : 1 factorial_loop ;

st start : 10 factorial print ;
```

Yeah, it's that simple.

## Command-Line Interface

The command-line interface for jalgo is as follows:

```
jalgo 1.0
Aidar Shigapov
compiler/interpreter for jango language

USAGE:
    jalgo [OPTIONS] <input> [output] <mode>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <input>    sets the input file to use
    <output>   sets the output file to use
    <mode>     sets the interprutation/compilation mode.
               posible values: c | i
```

The `input` argument is required and specifies the input file to use. The `output` argument is optional and specifies the output file to use. The `mode` argument is required and specifies the interprutation/compilation mode. Possible values are `c` for compilation and `i` for interprutation.

Who needs more options?

## Examples

### Sum of Numbers

Here's how you can calculate the sum of numbers from 1 to N:

```
st sum_of_loop : dup if dup swap0_2 sum swap 1 dif __self__goto__ else pop ;
st sum_of : 0 swap sum_of_loop ;
```

### Factorial

Here's how you can calculate the factorial of a number:

```
st factorial_loop : dup if dup swap0_2 mul swap 1 dif __self__goto__ else pop ;
st factorial : 1 swap factorial_loop ;
```

### Sum of Squares

Here's how you can calculate the sum of squares from 1 to N:

```
st sum_squares_loop : dup if dup swap0_2 swap dup mul sum swap 1 dif __self__goto__ else pop ;
st sum_squares : 0 swap sum_squares_loop ;
```

### Is negative
```
st is_negative : if 0 else 1 ; 
```

### Fibonacci Sequence

#### Recursive

Here's how you can calculate the Fibonacci sequence using recursion:

```
st fibonacci_recursion : dup 1 dif is_negative if pop 0 else dup 2 dif is_negative if pop 1 else 1 dif dup 1 dif __self__ swap __self__ sum ;
```

Calculating the 46th Fibonacci number took me 7 minutes, so just don't use recursion unless you're feeling particularly masochistic.

#### Iterative

Here's how you can calculate the Fibonacci sequence using iteration:

```
st fibonacci_iteration_loop : swap0_2 dup if 1 dif swap0_2 dup swap0_2 sum __self__goto__ else pop swap pop ;
st fibonacci_iteration : dup 1 dif is_negative if pop 0 else 1 dif 0 1 fibonacci_iteration_loop ;
```

### Start Example

Here's an example that combines multiple functions:

```
st start : 9 sum_of print 9 factorial print 9 sum_squares print 46 fibonacci_iteration print 35 fibonacci_recursion print ;
```

(In interprutation mode) It might be a quirk of the Rust, but print only works after all the called states have finished executing—but it does work correctly. So, if you want to see the result of sum_of, you'll have to wait for fibonacci_recursion to finish, which could take a while. Grab a coffee, maybe two.

## Contributing

Feel free to contribute to this **`language`**. Just make sure your code is as obscure and confusing as possible.

## License

This project is licensed under the [MIT license](license) (why not WTFPL? idk)
