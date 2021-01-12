mod opt;

use std::sync::mpsc::channel;

use clap::Clap;
use crossbeam_utils::thread;
use monkey::lexer::Lexer;
// use lexer::Lexer;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use opt::Opt;

fn main() {
    env_logger::init();

    let opt: Opt = Opt::parse();

    match opt.file_path {
        Some(path) => panic!("no implemented yet"),
        None => {
            let mut rl = Editor::<()>::new();
            if rl.load_history("history.txt").is_err() {
                println!("No previous history.")
            }
            loop {
                let readline = rl.readline(">> ");
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                        for token in Lexer::new(&line) {
                            println!("{:?}", token);
                        }
                    }
                    Err(ReadlineError::Interrupted) => {
                        println!("CTRL-C");
                        break;
                    }
                    Err(ReadlineError::Eof) => {
                        println!("CTRL-D");
                        break;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break;
                    }
                }
            }
            rl.save_history("history.txt").unwrap();
        }
    }
}
