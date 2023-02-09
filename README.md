# stack_sharp
an interpreter for a stack-based programming language written in rust

# Documentation:

## Syntax
stack_sharp uses reverse polish notation, being heavily inspired by Forth.
All functions use parameters loaded onto the stack and generally output back onto it.
For example `5 3 +` would push the number `5` on to the stack, then the number `3`, then `+` would pop the two top items on the stack and add them and push the result onto the stack. So this expression would be equivalent to just pushing an `8` onto the stack

- 