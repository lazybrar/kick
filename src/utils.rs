use std::{io::Result, process::Command};

// basic command execution
// TODO add error msgs
pub fn exec_cmd(cmd: String) -> Result<()> {
    if !cmd.trim().is_empty() {
        let mut parts = cmd.split_whitespace();
        let program = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();
        let _ = Command::new(program).args(args).output()?;
    }
    Ok(())
}
