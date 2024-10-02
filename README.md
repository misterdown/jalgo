# jalgo
A WEIRD stack-based programming language

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Introduction

This language is designed to make you question your life choices. It's a stack-based language with a twist: recursion is handled by the magical `__self__` keyword, and you can't call a function that hasn't been defined yet. Because who needs forward declarations, right?

**Disclaimer 1:** This isn't even a fully-fledged program that can execute files with code. It's just a string handler embedded directly into the interpreter's source code. Because who needs convenience?

**Disclaimer 2:** I'm not even sure if this language is Turing complete. But who cares about such trivial details?

## Features

- **Stack-Based**: Everything is a stack. Because why not?
- **Recursion**: You can call yourself with `__self__`. Isn't that cute?
- **Conditional Statements**: `if` and `else` are here to make your life a living hell.
- **Basic Operations**: `print`, `pop`, `sum`, `dif`, `mul`, `div`, `dup`, `swap`, `swap0_2`. Because who needs more?

## Getting Started

To get started, you'll need to write some code. Here's a simple example to calculate the factorial of a number:

```
st factorial_loop : swap dup if dup swap0_2 mul swap 1 dif swap __self__ else pop ;
st factorial : 1 factorial_loop ;
st start : 10 factorial print ;
```

Yeah, it's that simple.

## Examples

### Sum of Squares

Here's how you can calculate the sum of squares from 1 to N:

```
st sum_squares_loop : swap dup if dup swap0_2 swap dup mul sum swap 1 dif swap __self__ else pop ;
st sum_squares : 0 sum_squares_loop ;
st start : 9 sum_squares print ;
```

A bit weird.

## Contributing

Feel free to contribute to this **`language`**. Just make sure your code is as obscure and confusing as possible.

## License

This project is licensed under the [MIT license](license) (why not WTFPL? idk )
