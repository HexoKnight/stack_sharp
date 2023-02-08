use std::{fs::{self, File, ReadDir}, path::Path, io::{BufReader, BufRead}};

use super::interpret::Interpreter;

pub struct ImportManager<'a, 'b> {
    imports: Vec<String>,
    paths: &'b Vec<&'a Path>
}
impl ImportManager<'_, '_> {
    pub fn new<'a, 'b>(paths: &'b Vec<&'a Path>) -> ImportManager<'a, 'b> {
        ImportManager { imports: Vec::<String>::new(), paths }
    }
}

pub fn import_dir(manager: &mut ImportManager, interpreter: &mut Interpreter, path: &Path, compiler_optimise: bool) -> Result<(), ()> {
    if !path.is_dir() {
        super::print_err(format!("failed to import from directory {}: it is not a directory", path.display()));
        return Err(());
    }
    let dir: ReadDir;
    if let Ok(val) = fs::read_dir(path) {
        dir = val;
    } else {
        super::print_err(format!("failed to import from directory {}", path.file_name().unwrap_or_default().to_str().unwrap_or("[unknown]")));
        return Err(());
    }
    for file in dir {
        if let Ok(val) = file {
            if let Ok(()) = import_file(manager, interpreter, val.path().as_path(), compiler_optimise) {
            } else {
            }
        }
    }

    Ok(())
}

pub fn import_file(manager: &mut ImportManager, interpreter: &mut Interpreter, path: &Path, compiler_optimise: bool) -> Result<(), ()> {
    if path.is_relative() {
        for lib_path in manager.paths {
            let file_path = lib_path.join(&path);
            if file_path.is_file() {
                if import_file(manager, interpreter, &file_path, compiler_optimise).is_ok() {
                    return Ok(());
                }
            }
        }
    }
    if path.extension() == None {
        if let Ok(_) = import_file(manager, interpreter, &path.with_extension("ss"), compiler_optimise) {
            return Ok(());
        }
    }
    if !path.is_file() {
        super::print_err(format!("failed to import from {}: it is not a file", path.display()));
        return Err(());
    }
    let file: File;
    if let Ok(val) = fs::File::open(path) {
        file = val;
    } else {
        super::print_err(format!("failed to import {}", path.file_name().unwrap_or_default().to_str().unwrap_or("[unknown]")));
        return Err(());
    }
    let mut reader = BufReader::new(file);
    let mut first_line: String = String::new();
    match reader.read_line(&mut first_line) {
        Err(_) => {
            super::print_err(format!("failed to import {}", path.file_name().unwrap_or_default().to_str().unwrap_or("[unknown]")));
            return Err(());
        }
        Ok(0) => {
            super::print_err(format!("failed to import {}: file is empty", path.file_name().unwrap_or_default().to_str().unwrap_or("[unknown]")));
            return Err(());
        }
        _ => ()
    };
    let lines;
    if first_line.starts_with("//dep:") {
        let dependencies = first_line[6..].split_ascii_whitespace();
        if let Err(_) = import_multiple(manager, interpreter, dependencies, compiler_optimise) {
            super::print_err(format!("failed to import {}: failed to import dependencies", path.file_name().unwrap_or_default().to_str().unwrap_or("[unknown]")));
            return Err(());
        }
        first_line = String::new();
    }
    lines = std::iter::once(first_line).chain(reader.lines().map(|x| x.unwrap_or_default()));
    super::parse::parse_program_code(lines.flat_map(|x| x.chars().chain(std::iter::once('\n')).collect::<Vec<_>>()), interpreter.access_for_parsing(), compiler_optimise);
    manager.imports.push(path.file_stem().unwrap_or_default().to_str().unwrap_or("[unknown]").to_owned());
    Ok(())
}

pub fn import_multiple<'a>(manager: &mut ImportManager, interpreter: &mut Interpreter, imports: impl std::iter::IntoIterator<Item = &'a str>, compiler_optimise: bool) -> Result<(), ()> {
    for import in imports {
        let name = import.to_owned();
        if !manager.imports.contains(&name) {
            import_file(manager, interpreter, Path::new(&name), compiler_optimise)?;
        }
    }
    Ok(())
}