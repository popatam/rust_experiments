use std::env::args;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::exit;

const USAGE_STR: &str = "usage: wc [-l] [-m] [-w] [file ...]";

struct Flags {
    is_lines_count: bool,
    is_word_count: bool,
    is_char_count: bool,
}

impl Flags {
    fn check(&mut self) {
        if !self.is_lines_count && !self.is_word_count && !self.is_char_count {
            self.is_lines_count = true;
            self.is_word_count = true;
            self.is_char_count = true;
        }
    }
}

struct Counters {
    lines: usize,
    words: usize,
    chars: usize,
}

impl Counters {
    fn new() -> Self {
        Self {
            lines: 0,
            words: 0,
            chars: 0,
        }
    }

    // /// не исползуется, т.к. вывод идёт через Output
    // fn print(&self, file: &String, flags: &Flags) {
    //     println!("{}", file);
    //     if flags.is_lines_count {
    //         println!("lines: {}", self.lines);
    //     }
    //     if flags.is_word_count {
    //         println!("words: {}", self.words);
    //     }
    //     if flags.is_char_count {
    //         println!("chars: {}", self.chars);
    //     }
    //     println!();
    // }
}

/// Структура для печати результатов
struct Output<'a> {
    file: &'a PathBuf,
    counters: &'a Counters,
    flags: &'a Flags,
}

impl<'a> Output<'a> {
    fn new(file: &'a PathBuf, counters: &'a Counters, flags: &'a Flags) -> Self {
        Self {
            file,
            counters,
            flags,
        }
    }
}

impl Display for Output<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n", self.file.display())?;
        if self.flags.is_lines_count {
            write!(f, "lines: {}\n", self.counters.lines)?;
        }
        if self.flags.is_word_count {
            write!(f, "words: {}\n", self.counters.words)?;
        }
        if self.flags.is_char_count {
            write!(f, "chars: {}\n", self.counters.chars)?;
        }

        write!(f, "")
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("{USAGE_STR}");
        exit(1)
    }

    let mut flags = Flags {
        is_lines_count: false,
        is_word_count: false,
        is_char_count: false,
    };

    let mut files: Vec<PathBuf> = Vec::new();
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-l" => {
                flags.is_lines_count = true;
            }
            "-m" => {
                flags.is_char_count = true;
            }
            "-w" => {
                flags.is_word_count = true;
            }
            s if s.starts_with("-") => {
                eprintln!("unknown operator: {s}\n{USAGE_STR}");
                exit(1)
            }
            _ => {
                files.push(PathBuf::from(arg));
            }
        }
    }

    flags.check();

    for file in &files {
        let mut counters = Counters::new();

        let f = File::open(file).unwrap_or_else(|err| {
            eprintln!("error opening {}: {err}", file.display());
            exit(1);
        });

        let buffer = BufReader::new(f);
        for line in buffer.lines() {
            let line = line.unwrap_or_else(|err| {
                eprintln!("error reading line in {}: {err}", file.display());
                exit(1);
            });
            if flags.is_lines_count {
                // оригинальный wc считает по \n
                counters.lines += 1
            }
            if flags.is_word_count {
                counters.words += line.split_whitespace().count()
            }
            if flags.is_char_count {
                // подсчёт примитивный, эмодзи - мимо
                counters.chars += line.chars().count()
            }
        }

        let output = Output::new(&file, &counters, &flags);
        println!("{output}");
    }
}
