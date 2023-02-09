use super::interpret::{ProgramCode, OpCode, Variable};

//: format program code
pub fn format_and_split_program_code(chars: impl IntoIterator<Item = char>) -> Vec<String> {
    let mut split_str: Vec<String> = Vec::new();
    let mut current_str = String::new();
    let mut last_chr: char = ' ';
    let mut in_double_quote: bool = false;
    let mut in_single_quote: bool = false;
    let mut line_comment: bool = false;
    let mut block_comment: bool = false;
    macro_rules! push_current_string { () => {
        if !current_str.is_empty() {
            split_str.push(current_str);
            current_str = String::new();
        }
    } }
    for chr in chars.into_iter().chain(std::iter::once('\n')) {
        if in_double_quote {
            current_str.push(chr);
            if chr == '"' {
                if !current_str.ends_with("\\\"") {
                    push_current_string!();
                    in_double_quote = false;
                }
            }
        } else if in_single_quote {
            current_str.push(chr);
            if chr == '\'' {
                if !current_str.ends_with("\\'") {
                    push_current_string!();
                    in_single_quote = false;
                }
            }
        } else if line_comment {
            if chr == '\n' {
                current_str.clear();
                line_comment = false;
            }
        } else if block_comment {
            if last_chr == '*' && chr == '/' {
                current_str.clear();
                block_comment = false;
                last_chr = ' ';
            } else {
                last_chr = chr;
            }
        } else if "\t\x0c\r\n ".contains(chr) {
            push_current_string!();
            if line_comment && chr == '\n' {
                current_str.clear();
                line_comment = false;
            }
        } else {
            current_str.push(chr);
            if chr == '"' {
                if current_str.len() == 1 {
                    in_double_quote = true;
                }
            } if chr == '\'' {
                if current_str.len() == 1 {
                    in_single_quote = true;
                }
            } else if current_str.ends_with("//") {
                current_str.truncate(current_str.len().saturating_sub(2));
                push_current_string!();
                line_comment = true;
            } else if current_str.ends_with("/*") {
                current_str.truncate(current_str.len().saturating_sub(2));
                push_current_string!();
                block_comment = true;
            }
        }
    }
    return split_str;
}
//;

pub struct ParserIn<'a> {
    pub program_codes: &'a mut Vec<ProgramCode>,
    pub macro_codes: &'a mut Vec<(Vec<String>, Vec<String>, u8)>,
    pub variables: &'a mut std::collections::HashMap<String, Variable>,
    pub memory: &'a mut [i64; super::interpret::MEMORY_SIZE],
    pub var_pointer: &'a mut usize,
    pub pc: usize,
}

