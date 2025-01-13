mod lexer;
mod parser;
mod interpreter;
mod codegen;
mod optimizer;

use std::env;
use std::fs;

fn main() {
    // get arguments
    let args: Vec<String> = env::args().collect();
    
    let program = match args.len() {
        // no arguments, use default hello world
        1 => {
            println!("No input provided, running Hello World example:");
            "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
        },
        // file input
        2 => {
            println!("Reading from file: {}", args[1]);
            &fs::read_to_string(&args[1]).expect("Could not read file")
        },
        // program input
        3 if args[1] == "-p" => {
            println!("Running program: {}", args[2]);
            &args[2]
        },
        _ => {
            println!("Usage:");
            println!("  cargo run              # Run Hello World example");
            println!("  cargo run file.bf      # Run program from file");
            println!("  cargo run -p '++++.'   # Run program directly");
            println!("\nDebug options:");
            println!("  Add --debug            # Enable debug mode");
            println!("  Add --step             # Enable step-by-step");
            println!("  Add --stats            # Show execution statistics");
            return;
        }
    };

    // parse debug options
    let debug = args.contains(&"--debug".to_string());
    let step = args.contains(&"--step".to_string());
    let stats = args.contains(&"--stats".to_string());

    // run the program
    let mut lexer = lexer::Lexer::new(program);
    let tokens = lexer.tokenize();
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let mut interpreter = interpreter::Interpreter::new();
    interpreter.set_debug(debug);
    interpreter.set_step_by_step(step);
    
    match interpreter.run(&ast) {
        Ok(_) => {
            if stats {
                interpreter.print_statistics();
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}