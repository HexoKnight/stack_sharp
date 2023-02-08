use super::{stack::Stack, io};
use std::collections::HashMap;

//: program codes
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProgramCode {
    Int(i64),
    FuncCall(usize),
    FuncStart(Option<std::num::NonZeroUsize>),
    FuncEnd,
    PopMemLoc,
    PushMem,
    PopMem,
    Err,
    If(Option<std::num::NonZeroUsize>),
    ElseIf(Option<std::num::NonZeroUsize>),
    EndIf,
    Loop,
    EndLoop(usize),
    GtZero,
    EqZero,
    HeapAlloc,
    HeapFree,
    Dup,
    Swap,
    Op(OpCode),
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OpCode {
    NOT,
    AND,
    OR,
    XOR,
    NEG,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
}
//;

#[derive(/* Copy,  */Clone)]
pub enum Variable {
    Variable(usize),
    Function(usize),
    Macro(Vec<String>)
}

pub const MEMORY_SIZE: usize = 1000;
pub const HEAP_START: usize = 500;

//: interpreter variables
pub struct Interpreter {
    // imaginary actual stack
    pub data_stack: Stack<i64>,

    // imaginary call stack, probably just before arbitrary memory
    call_stack: Stack<usize>,
    // imaginary program byte-code
    program_codes: Vec<ProgramCode>,
    // predefined variables before all other memory
    #[cfg(debug_assertions)]
    pub pc: usize,
    #[cfg(not(debug_assertions))]
    pc: usize,

    // imaginary extra memory before the stack
    pub memory: [i64; MEMORY_SIZE],
    mem_loc: usize, // 0 is std in/out
    pub heap_pointer: usize,
    pub heap_free_pointer: usize,

    control_flow: ControlFlow,

    // only in interpreter:
    // names for memory locations
    variables: HashMap<String, Variable>,
    var_pointer: usize,
    // macros being built
    macro_codes: Vec<(Vec<String>, Vec<String>, u8)>,
}
//;

//: interpreter methods
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            memory: [0; MEMORY_SIZE],
            call_stack: Stack::new(),
            data_stack: Stack::new(),
            program_codes: Vec::new(),
            pc: 0,
            macro_codes: Vec::new(),
            mem_loc: 0, // 0 is std in/out
            var_pointer: 1, //0;
            heap_pointer: HEAP_START,
            heap_free_pointer: 0,
            control_flow: ControlFlow { func_depth: 0, if_depth: 0, if_succeeded: false, if_else: false },
            variables: HashMap::new(),
        }
    }

    pub fn input_required(&self) -> bool {
        self.pc >= self.program_codes.len()
    }

    pub fn access_for_parsing(&mut self) -> super::parse::ParserIn {
        super::parse::ParserIn {
            program_codes: &mut self.program_codes,
            macro_codes: &mut self.macro_codes,
            variables: &mut self.variables,
            memory: &mut self.memory,
            var_pointer: &mut self.var_pointer,
            pc: self.pc
        }
    }

    pub fn interpret(&mut self) -> InterpreterOut {
        interpret(
            &mut self.data_stack,
            &mut self.call_stack,
            &mut self.program_codes, &mut self.pc,
            &mut self.memory, &mut self.mem_loc,
            &mut self.heap_pointer, &mut self.heap_free_pointer,
            &mut self.control_flow)
    }
}
//;

//: extra structs
pub struct InterpreterOut {
    pub printed: bool,
    pub err: bool,
}

pub struct ControlFlow {
    pub func_depth: u8,
    pub if_depth: u8,
    pub if_succeeded: bool,
    pub if_else: bool,
}
//;

