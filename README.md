# stack_sharp
an interpreter for a stack-based programming language written in rust

# Documentation:

## Syntax
stack_sharp uses reverse polish notation, being heavily inspired by Forth.
All functions use parameters loaded onto the stack and generally output back onto it.
For example `5 3 +` would push the number `5` on to the stack, then the number `3`, then `+` would pop the two top items on the stack and add them and push the result onto the stack. So this expression would be equivalent to just pushing an `8` onto the stack

### built in operators/functions
All the following functions use up their parameter(s) and those that calculate a result push it onto the stack:
Arithmetic:
- integer - pushes the integer to the stack, can be negative
- `+` - adds the top 2 numbers
- `-` - subtracts the top number from the next number
- `*` - multiplies the top 2 numbers
- `/` - divides the second top number by the top number
- `%` - modulos the second top number by the top number
- `~` - negates the top number (eg. `5 ~` -> `-5`)
- `&` - binary ands the top 2 numbers
- `|` - binary ors the top 2 numbers
- `^` - binary xors the top 2 numbers
- `!` - binary nots the top number#

- `@` - 