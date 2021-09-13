fn main() {
    use interpreter_rs::repl;
    repl::start(std::io::stdin(), std::io::stdout())
}
