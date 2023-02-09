# stack_sharp
an interpreter for a stack-based programming language written in rust

# Documentation:

The notation `abc` -> `xyz` used later in this documentation means interchangeably that `abc` compiles to, is equivalent to or results in the output of `xyz`.

## Syntax
In stack_sharp, code is parsed word by word. With the exception of within a string, where the entire string, including the `"`s or `'`s, is treated as a single word, a word is a sequence of characters separated from other words by any of the following: whitespace, a newline, a block comment (`/* ... */`) or a line comment (`// ...`). Therefore, any brackets must be separated from what they are encapsulaating except in certain conditions (eg. function declarations and aliases).

stack_sharp uses reverse polish notation, being heavily inspired by Forth.
All functions use parameters loaded onto the stack and generally output back onto it.
For example `5 3 +` would push the number `5` on to the stack, then the number `3`, then `+` would pop the two top items on the stack and add them and push the result onto the stack. So this expression would be equivalent to just pushing an `8` onto the stack.

stack_sharp has 3 stacks available for use:
- a data stack which stores the parameters and output of functions as well as any temporary values
- a variable stack (addresses 1 - 499) which stores the values of named variables
- a heap (addresses 500 - 999) which stores larger data structures of arbitrary length (arrays and lists)

Variables in stack_sharp are simply aliases for their addresses in memory. For example, if `foo` was the first variable declared, it would become an alias for `1`: `1 foo +` -> `2`. It is recommended to not use variables for temporary value storage, instead opting to store such on the stack directly or on the heap if it is arbitrarily large.

Function notation in stack_sharp is also similar to Forth, for example:<br>
`a b c -- a c d` - the parameters are `a`, `b` and `c`, the parameter `a` is left untouched, the parameter `b` is consumed and the parameter `c` drops back to be next to `a`, and finally the result `d` is pushed onto the stack.<br>
A word on the right hand side that is in brackets generally conveys that a result has been pushed somewhere other than the stack (eg. is has been outputted in the console).<br>
A `..` represents an arbitrary number of parameters of results and `...` represents all the previous items in the stack.

One uniqueness of this interpreter is that programs can be entered one word at a time, with execution continuing until the end of the program so far has been reached.
This is not that useful, but kind of fun :)

### Built in operators/functions
All the following functions use up their parameter(s) (unless otherwise specified) and those that calculate a result push it onto the stack:

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
- `!` - binary nots the top number

**Stack Manipulation:**
- `>>`(dup) - pops the top number then:
    - if the number is > 0, it copies the new top number that many times without consuming it, eg. `45 2 >>` -> `45 45 45`
    - if the number is < 0, it removes that many items from the stack, eg. `1 2 3 4 5 -3 >>` -> `1 2`
    - if the number is = 0, it clears the stack, eg. `... 1 2 3 4 5 -1 >>` -> ` `
        - **Try not to use this outside of testing, especially within a function**
- `<>`(nswap) - pops the top number, `n`, then swaps the next top number with the number `n` numbers behind it, starting at 0, eg. `1 2 3 4 5 2 <>` -> `1 5 3 4 2`, `1 2 0 <>` -> `2 1`

**Memory:**
- `.` - pops the top number and stores it as the current memory address to be accessed (address 0 is used for I/O)
- `<` - pushes the value at the current memory address onto the stack (if the address is 0, gets a key press from the user and stores the ascii code)
- `>` - pops the top number and stores it at the current memory address (if the address is 0, prints the nuber as ascii)
- `#+` - pops the top number and allocates the next available block of memory of that length, pushing the address of the first cell to the stack
    - be aware that each allocation call must allocate > 1 cell otherwise it will fail
- `#-` - pops the top number, `loc`, and the next number, `len`, and frees a block of memory of length `len` starting at address `loc`
    - be aware that each free call must free > 1 cell otherwise it will fail, therefore it is highly recomended that you keep track of how much memory has been allocated and where and free it all at once, otherwise you could end up freeing the first 4 cells of a 5 cell allocation and be left unable to free the last one

**Testing:**
Boolean values in stack_sharp are stored as `0` for false and `-1` for true (as this is equivalent to all ones in binary), however, any non-zero number will be considered true
- `@` - tests if the top number is equal to 0 and pushes the boolean result
- `@+` - tests if the top number is greater than 0 and pushes the boolean result

