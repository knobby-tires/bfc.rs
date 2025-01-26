use wasm_bindgen::prelude::*;

mod lexer;
mod parser;
mod interpreter;
mod optimizer;
mod codegen;

// Struct to hold the execution state
#[wasm_bindgen]
pub struct ExecutionResult {
    output: String,
    memory: Vec<u8>,  
    pointer: usize,
    error: Option<String>,
    //stats: ExecutionStats,
}

#[wasm_bindgen]
impl ExecutionResult {
    #[wasm_bindgen(getter)]
    pub fn output(&self) -> String {
        self.output.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn memory(&self) -> Vec<u8> {
        self.memory.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn pointer(&self) -> usize {
        self.pointer
    }

    #[wasm_bindgen(getter)]
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
}

#[wasm_bindgen]
pub fn compile_and_run(input: &str) -> ExecutionResult {
    let result: Result<ExecutionResult, String> = (|| {
        let tokens = lexer::tokenize(input)?;
        let ast = parser::parse(tokens)?;
        let optimized = optimizer::Optimizer::new().optimize(&ast);
        let (output, memory, pointer) = interpreter::interpret_with_state(&optimized)?;
        
        Ok(ExecutionResult {
            output,
            memory,
            pointer,
            error: None,
        })
    })();

    // Handle any errors.
    match result {
        Ok(execution_result) => execution_result,
        Err(e) => ExecutionResult {
            output: String::new(),
            memory: vec![0; 30],  
            pointer: 0,
            error: Some(format!("Error: {}", e)),
        }
    }
}
