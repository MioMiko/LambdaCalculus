use lambda::interpreter::Interpreter;

fn main() {
    let mut intepreter = Interpreter::new();
    intepreter.run("(λx.λy.x) a b");
    intepreter.run("(λx.λy.x) ((λx.λy.y a) b)");
    intepreter.run("λx.x λx.x");
    intepreter.run("λx.λx.x x");
    intepreter.run("(λx. (λy. y) x) z");
}
