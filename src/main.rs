mod lexer;

fn main() {
    let lexer = lexer::lexer::make();
    lexer.run();
}