//: interpret
fn interpret(
    data_stack: &mut Stack<i64>,
    call_stack: &mut Stack<usize>,
    program_codes: &mut Vec<ProgramCode>,
    pc: &mut usize,
    memory: &mut [i64; MEMORY_SIZE],
    mem_loc: &mut usize,
    heap_pointer: &mut usize,
    heap_free_pointer: &mut usize,
    control_flow: &mut ControlFlow) -> InterpreterOut {
    use OpCode::*;
    use ProgramCode::*;
    use super::print_err;
    let ControlFlow { func_depth, if_depth, if_succeeded, if_else } = control_flow;
    macro_rules! ignore {
        () => { *func_depth != 0 || *if_depth != 0 };
    }
    let mut newline: bool = true;
    let mut err: bool = false;
    while *pc < program_codes.len() {
        match program_codes[*pc] {
            Int(int) => {
                if !ignore!() {
                    data_stack.push(int)
                }
            }
            FuncCall(loc) => {
                if !ignore!() {
                    call_stack.push(*pc);
                    *pc = loc;
                }
            }
            FuncStart(op_loc) => {
                if let Some(loc) = op_loc {
                    *pc = loc.into();
                } else {
                    *func_depth += 1;
                }
            }
            FuncEnd => {
                if !ignore!() {
                    if let Some(int) = call_stack.try_pop_with_err("] without [") {
                        *pc = int;
                    }
                } else if *func_depth > 0 {
                    *func_depth -= 1;
                }
            }
            PopMemLoc => {
                if !ignore!() {
                    if let Some(int) = data_stack.try_pop() {
                        if int >= 0 {
                            *mem_loc = int as usize;
                        } else {
                            print_err("try set memory address < 0");
                        }
                    }
                }
            }
            PushMem => {
                if !ignore!() {
                    if *mem_loc == 0 {
                        /* let mut input = String::new();
                        loop {
                            //print!(">> ");
                            _ = stdout().flush();
                            match io::stdin().read_line(&mut input) {
                                Ok(_) => break,
                                core::result::Result::Err(error) => print_err(format!("read error: {error}")),
                            }
                        }
                        data_stack.push(-1);
                        for chr in input.trim_end_matches(['\n', '\r']).chars().rev() {
                            data_stack.push(chr as i64);
                        } */
                        data_stack.push(io::read_char() as i64);
                    } else {
                        data_stack.push(memory[*mem_loc]);
                    }
                }
            }
            PopMem => {
                if !ignore!() {
                    if let Some(int) = data_stack.try_pop() {
                        if *mem_loc == 0 {
                            if int < 0 {
                                print_err("char cannot be < 0")
                            } else {
                                let chr = char::from_u32(int as u32).unwrap();
                                io::print_flushed(chr);
                                newline = chr == '\n';
                            }
                        } else {
                            memory[*mem_loc] = int;
                        }
                    }
                }
            }
            Err => {
                if !ignore!() {
                    print_err(format!("\n"));
                    err = true;
                    break;
                }
            }
            If(op_loc) => {
                if *func_depth == 0 {
                    if *if_depth == 0 && !*if_succeeded {
                        if let Some(0) = data_stack.try_pop() {
                            if let Some(loc) = op_loc {
                                *pc = loc.into();
                            } else {
                                *if_depth += 1;
                            }
                        }
                    } else if !*if_else {
                        *if_depth += 1;
                    }
                    *if_else = false;
                }
            }
            ElseIf(op_loc) => {
                if *func_depth == 0 {
                    *if_depth = match *if_depth {
                        0 => {
                            *if_succeeded = true;
                            if let Some(loc) = op_loc {
                                *pc = loc.into();
                                //0
                                continue;
                            } else {
                                1
                            }
                        }, // succeeded previously so prevent from being given opportunity to succeed at next ElseIf
                        1 if !*if_succeeded => 0, // failed previously so allow opportunity to succeed at next ElseIf
                        _ => *if_depth
                    };
                    *if_else = true;
                }
            }
            EndIf => {
                if *func_depth == 0 {
                    if *if_depth > 1 {
                        *if_depth -= 1;
                    } else {
                        *if_depth = 0;
                        *if_succeeded = false;
                    }
                    *if_else = false; // shouldn't really be necessary
                }
            }
            Loop => {}
            EndLoop(loc) => {
                if !ignore!() {
                    *pc = loc;
                    // let mut loop_depth: u8 = 1;
                    // while loop_depth > 0 {
                    //     if *pc == 0 {
                    //         print_err("'endloop' without 'loop'");
                    //     }
                    //     *pc -= 1;
                    //     match program_codes[*pc] {
                    //         EndLoop(_) => loop_depth += 1,
                    //         Loop => loop_depth -= 1,
                    //         _ => {}
                    //     }
                    // }
                }
            }
            GtZero => {
                if !ignore!() {
                    if let Some(value) = data_stack.try_pop() {
                        if value > 0 {
                            data_stack.push(-1)
                        } else {
                            data_stack.push(0)
                        }
                    }
                }
            }
            EqZero => {
                if !ignore!() {
                    if let Some(value) = data_stack.try_pop() {
                        if value == 0 {
                            data_stack.push(-1)
                        } else {
                            data_stack.push(0)
                        }
                    }
                }
            }
            HeapAlloc => {
                if !ignore!() {
                    if let Some(length) = data_stack.try_pop() {
                        if length < 2 {
                            print_err("cannot allocate under 2 cells to heap");
                        } else {
                            let mut prev_mem_loc: usize = 0;
                            let mut mem_loc: usize = *heap_free_pointer;
                            while mem_loc != 0 {
                                match memory[mem_loc + 1] - length {
                                    0 => {
                                        data_stack.push(mem_loc as i64);
                                        if prev_mem_loc == 0 {
                                            *heap_free_pointer = 0;
                                        } else {
                                            memory[prev_mem_loc] = memory[mem_loc];
                                        }
                                        break;
                                    }
                                    2.. => {
                                        data_stack.push(mem_loc as i64);
                                        memory[prev_mem_loc] = mem_loc as i64 + length;
                                        break;
                                    }
                                    _ => {
                                        prev_mem_loc = mem_loc;
                                        mem_loc = memory[mem_loc] as usize;
                                    }
                                }
                            }
                            if mem_loc == 0 {
                                if *heap_pointer + length as usize > memory.len() + 1 {
                                    print_err(format!("!? wouldn't fit in the heap :/ (only {} / {} cells left)", MEMORY_SIZE - *heap_pointer, MEMORY_SIZE - HEAP_START));
                                } else {
                                    data_stack.push(*heap_pointer as i64);
                                    *heap_pointer += length as usize;
                                }
                            }
                        }
                    }
                }
            }
            HeapFree => {
                if !ignore!() {
                    if let Some(addr) = data_stack.try_pop() {
                        if addr < HEAP_START as i64 {
                            print_err("cannot free outside of heap");
                        } else if let Some(length) = data_stack.try_pop() {
                            if length < 2 {
                                print_err("cannot free under 2 cells from heap");
                            } else if addr + length > *heap_pointer as i64 {
                                print_err("cannot free unallocated memory from heap");
                            } else {
                                // all in format ([location of pointer to current], [location of current])
                                let mut mem_loc: (usize, usize) = (0, *heap_free_pointer);
                                let mut before_mem_loc: (usize, usize) = (0, 0);
                                let mut after_mem_loc: (usize, usize) = (0, 0);
                                let mut err: bool = false;
                                while mem_loc.1 != 0 {
                                    match (mem_loc.1 as i64 + memory[mem_loc.1 + 1] - addr/* amount mem-end is after start */,
                                        addr + length - mem_loc.1 as i64/* amount mem-start is before end */) {
                                        (0, _) => { // mem directly before
                                            before_mem_loc = mem_loc;
                                        }
                                        (_, 0) => { // mem directly after
                                            after_mem_loc = mem_loc;
                                        }
                                        (1.., 1..) => { // mem overlapping
                                            print_err("cannot free (partially) unallocated memory from heap");
                                            err = true;
                                            break;
                                        }
                                        _ => {} // mem disjoint
                                    }
                                    if before_mem_loc != (0, 0) && after_mem_loc != (0, 0) {
                                        break;
                                    }
                                    mem_loc.0 = mem_loc.1;
                                    mem_loc.1 = memory[mem_loc.0] as usize; // mem_loc now points to nect in free_heap list
                                }
                                if !err {
                                    let addr: usize = addr as usize; // addr and length now usize as have been proved to be positive
                                    let length: usize = length as usize;
                                    macro_rules! reassign_pointer {
                                        ($previous:expr, $next:expr) => {
                                            if $previous == 0 {
                                                *heap_free_pointer = $next as usize;
                                            } else {
                                                memory[$previous] = $next;
                                            }
                                        };
                                    }
                                    if after_mem_loc != (0, 0) { // mem after
                                        if before_mem_loc != (0, 0) { // mems both before and after
                                            // remove mem after from free_heap list
                                            reassign_pointer!(after_mem_loc.0, memory[after_mem_loc.1]);
                                            // extend mem before length to cover current and mem after
                                            memory[before_mem_loc.1 + 1] += memory[after_mem_loc.1 + 1] + length as i64;

                                        } else { // mem only after
                                            reassign_pointer!(after_mem_loc.0, addr as i64);
                                            memory[addr] = memory[after_mem_loc.1];
                                            memory[addr + 1] = memory[after_mem_loc.1 + 1] + length as i64;
                                        }
                                    } else { // no mem after
                                        if before_mem_loc != (0, 0) { // mem only before
                                            if addr + length == *heap_pointer { // at end of heap
                                                reassign_pointer!(before_mem_loc.0, memory[before_mem_loc.1]); // before_mem_loc.0 is previous in free_heap list
                                                *heap_pointer = before_mem_loc.1;
                                            } else {
                                                memory[before_mem_loc.1 + 1] += length as i64;
                                            }
                                        } else { // no mem on either side
                                            if addr + length == *heap_pointer { // at end of heap
                                                *heap_pointer = addr;
                                            } else {
                                                reassign_pointer!(mem_loc.0, addr as i64); // mem_loc.0 is last in free_heap list
                                                memory[addr] = 0;
                                                memory[addr + 1] = length as i64;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Dup => {
                if !ignore!() {
                    if let Some(amount) = data_stack.try_pop() {
                        if amount > 0 {
                            // if let Some(from_top) = data_stack.try_pop() {
                                if let Some(value) = data_stack.try_peek(0/* from_top as usize */) {
                                    data_stack.push_multiple(std::iter::repeat(value).take(amount as usize));
                                }
                            // }
                        } else if amount == 0 {
                            data_stack.clear();
                        } else {
                            data_stack.pop_multiple((-amount) as usize);
                        }
                    }
                }
            }
            Swap => {
                if !ignore!() {
                    if let Some(from_top) = data_stack.try_pop() {
                        if from_top >= 0 {
                            if let Some(top) = data_stack.try_pop() {
                                if let Some(middle) = data_stack.try_peek(from_top as usize) {
                                    data_stack.try_set(from_top as usize, top);
                                    data_stack.push(middle);
                                }
                            }
                        } else {
                            print_err("cannot swap with index < 0");
                        }
                    }
                }
            }
            Op(op) => {
                if !ignore!() {
                    if let Some(right) = data_stack.try_pop() {
                        if matches!(op, NOT) {
                            data_stack.push(!right)
                        } else if matches!(op, NEG) {
                            data_stack.push(-right)
                        } else if let Some(left) = data_stack.try_pop() {
                            let overflow = |tuple: (i64, bool)| {
                                if tuple.1 {
                                    print_err("integer overflow");
                                };
                                tuple.0
                            };
                            data_stack.push(match op {
                                AND => left & right,
                                OR  => left | right,
                                XOR => left ^ right,
                                ADD => overflow(left.overflowing_add(right)),
                                SUB => overflow(left.overflowing_sub(right)),
                                MUL => overflow(left.overflowing_mul(right)),
                                DIV => overflow(left.overflowing_div(right)),
                                MOD => left % right,
                                NOT|NEG => 0
                            })
                        }
                    }
                }
            }
        }
        *pc += 1;
    }
    InterpreterOut { printed: newline, err }
}
//;