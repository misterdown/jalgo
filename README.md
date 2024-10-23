# jalgo
A WEIRD stack-based programming language that will make you question your life choices

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
- [Command-Line Interface](#command-line-interface)
- [Examples](#examples)
  - [Sum of Numbers](#sum-of-numbers)
  - [Factorial](#factorial)
  - [Sum of Squares](#sum-of-squares)
  - [Fibonacci Sequence](#fibonacci-sequence)
    - [Recursive](#recursive)
    - [Iterative](#iterative)
  - [Rule 110](#rule-110)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Welcome to jalgo, a stack-based programming language that's designed to make you question your life choices. With jalgo, recursion is handled by the magical `__self__` keyword, and you can't call a function that hasn't been defined yet. Because who needs forward declarations, right?

**Disclaimer:** I'm not even sure if this language is Turing complete. But who cares about such trivial details?

**Update:** Turns out, jalgo is Turing complete. So, you can now officially question your life choices with confidence.

## Features

- **Stack-Based**: Everything is a stack. Because why not?
- **Recursion**: You can call yourself with `__self__`. Isn't that cute?
- **Iteration**: You can restart the current expression with `__self__goto__`. Because who needs loops?
- **Conditional Statements**: `if` and `else` are here to make your life a living hell.
- **Basic Operations**: `print`, `write_raw`, `exit`, `pop`, `sum`, `dif`, `mul`, `div`, `dup`, `swap`, `swap<x,y>`, `inc`, `dec`, `eq`, `neq`, `more`, `less`, `stack_head`, `read_from`, `write_to`. Who needs more?
- **Stack Management**: `stack_head` returns a pointer to the top of the stack.
- **Memory Operations**: `write_to`( != write_raw) writes a value to a memory location pointed to by the top value on the stack. Syntax: `POINTER VALUE write_to`. `read_from` reads a value from a memory location pointed to by the top value on the stack. Syntax: `POINTER read_from`. Who needs fancy data structures?
- **Template Commands**: Some commands now support templates, similar to C++ or rust. Yeah, I fell in love with the templates. `pop<n>`, `dup<n>`, and `swap<x,y>`

## Getting Started

To get started, you'll need to write some code. Here's a simple example to calculate the factorial of a number:

```jalgo
st factorial_loop : swap dup if dup swap<0,2> mul swap 1 dif swap __self__goto__ else pop ;
st factorial : 1 factorial_loop ;

st start : 10 factorial print ;
```

Yeah, it's that simple.

## Command-Line Interface

The command-line interface for jalgo is as follows:

```
. . .

ARGS:
    <input>    sets the input file to use
    <output>   sets the output file to use
    <mode>     sets the interpretation/compilation mode.
               possible values: c | i
```

The `input` argument is required and specifies the input file to use. The `output` argument is optional and specifies the output file to use. The `mode` argument is required and specifies the interpretation/compilation mode. Possible values are `c` for compilation jalgo code into asm (NASM), asm code to executable and `i` for interpretation.

## Examples

### Sum of Numbers

Here's how you can calculate the sum of numbers from 1 to N:

```jalgo
st sum_of_loop : dup if dup swap<0,2> sum swap 1 dif __self__goto__ else pop ;
st sum_of : 0 swap sum_of_loop ;
```

### Factorial

Here's how you can calculate the factorial of a number:

```jalgo
st factorial_loop : dup if dup swap<0,2> mul swap 1 dif swap __self__goto__ else pop ;
st factorial : 1 swap factorial_loop ;
```

### Sum of Squares

Here's how you can calculate the sum of squares from 1 to N:

```jalgo
st sum_squares_loop : dup if dup swap<0,2> swap dup mul sum swap 1 dif __self__goto__ else pop ;
st sum_squares : 0 swap sum_squares_loop ;
```

### Fibonacci Sequence

#### Recursive

Here's how you can calculate the Fibonacci sequence using recursion:

```jalgo
st is_false : if 0 else 1 ;
st fibonacci_recursion : dup 1 dif is_false if pop 0 else dup 2 dif is_false if pop 1 else 1 dif dup 1 dif __self__ swap __self__ sum ;
```

Calculating the 46th Fibonacci number took me 7 minutes, so just don't use recursion unless you're feeling particularly masochistic.

#### Iterative

Here's how you can calculate the Fibonacci sequence using iteration:

```jalgo
st is_false : if 0 else 1 ;
st fibonacci_iteration_loop : swap<0,2> dup if 1 dif swap<0,2> dup swap<0,2> sum __self__goto__ else pop swap pop ;
st fibonacci_iteration : dup 1 dif is_false if pop 0 else 1 dif 0 1 fibonacci_iteration_loop ;
```

### Rule 110

Check [it](examples/rule110.jalgo)

## Contributing

Feel free to contribute to this **`language`**. Just make sure your code is as obscure and confusing as possible.

## License

This project is licensed under the [MIT license](license) (why not WTFPL? idk)
