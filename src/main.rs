use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::fs::File;

// ECHO
fn rust_echo(args: Vec<String>) {
    let mut interpret_escapes = false;
    let mut omit_newline = false;

    // Verificar opciones de línea de comando
    let mut args_iter = args.iter().skip(1);
    while let Some(arg) = args_iter.next() {
        if arg == "-e" {
            interpret_escapes = true;
        } else if arg == "-E" {
            interpret_escapes = false;
        } else if arg == "-n" {
            omit_newline = true;
        } else {
            // Interpretar argumento y manejar secuencias de escape
            let arg = if interpret_escapes {
                interpret_special_characters(&arg)
            } else {
                arg.clone()
            };
            print!("{} ", arg);
        }
    }

    // Imprimir nueva línea si no se omite
    if !omit_newline {
        println!();
    }
}

fn interpret_special_characters(s: &str) -> String {
    let mut result = String::new();
    let mut escape = false;

    for c in s.chars() {
        if escape {
            match c {
                'n' => result.push('\n'),
                't' => result.push('\t'),
                '\\' => result.push('\\'),
                '\"' => result.push('\"'),
                '\'' => result.push('\''),
                _ => result.push(c),
            }
            escape = false;
        } else if c == '\\' {
            escape = true;
        } else {
            result.push(c);
        }
    }

    result
}


// CAT
fn rust_cat(args: Vec<String>) -> io::Result<()> {
    if args.len() != 2 {
        if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
            println!("Usage: rust_cat <filename>");
        } else {
            eprintln!("Usage: rust_cat <filename>");
        }
        return Ok(());
    }
    let filename = &args[1];
    let file = std::fs::read_to_string(filename)?;
    println!("{}", file);
    Ok(())
}

// LS
fn rust_ls(args: Vec<String>) -> io::Result<()> {
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        println!("Usage: rust_ls [directory]");
        return Ok(());
    }

    let dir = match args.get(1) {
        Some(d) => d,
        None => ".",
    };
    let entries = fs::read_dir(dir)?;
    for entry in entries {
        println!("{}", entry?.file_name().to_string_lossy());
    }
    Ok(())
}

// FIND
fn rust_find(args: Vec<String>) -> io::Result<()> {
    if args.len() != 3 {
        if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
            println!("Usage: rust_find <directory> <pattern>");
        } else {
            eprintln!("Usage: rust_find <directory> <pattern>");
        }
        return Ok(());
    }
    let directory = &args[1];
    let pattern = &args[2];
    let path = Path::new(directory);
    if path.is_dir() {
        find_files(path, pattern)?;
    } else {
        eprintln!("Not a valid directory");
    }
    Ok(())
}

fn find_files(dir: &Path, pattern: &str) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            find_files(&path, pattern)?;
        } else if let Some(file_name) = path.file_name() {
            if let Some(name) = file_name.to_str() {
                if name.contains(pattern) {
                    println!("{}", path.display());
                }
            }
        }
    }
    Ok(())
}


// GREP
fn rust_grep(args: Vec<String>) -> io::Result<()> {
    if args.len() != 3 {
        if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
            println!("Usage: rust_grep <pattern> <filename>");
        } else {
            eprintln!("Usage: rust_grep <pattern> <filename>");
        }
        return Ok(());
    }
    let pattern = &args[1];
    let filename = &args[2];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [<args>]", &args[0]);
        std::process::exit(1);
    }
    match args[1].as_str() {
        "rust_echo" => rust_echo(args),
        "rust_cat" => match rust_cat(args) {
            Ok(_) => (),
            Err(err) => eprintln!("Error: {}", err),
        },
        "rust_ls" => match rust_ls(args) {
            Ok(_) => (),
            Err(err) => eprintln!("Error: {}", err),
        },
        "rust_find" => match rust_find(args) {
            Ok(_) => (),
            Err(err) => eprintln!("Error: {}", err),
        },
        "rust_grep" => match rust_grep(args) {
            Ok(_) => (),
            Err(err) => eprintln!("Error: {}", err),
        },
        _ => eprintln!("Unknown command"),
    }
}
