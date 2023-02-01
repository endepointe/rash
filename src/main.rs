
//use std::fs;
use std::io::{self,Write};
use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::process;

fn main() -> Result<(), io::Error> {
    // allow cli options to be set before rash occurs
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("rash option: {}", config.opts);
    println!("rash command: {}", config.cmds);

    // setup the rash
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;
    let mut input = String::new();
    let EXIT = String::from("exit");

    // allow the rash to run until cleared
    while !term.load(Ordering::Relaxed) {
        io::stdout().write_all(b"rash#>")?;
        io::stdout().flush()?; 
        io::stdin().read_line(&mut input)?;
        if input == "exit\r\n" { println!("exiting..."); break; }
        println!("{}",input);
        input.clear();
    }

    Ok(())
}


struct Config {
    opts: String,
    cmds: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 2 {
            let opts = String::from("-D");
            let cmds = String::from("test");
            Config {opts, cmds}
        } else {
            let opts = args[1].clone();
            let cmds = args[2].clone();
            Config {opts,cmds}
        }
    }

}
