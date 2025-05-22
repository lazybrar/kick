pub trait CmdHandler {
    fn new(args: Vec<String>) -> Self;
    fn init(&mut self);
}
