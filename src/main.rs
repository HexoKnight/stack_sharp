mod stack;
mod parse;
mod debug;
mod io;
mod interpret;
mod import;

//use stack::Stack;
use parse::*;
use interpret::Interpreter;

#[inline(always)]
fn print_err(err: impl std::fmt::Display) {
    println!("!? {}", err);
}

fn main() {
    let mut interpreter: Interpreter = Interpreter::new();
    let mut paths = Vec::new();
    paths.push(std::path::Path::new("ss_src"));
    let mut import_manager: import::ImportManager = import::ImportManager::new(&paths);

    let /* mut */ compiler_optimise: bool = true;

    import::import_dir(&mut import_manager, &mut interpreter, std::path::Path::new("ss_src/stdlib"), compiler_optimise).unwrap();
    
    let mut settings = std::collections::HashMap::from([
        ("show_heap", false),
        ("pause", false),
        #[cfg(debug_assertions)]
        ("show_pc", false),
    ]);

    loop {
        while interpreter.input_required() || settings["pause"] {
            let input = io::read_line(">> ");
            if input.starts_with("//") {
                let mut command_args = input[3..].split_ascii_whitespace();
                match command_args.next() {
                    Some("import"|"dep:") => {
                        if let Err(_) = import::import_multiple(&mut import_manager, &mut interpreter, command_args, compiler_optimise) {
                            println!("failed to import files");
                        }
                    }
                    Some("clr"|"clear") => {
                        io::clear_screen();
                        break;
                    }
                    Some("heap"|"show_heap"|"hide_heap") => {
                        *settings.get_mut("show_heap").unwrap() = !settings["show_heap"];
                        break;
                    }
                    Some("pause"|"unpause"|"p") => {
                        *settings.get_mut("pause").unwrap() = !settings["pause"];
                    }
                    #[cfg(debug_assertions)]
                    Some("pc"|"show_pc"|"hide_pc") => {
                        *settings.get_mut("show_pc").unwrap() = !settings["show_pc"];
                        break;
                    }
                    _ => {}
                }
            }
            //println!("{:?}", input.as_bytes());
            parse_program_code(input.chars(), interpreter.access_for_parsing(), compiler_optimise);
        }
        let interpret::InterpreterOut { printed: newline, err } = interpreter.interpret();
        if !newline {
            println!("");
        }
        println!("{}", interpreter.data_stack);
        if settings["show_heap"] {
            debug::print_heap(&interpreter.memory, &interpreter.heap_pointer, &interpreter.heap_free_pointer);
        }
        #[cfg(debug_assertions)]
        if settings["show_pc"] {
            println!("{}", interpreter.pc);
        }
        // println!("{}", program_stack.len())
        if err {
            break;
        }
    }
    io::print_flushed("Press any key to continue...");
    io::read_char();
}