use std::io::{self, Write};

use lambda::{error::InterpreterError, interpreter::Interpreter};

fn main() -> anyhow::Result<()> {
    let mut interpreter = Interpreter::new();
    run_and_print(&mut interpreter, "(λx.λy.x) a b");
    run_and_print(&mut interpreter, "(λx.λy.x) ((λx.λy.y a) b)");
    run_and_print(&mut interpreter, "(λx.x) λx.x");
    run_and_print(&mut interpreter, "λx.λx.x x");
    run_and_print(&mut interpreter, "(λx. (λy. y) x) z");
    run_and_print(&mut interpreter, "(λx. λy. x y λy. y) y");
    run_and_print(&mut interpreter, "λx.f x");
    run_and_print(&mut interpreter, "λx.x x");
    run_and_print(&mut interpreter, "λs.λz.s z");

    assert!(is_equivalent(&mut interpreter, "λx.x z", "λy.y z")?);
    assert!(!is_equivalent(&mut interpreter, "λx.x u", "λy.y v")?);
    assert!(!is_equivalent(&mut interpreter, "λx.λy.x y", "λy.λx.x y")?);
    assert!(!is_equivalent(&mut interpreter, "λx.λx.x", "λy.λx.y")?);

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
        Ok(term) => {
            let term = interpreter.normalize(term);
            println!("{}", interpreter.format_term(&term));
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
}

fn is_equivalent(
    interpreter: &mut Interpreter,
    code1: &str,
    code2: &str,
) -> Result<bool, InterpreterError> {
    let t1 = interpreter.run(code1)?;
    let t1 = interpreter.normalize(t1);
    let t2 = interpreter.run(code2)?;
    let t2 = interpreter.normalize(t2);

    Ok(Interpreter::equivalent(&t1, &t2))
}

// "(λw.λy.λx.y (w y x)) λs.λz.z"
// "(λx.λy.λs.λz.x s (y s z)) (λs.λz.s z) (λs.λz.s z)"
