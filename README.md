# stack_sharp
an interpreter for a stack-based programming language written in rust

# Documentation:

## Syntax
stack_sharp uses reverse polish notation, being heavily inspired by Forth.
All functions use parameters loaded onto the stack and generally output back onto it.
For example `5 3 +` would push the number `5` on to the stack, then the number `3`, then `+` would pop the two top items on the stack and add them and push the result onto the stack. So this expression would be equivalent to just pushing an `8` onto the stack.

Function notation in stack_sharp is also similar to Forth, for example:<br>
`a b c -- a c d` - the parameters are `a`, `b` and `c`, the parameter `a` is left untouched, the parameter `b` is consumed and the parameter `c` drops back to be next to `a`, and finally the result `d` is pushed onto the stack.<br>
An word on the right hand side that is in brackets generally conveys that a result has been pushed somewhere other than the stack (eg. is has been outputted in the console).<br>
A `..` represents an arbitrary number of parameters of results and `...` represents all the previous items in the stack.

### built in operators/functions
All the following functions use up their parameter(s) and those that calculate a result push it onto the stack:

**Arithmetic:**
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

**Stack Manipulation:**
- `>>`(dup) - pops the top number then:
    - if the number is > 0, it copies the new top number that many times without consuming it, eg. `45 2 >>` -> `45 45 45`
    - if the number is < 0, it removes that many items from the stack, eg. `1 2 3 4 5 -3 >>` -> `1 2`
    - if the number is = 0, it clears the stack, eg. `... 1 2 3 4 5 -1 >>` -> ` `
        - **Try not to use this outside of testing, especially within a function**
- `<>`(swap) - pops the top number, `n`, then swaps the next top number with the number `n` numbers behind it, starting at 0, eg. `1 2 3 4 5 2 <>` -> `1 5 3 4 2`, `1 2 0 <>` -> `2 1`

**Testing:**

Boolean values in stack_sharp are stored as `0` for false and `-1` for true (as this is equivalent to all ones in binary), however, any non-zero number will be considered true
- `@` - tests if the top number is equal to 0 and pushes the boolean result
- `@+` - tests if the top number is greater than 0 and pushes the boolean result



*this section is still incomplete*