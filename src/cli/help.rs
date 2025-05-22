use crate::cli::cmd::CmdHandler;

pub struct CmdHelp;
impl CmdHandler for CmdHelp {
    fn new(_: Vec<String>) -> Self {
        Self {}
    }
    fn init(&mut self) {
        print_help();
    }
}

fn print_help() {
    print!(
        r#"kick - Project Template Generator

USAGE:
    kick <COMMAND> [OPTIONS]

COMMANDS:
    new <template> <project_name [--options]
        Creates project with given name using template

    list
        List all available templates in config dir

    help
        print commands and discription"#
    );
}
