use std::io::{self, Write};

use lambda::interpreter::Interpreter;

fn main() -> io::Result<()> {
    let mut interpreter = Interpreter::new();
    run_and_print(&mut interpreter, "(λx.λy.x) a b");
    run_and_print(&mut interpreter, "(λx.λy.x) ((λx.λy.y a) b)");
    run_and_print(&mut interpreter, "(λx.x) λx.x");
    run_and_print(&mut interpreter, "λx.λx.x x");
    run_and_print(&mut interpreter, "(λx. (λy. y) x) z");
    run_and_print(&mut interpreter, "(λx. λy. x y λy. y) y");

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    loop {
        print!("> ");
        stdout.flush()?;
        input.clear();
        if stdin.read_line(&mut input)? == 0 {
            break;
        }
        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }
        run_and_print(&mut interpreter, trimmed);
    }

    Ok(())
}

fn run_and_print(interpreter: &mut Interpreter, code: &str) {
    match interpreter.run(code) {
        Ok(mut term) => {
            term = interpreter.normalize(term);
            println!("{}", interpreter.format_term(&term));
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
}
// "(λw.λy.λx.y (w y x)) λs.λz.z"
// "(λx.λy.λs.λz.x s (y s z)) (λs.λz.s z) (λs.λz.s z)"