**Control flow:**
- `}` - this does not compile to anything, acting only as a marker for `}`:
- `{` - [must be eventually proceeded by a `}`] when the program meets an end bracket, it will unconditionally jump to the corresponding open bracket, eg. `{ 1 2 3 + }` -> `1 5 1 5 1 5 ...`
- `(`(if) [must be eventually proceeded by a `)`] - pops the top number from the stack and if the number is 0, the program will jump to the next corresponding `)` or `)!`:
- `)`(endif) - this does not compile to anything, acting only as a marker for `(`
- `)!`(elseif) [must be eventually proceeded by a `(`] - depends on whether the preceding if statement succeeded:
    - if it succeeded, the 'else' will fail and the program will jump to the next corresponding `)` or `)!` (in which case this will occur recursively until a `)` is reached)
    - if it failed, the program will continue as usual

**Other:**
- `!?`(error) - outputs a newline in the console and ends the program, it is recommended to print an error message before calling this function

### Variables
A variable can be declared by prefixing its name with a `:`, then it acts as an alias for its address in memory. A variable's value can then be set and retrieved using `set`/`.>` and `get`/`.<` respectively (see stdlib.ss for syntax). For example, `:foobar /*declare*/ 5 foobar set /*set to 5*/ foobar get /*retrieve*/` -> `5`. Be aware that variables do not go out of scope and cannot be removed once declared so do not declare them unnecessarily.<br>
Variable 'calls' can be prefixed with `*`, which acts as an alias for instead appending ` .<`, which aids in the retrieval of values from variables that hold pointers. For example, `*foo` -> `foo .<` and `***bar` -> `bar .< .< .<`.

### Functions
A function is declared by prefixing its name (multiple aliases can be used with a `|` delimiting) with a `[` and ending the declaration with a `]` and can then be called using its name. A function can be redeclared as many times as you want, however, this is not recommended as the previous versions stick around in memory taking up space. For example, `[foo|bar 1 + ] 2 bar foo` -> `4`. Functions are compiled only once so make sure that they, for example, do not contain references to memory that might be freed later on. This also means that functions do not change even when a function that it calls changes. For example: `[foo 1 + ] [bar foo ] [foo 2 + ] 1 bar` -> `2`<br>
Function calls can be prefixed with a `;` to insert them as 'macro's, whereby the compiled code is simply copied from the function definition. This is only useful for small functions that could be compiler optimised, however, this is generally not used, with actual macros completing this role more efficiently.

### Macros
A macro is declared very similarly to a function, inculing the alias capability, except that their declaration is prefixed with a `[;` instead of just a `[`. Unlike functions, a macro definition is not compiled, instead it is stored until is is 'called' at which point it is copied into program word for word. This means that words that had their definitions changed also change. For example, `[foo 1 + ] [;bar foo ] 1 bar` -> `2`, whereas, `[foo 1 + ] [;bar foo ] [foo 2 + ] 1 bar` -> `3`.<br>
In addition, single value macros (essentially constants) can be declared by declaring them similarly to variables except then appending a `;` and the value. For example, `:foo;42 foo foo +` -> `84`, `:foo;"bar" foo foo` -> `"bar" "bar"`.

### Strings
A string starts and ends with either `'`s or `"`s, but not a mixture of both, and is compiled to each character pushed to the stack as its ascii equivalent in reverse order, eg. `'hello'` -> `111 108 108 101 104`. These characters can be used within theier respective strings by prefixing them with `/`, eg. `'\''` -> `39`. A string bounded by `"`s will have a `-1` prefixed on the stack to enable the end of the string to be identified. Both types of strings can also be prefixed with `#` to push the length of the string to the stack after it. For example, `"hello"` -> `-1 111 108 108 101 104` and `#'hi'` -> `105 104 2`. These are mainly used to store a string as a linked list and to store one as an array, respectively.

## Interpreter commands
All interpreter commands consist of `///` immediately followed by the command then whitespace separated arguments, which are:
- `import`/`dep:` - imports the file names that follow from the ss_src folder, eg. `///import fib.ss foo/bar bar/foo.txt` imports ss_src/fib.ss, ss_src/foo/bar.ss and ss_src/bar/foo.txt.ss
    - see [importing](#importing) for more details
- `clr`/`clear` - clears the console window
- `heap`/`show_heap`/`hide_heap` - toggles display of the heap
- `pause`/`unpause`/`p` - toggles execution of program

## Importing
The [import](#interpreter-commands) command can be used to import files. When importing a file, the interpreter will first import any dependencies recursively, then it will simply compile the imported files as if they were typed out in the interpreter, ignoring only the first line related to dependencies.<br>
Dependencies for a file can be defined on the first line of the file using `///dep: ` followed by whitespace separated file names, identically to the import command.<br>
By default, the interpreter will try to import the files with the stack_sharp `.ss` file extension, however, any file extension is fine as long as it is specified.

## Stdlib
The standard library functions can be found in `ss_src/stdlib/stdlib.ss`.
Documentation for these functions may follow but their names/aliases, function notations and other comments make most of them self explanatory. Feel free to add documentation if you wish.

*this section is now probably ~~still in~~complete*