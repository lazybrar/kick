mod cli;
mod config;
mod utils;

use cli::{cmd::CmdHandler, help::CmdHelp, list::CmdList, new::CmdNew};
use std::io;

fn main() {
    // Skip first and pass other arguments.
    let mut args: Vec<String> = std::env::args().skip(0).collect();
    if args.len() > 1 {
        match args.remove(1).as_str() {
            "list" => CmdList::new(args).init(),
            "new" => CmdNew::new(args).init(),
            "help" => CmdHelp::new(args).init(),
            _ => {
                println!("command not found");
                CmdHelp::new(args).init();
            }
        }
    } else {
        CmdHelp::new(args).init();
    }
}