//: parse program code
pub fn parse_program_code(chars: impl IntoIterator<Item = char>, interpreter: ParserIn, compiler_optimise: bool) {
    let ParserIn { program_codes, macro_codes,
        variables, memory, var_pointer, pc } = interpreter;
    //println!("{:?}", format_and_split_program_code(string));
    use OpCode::*;
    use ProgramCode::*;
    use super::interpret::Variable::*;
    let mut words = format_and_split_program_code(chars.into_iter()).into_iter();
    let mut word: String;
    loop {
        if let Some(next) = words.next() {
            word = next.to_string();
        } else {
            break;
        }
        if let Some(last) = macro_codes.last_mut() {
            if word.starts_with('[') && word.len() > 1 {
                if word[1..].starts_with(';') && word.len() > 2 {
                    macro_codes.push((word[2..].split("|").map(|x| x.to_owned()).collect(), Vec::new(), 0));
                    continue;
                } else {
                    last.2 += 1;
                }
            } else if word.starts_with(':') && word.len() > 1 && word.contains(';') {
                let parts = word[1..].rsplit_once(';').unwrap_or_default();
                if !variables.contains_key(parts.0) {
                    variables.insert(parts.0.to_owned(), Macro(format_and_split_program_code(parts.1.chars().into_iter())));
                }
                continue;
            } else if word == "]" {
                if last.2 == 0 {
                    for name in last.0.drain(..) {
                        variables.insert(name, Macro(last.1.clone()));
                    }
                    macro_codes.pop();
                    continue;
                } else {
                    last.2 -= 1;
                }
            }
            last.1.push(word);
        }
        else if let Some(var) = variables.get(&word) {
            match var {
                Variable(value) => program_codes.push(Int(*value as i64)),
                Function(value) => program_codes.push(FuncCall(*value as usize)),
                Macro(value) => {
                    let mut new_words = value.clone();
                    new_words.extend(words);
                    words = new_words.into_iter();
                }
            }
        } else {
            if word.starts_with(':') && word.len() > 1 {
                let name = word[1..].to_owned();
                if let Some(parts) = name.split_once(';') {
                    if !variables.contains_key(parts.0) {
                        variables.insert(parts.0.to_owned(), Macro(format_and_split_program_code(parts.1.chars().into_iter())));
                    }
                } else {
                    if !variables.contains_key(&name) {
                        variables.insert(name.to_owned(), Variable(*var_pointer));
                        memory[*var_pointer] = 0;
                        *var_pointer += 1;
                    }
                }
            } else if word.starts_with('*') && word.contains(|x| x != '*') {
                let num = word.chars().take_while(|c| *c == '*').count();
                let name = &word[num..];
                if let Some(Variable(loc)) = variables.get(name) {
                    program_codes.extend(std::iter::once(Int(*loc as i64)).chain([PopMemLoc, PushMem].into_iter().cycle().take(num * 2)));
                } else {
                    println!("!? {} ?", word);
                }
            } else if word.starts_with('[') && word.len() > 1 {
                if word[1..].starts_with(';') && word.len() > 2 {
                    macro_codes.push((word[2..].split("|").map(|x| x.to_owned()).collect(), Vec::new(), 0));
                } else {
                    for name in word[1..].split("|") {
                        variables.insert(name.to_owned(), Function(program_codes.len()));
                    }
                    program_codes.push(FuncStart(None));
                }
            } else if word.starts_with(';') && word.len() > 1 {
                let name = &word[1..];
                match variables.get(name) {
                    Some(Function(loc)) => {
                        let mut depth: u8 = 0;
                        let mut i: usize = *loc;
                        loop {
                            i += 1;
                            match &program_codes[i] {
                                ProgramCode::FuncStart(_) => depth += 1,
                                ProgramCode::FuncEnd => {
                                    if depth == 0 {
                                        break;
                                    } else {
                                        depth -= 1;
                                    }
                                }
                                code => program_codes.push(*code)
                            }
                        }
                    }
                    Some(Macro(value)) => {
                        let mut new_words = value.clone();
                        new_words.extend(words);
                        words = new_words.into_iter();
                    }
                    _ => println!("!? {} ?", word)
                }
            } else if (word.starts_with('"') && word.len() > 1 && word.ends_with('"')) || (word.starts_with("#\"") && word.len() > 2 && word.ends_with('"')) || 
                    (word.starts_with('\'') && word.len() > 1 && word.ends_with('\'')) || (word.starts_with("#'") && word.len() > 2 && word.ends_with('\'')) {
                let start = if word.starts_with('#') { 2 } else { 1 };
                let string = &word[start..word.len()-1];
                if word.ends_with('"') {
                    program_codes.push(Int(-1));
                }
                let mut previous: Option<char> = None;
                for chr in string.chars().rev() {
                    if chr == '\\' {
                        if let Some(prev) = previous {
                            program_codes.push(Int(match prev {
                                'n' => '\n',
                                't' => '\t',
                                'r' => '\r',
                                _ => prev
                            } as i64));
                            previous = None;
                        } else {
                            previous = Some('\\');
                        }
                    } else {
                        if let Some(prev) = previous {
                            program_codes.push(Int(prev as i64));
                        }
                        previous = Some(chr);
                    }
                }
                if let Some(prev) = previous {
                    program_codes.push(Int(prev as i64));
                }
                if start == 2 { // length prefixed
                    program_codes.push(Int(string.len() as i64));
                }
            } else {
                macro_rules! end_if {
                    () => {   
                        if let Some(loc) = program_codes.iter().rposition(|x| matches!(x, If(None))) {
                            program_codes[loc] = If(std::num::NonZeroUsize::new(program_codes.len()));
                        }
                    };
                }
                if let Some(code) = 
                    match word.as_str() {
                        "]" => {
                            if let Some(loc) = program_codes.iter().rposition(|x| matches!(x, FuncStart(None))) {
                                program_codes[loc] = FuncStart(std::num::NonZeroUsize::new(program_codes.len()));
                            }
                            Some(FuncEnd)
                        }
                        "." => Some(PopMemLoc),
                        "<" => Some(PushMem),
                        ">" => Some(PopMem),
                        "!?" => Some(Err),
                        "(" => {
                            let mut back_pc = program_codes.len();
                            let mut depth = 0;
                            while back_pc > 0 { back_pc -= 1;
                                match program_codes[back_pc] {
                                    EndIf | ElseIf(Some(_)) => depth += 1,
                                    If(_) => depth -= 1,
                                    ElseIf(None) => {
                                        program_codes[back_pc] = ElseIf(std::num::NonZeroUsize::new(program_codes.len()));
                                    }
                                    _ => {}
                                }
                                if depth < 0 {
                                    break;
                                }
                            }
                            Some(If(None))
                        }
                        ")!" => { end_if!(); Some(ElseIf(None)) },
                        ")" => { end_if!(); Some(EndIf) },
                        "{" => Some(Loop),
                        "}" => {
                            Some(EndLoop(
                            if let Some(loc) = program_codes.iter().rposition(|x| matches!(x, Loop)) {
                                loc
                            } else {
                                super::print_err("} without {");
                                continue;
                            }))}
                        "+@" => Some(GtZero),
                        "@" => Some(EqZero),
                        "#+" => Some(HeapAlloc),
                        "#-" => Some(HeapFree),
                        ">>" => Some(Dup),
                        "<>" => Some(Swap),
                        "!" => Some(Op(NOT)),
                        "&" => Some(Op(AND)),
                        "|" => Some(Op(OR)),
                        "^" => Some(Op(XOR)),
                        "~" => Some(Op(NEG)),
                        "+" => Some(Op(ADD)),
                        "-" => Some(Op(SUB)),
                        "*" => Some(Op(MUL)),
                        "/" => Some(Op(DIV)),
                        "%" => Some(Op(MOD)),
                        _ => {
                            if let Some(int) = word.parse::<i64>().ok() {
                                Some(Int(int))
                            } else {
                                println!("!? {} ?", word);
                                None
                            }
                        }
                } {
                    program_codes.push(code);
                }
            }
        }
        if compiler_optimise {
            macro_rules! remove_last {
                ($num:expr) => {
                    program_codes.truncate(program_codes.len() - $num)
                };
            }
            macro_rules! replace_last {
                ($num:expr, $slice:expr) => { {
                    program_codes.truncate(program_codes.len() - $num);
                    program_codes.extend_from_slice($slice);
                } };
            }
            loop {
                match &program_codes[std::cmp::max(program_codes.len().saturating_sub(4), pc)..] {
                    &[.., Int(_)|GtZero|EqZero|Dup|Swap|Op(_), Int(0), Dup] => replace_last!(3, &[Int(0), Dup]),
                    &[.., Int(_)|PushMem|GtZero|EqZero, Int(drop), Dup] if drop < 0 => if drop == -1 {remove_last!(3)} else {replace_last!(3, &[Int(drop + 1), Dup])},
                    &[.., Int(swap), Swap, Int(drop), Dup] if drop < 0 && -drop >= swap + 2 => replace_last!(4, &[Int(drop), Dup]),
                    &[.., Int(num1), Swap, Int(num2), Swap] if num1 == num2 => remove_last!(4),
                    &[.., Int(dupped), Dup, Int(num2), Dup] if dupped > 0 => if dupped + num2 == 0 { remove_last!(4) } else { replace_last!(4, &[Int(dupped + num2), Dup]) },
                    &[.., Int(num), Int(0), Swap, op] if matches!(op, PopMemLoc|PushMem|PopMem|HeapAlloc) => replace_last!(4, &[op, Int(num)]),
                    &[.., PushMem, Int(num2), Int(0), Swap] => replace_last!(4, &[Int(num2), PushMem]),
                    &[.., Int(num1), Int(num2), Int(0), Swap] => replace_last!(4, &[Int(num2), Int(num1)]),
                    &[.., Int(0), Op(SUB|ADD|OR)] => remove_last!(2),
                    &[.., Int(1), Op(MUL|DIV)] => remove_last!(2),
                    &[.., Int(-1), Op(AND)] => remove_last!(2),
                    &[.., Int(num), Op(SUB)] => replace_last!(2, &[Int(-num), Op(ADD)]),
                    &[.., Op(NEG), Op(op @ ADD|op @ SUB)] => replace_last!(2, &[Op(if op == ADD { SUB } else { ADD })]),
                    &[.., Op(MUL), Int(num), Op(MUL)] => replace_last!(3, &[Int(num), Op(MUL), Op(MUL)]),
                    &[.., Op(ADD), Int(num), Op(ADD)] => replace_last!(3, &[Int(num), Op(ADD), Op(ADD)]),
                    &[.., Int(num1), Op(op1), Int(num2), Op(op2)] if num1 == num2 && matches!((op1, op2), (ADD, SUB)|(SUB, ADD)|(MUL, DIV)) => remove_last!(4),
                    &[.., Op(NEG), Op(NEG)] => remove_last!(2),
                    &[.., Op(NOT), Op(NOT)] => remove_last!(2),
                    &[.., Int(num), EqZero] => replace_last!(2, &[Int(if num == 0 { -1 } else { 0 })]),
                    &[.., Int(num), GtZero] => replace_last!(2, &[Int(if num > 0 { -1 } else { 0 })]),
                    &[.., Int(num), Op(NEG)] => replace_last!(2, &[Int(-num)]),
                    &[.., Int(num), Op(NOT)] => replace_last!(2, &[Int(!num)]),
                    &[.., Int(left), Int(right), Op(op)] if !matches!(op, NEG|NOT) => {
                        remove_last!(3);
                        program_codes.push(Int(match op {
                            AND => left & right,
                            OR  => left | right,
                            XOR => left ^ right,
                            ADD => left.overflowing_add(right).0,
                            SUB => left.overflowing_sub(right).0,
                            MUL => left.overflowing_mul(right).0,
                            DIV => left.overflowing_div(right).0,
                            MOD => left % right,
                            NOT|NEG => 0
                        }));
                    }
                    _ => break
                }
            }
        }
    }
}
//;