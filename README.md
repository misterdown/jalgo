# jalgo
A WEIRD stack-based programming language that will make you question your life choices

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
- [Command-Line Interface](#command-line-interface)
- [Examples](#examples)
  - [Factorial](#factorial)
  - [Fibonacci Sequence](#fibonacci-sequence)
    - [Recursive](#recursive)
    - [Iterative](#iterative)
  - [Rule 110](#rule-110)
  - [Hello World](#hello-world)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Welcome to jalgo, a stack-based programming language that's designed to make you question your life choices. With jalgo, recursion is handled by the magical `__self__` keyword, and you can't call a function that hasn't been defined yet. Because who needs forward declarations, right?

**Disclaimer:** I'm not even sure if this language is Turing complete. But who cares about such trivial details?

**Update:** Turns out, jalgo is Turing complete. So, you can now officially question your life choices with confidence.

## Features

- **Stack-Based**: Everything is a stack.
- **Recursion**: You can call yourself with `__self__`. Isn't that cute?
- **Iteration**: You can restart the current expression with `__self__goto__`. Who needs loops?
- **Conditional Statements**: `if` and `else` are here to make your life a bit worse.
- **Basic Operations**: `print`, `write_raw`, `exit`, `pop`, `sum`, `dif`, `mul`, `div`, `dup`, `swap`, `swap<x,y>`, `inc`, `dec`, `eq`, `neq`, `more`, `less`, `stack_head`, `read_from`, `write_to`.
- **Stack Management**: `stack_head` returns a pointer to the top of the stack.
- **Memory Operations**: (not allowed in interpretation mode) `write_to` (not the same as `write_raw`) writes a value to a memory location pointed to by the top value on the stack. Syntax: `POINTER VALUE write_to`. `read_from` reads a value from a memory location pointed to by the top value on the stack. Syntax: `POINTER read_from`. Who needs fancy data structures?
- **Template Commands**: Some commands now support templates, similar to C++ or Rust. Yeah, I fell in love with the templates. `pop<n>`, `dup<n>`, and `swap<x,y>`.

## Getting Started

To get started, you'll need to write some code. Here's a simple example to print numbers from 9 to 0:

```jalgo
st print_numbers_until_zero :
    dup if
        print
        __self__goto__
    else
        print ;
st start : 0 1 2 3 4 5 6 7 8 9 print_numbers_until_zero ;
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

The `input` argument is required and specifies the input file to use. The `output` argument is optional and specifies the output file to use. The `mode` argument is required and specifies the interpretation/compilation mode. Possible values are `c` for compilation of jalgo code into asm (NASM), asm code to executable, and `i` for interpretation.

## Examples

### Factorial

Here's how you can calculate the factorial of a number:

```jalgo
st factorial_loop :
    dup if
        dup
        swap<0,2> mul
        swap 1 dif
        swap
        __self__goto__
    else
        pop ;
st factorial : 1 swap factorial_loop ;
```

### Fibonacci Sequence

#### Recursive

Here's how you can calculate the Fibonacci sequence using recursion:

```jalgo
st is_false : if 0 else 1 ;
st fibonacci_recursion :
    dup dec
    is_false if
        pop 0
    else
        dup 2 dif
        is_false if
            pop 1
        else
            dec
            dup dec
            __self__
            swap
            __self__
            sum ;
```

Calculating the 46th Fibonacci number took me 7 minutes, so just don't use recursion unless you're feeling particularly masochistic.

#### Iterative

Here's how you can calculate the Fibonacci sequence using iteration:

```jalgo
st is_false : if 0 else 1 ;
st fibonacci_iteration_loop :
    swap<0,2> dup if
        dec
        swap<0,2> dup
        swap<0,2> sum
        __self__goto__
    else
        pop swap pop ;
st fibonacci_iteration :
    dup dec is_false if
        pop 0
    else
        dec
        0 1 fibonacci_iteration_loop ;
```

### Rule 110

Check [it](examples/rule110.jalgo)

### Hello World

Oh, you wanted a ["Hello World"](examples/HelloWorld.jalgo) in jalgo? Well, jalgo doesn't have string literals.

## Contributing

Feel free to contribute to this **`language`**. Just make sure your code is as obscure and confusing as possible.

## License

This project is licensed under the [MIT license](LICENSE) (why not WTFPL? idk)
