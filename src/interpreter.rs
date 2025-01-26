// src/interpreter.rs 

// use std::hash::Hash;
use std::vec::Vec;
use crate::parser::AstNode;
use std::collections::HashMap;
use std::time::{Instant, Duration};

pub fn interpret_with_state(ast: &AstNode) -> Result<(String, Vec<u8>, usize), String> {
    let mut interpreter = Interpreter::new();
    interpreter.run_and_capture_output(ast)
}

pub struct Interpreter {
    memory: Vec<u8>,     // Memory tape
    pointer: usize,     // Data pointer
    tape_size: usize,    // 30k cells
    debug: bool,
    pub instruction_count: usize, // # instructions executed
    loop_depth: usize,        
    step_by_step: bool,
    instruction_times: HashMap<String, Duration>,
    instruction_counts: HashMap<String, usize>,
    loop_iterations: HashMap<usize, usize>, // loop_depth -> iteration count
    start_time: Option<Instant>,
    breakpoints: Breakpoints,
}

pub struct Breakpoints {
    instruction_count: Option<usize>,
    memory_value: Option<u8>,
    loop_depth: Option<usize>,
}

impl Interpreter {

    pub fn new() -> Self {
        const DEFAULT_TAPE_SIZE: usize = 30000;
        Interpreter {
            memory: vec![0; DEFAULT_TAPE_SIZE],
            pointer: 0,
            tape_size: DEFAULT_TAPE_SIZE,
            debug: false, 
            instruction_count: 0,
            loop_depth: 0,
            step_by_step: false,
            instruction_times: HashMap::new(),
            instruction_counts: HashMap::new(),
            loop_iterations: HashMap::new(),
            start_time: None,
            breakpoints: Breakpoints {
                instruction_count: None,
                memory_value: None,
                loop_depth: None,
            },
        }
    }

    // ==================== WEBASSEMBLY IMPLEMENTATIONS ============================

    pub fn run_and_capture_output(&mut self, ast: &crate::parser::AstNode) -> Result<(String, Vec<u8>, usize), String> {
        let mut output = String::new();
        
        match ast {
            crate::parser::AstNode::Program(instructions) => {
                for inst in instructions {
                    self.execute_instruction_capture(&mut output, inst)?;
                }
                Ok((output, self.memory.clone(), self.pointer))
            },
            _ => Err("Expected program node".to_string())
        }
    }

    // New execute method that captures output
    fn execute_instruction_capture(&mut self, output: &mut String, instruction: &AstNode) -> Result<(), String> {
        self.instruction_count += 1;
        self.debug_step(instruction);
        
        let start = Instant::now();
    
        let result = match instruction {
            AstNode::Output => {
                output.push(self.memory[self.pointer] as char);
                Ok(())
            },
            AstNode::Loop(instructions) => {
                self.loop_depth += 1;
                let mut loop_count = 0;
                
                while self.memory[self.pointer] != 0 {
                    loop_count += 1;
                    for instruction in instructions {
                        self.execute_instruction_capture(output, instruction)?;
                    }
                }
                
                *self.loop_iterations.entry(self.loop_depth).or_insert(0) += loop_count;
                self.loop_depth -= 1;
                Ok(())
            },
            AstNode::Increment => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
                Ok(())
            },
            AstNode::Decrement => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
                Ok(())
            },
            AstNode::Add(n) => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(*n as u8);
                Ok(())
            },
            AstNode::Sub(n) => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(*n as u8);
                Ok(())
            },
            AstNode::MoveRight => {
                if self.pointer + 1 >= self.tape_size {
                    return Err("Pointer out of bounds".to_string());
                }
                self.pointer += 1;
                Ok(())
            },
            AstNode::MoveLeft => {
                if self.pointer == 0 {
                    return Err("Pointer out of bounds".to_string());
                }
                self.pointer -= 1;
                Ok(())
            },
            AstNode::Input => {
                self.memory[self.pointer] = 0;
                Ok(())
            },
            _ => Err("Invalid instruction".to_string()),
        };

        let duration = start.elapsed();
        self.record_instruction(instruction, duration);
        
        result
    }

    pub fn interpret_with_state(ast: &AstNode) -> Result<(String, Vec<u8>, usize), String> {
        let mut interpreter = Interpreter::new();
        interpreter.run_and_capture_output(ast)
    }

    // ==================== BREAKPOINT IMPLEMENTATION FUNCTIONS ====================

    pub fn set_instruction_breakpoint(&mut self, count: usize) {
        self.breakpoints.instruction_count = Some(count);
    }

    pub fn set_memory_breakpoint(&mut self, value: u8) {
        self.breakpoints.memory_value = Some(value);
    }

    pub fn set_loop_breakpoint(&mut self, depth: usize) {
        self.breakpoints.loop_depth = Some(depth);
    }

    fn check_breakpoints(&self) -> bool {
        // check if any breakpoint condition is met
        if let Some(count) = self.breakpoints.instruction_count {
            if self.instruction_count == count {
                println!("\nBreakpoint hit: Instruction count = {}", count);
                return true;
            }
        }

        if let Some(value) = self.breakpoints.memory_value {
            if self.memory[self.pointer] == value {
                println!("\nBreakpoint hit: Memory value = {}", value);
                return true;
            }
        }

        if let Some(depth) = self.breakpoints.loop_depth {
            if self.loop_depth == depth {
                println!("\nBreakpoint hit: Loop depth = {}", depth);
                return true;
            }
        }

        false
    }

    // ================================== Stats Implementations ===========================================

    fn record_instruction(&mut self, instruction: &AstNode, duration: Duration) {
        let instruction_type = format!("{:?}", instruction);
        *self.instruction_counts.entry(instruction_type.clone()).or_insert(0) += 1;
        *self.instruction_times.entry(instruction_type).or_insert(Duration::new(0, 0)) += duration;
    }

    pub fn print_statistics(&self) {
        println!("\nExecution Statistics:");
        println!("Total instructions executed: {}", self.instruction_count);
        
        println!("\nInstruction counts:");
        for (instruction, count) in &self.instruction_counts {
            println!("{}: {} times", instruction, count);
        }

        println!("\nInstruction times:");
        for (instruction, time) in &self.instruction_times {
            println!("{}: {:?}", instruction, time);
        }

        println!("\nLoop statistics:");
        for (depth, iterations) in &self.loop_iterations {
            println!("Loop at depth {}: {} iterations", depth, iterations);
        }
    }

    pub fn set_step_by_step(&mut self, enabled: bool) {
        self.step_by_step = enabled;
    }

    fn debug_step(&self, instruction: &AstNode) {
        if self.debug {
            println!("\nStep {}:", self.instruction_count);
            println!("Loop depth: {}", self.loop_depth);
            println!("Executing: {:?}", instruction);
            println!("Pointer: {}", self.pointer);
            println!("Memory around pointer: {:?}", self.get_memory_window());
            
            if self.step_by_step {
                println!("\nPress Enter to continue...");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
            }
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn run(&mut self, ast: &crate::parser::AstNode) -> Result<(), String> {
        match ast {
            AstNode::Program(instructions) => {
                for instruction in instructions {
                    self.execute_instruction(instruction)?;
                }
                Ok(())
            }
            _=> Err("Expected program node".to_string()),
        }   
    }

    fn execute_instruction(&mut self, instruction: &AstNode) -> Result<(), String> {
        self.instruction_count += 1;
    
        // Check breakpoints before executing
        if self.check_breakpoints() {
            println!("Program paused at breakpoint.");
            println!("Current state:");
            println!("  Instruction: {:?}", instruction);
            println!("  Memory at pointer: {}", self.memory[self.pointer]);
            println!("  Loop depth: {}", self.loop_depth);
            
            println!("\nPress Enter to continue or 'q' to quit...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            
            if input.trim() == "q" {
                return Err("Execution terminated by user".to_string());
            }
        }
    
        self.debug_step(instruction);
        //start timing
        let start = Instant::now();
    
        let result = match instruction {
            AstNode::Increment => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
                Ok(())
            },
            AstNode::Decrement => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
                Ok(())
            },
            AstNode::Add(n) => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(*n as u8);
                Ok(())
            },
            AstNode::Sub(n) => {
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(*n as u8);
                Ok(())
            },
            AstNode::MoveRight => {
                if self.pointer + 1 >= self.tape_size {
                    return Err("Pointer out of bounds".to_string());
                }
                self.pointer += 1;
                Ok(())
            },
            AstNode::MoveLeft => {
                if self.pointer == 0 {
                    return Err("Pointer out of bounds".to_string());
                }
                self.pointer -= 1;
                Ok(())
            },
            AstNode::Output => {
                print!("{}", self.memory[self.pointer] as char);
                Ok(())
            },
            AstNode::Input => {
                use std::io::{stdin, Read};
                let mut input = [0];
                if stdin().read_exact(&mut input).is_ok() {
                    self.memory[self.pointer] = input[0];
                }
                Ok(())
            },
            AstNode::Loop(instructions) => {
                self.loop_depth += 1;
                let mut loop_count = 0;
                
                while self.memory[self.pointer] != 0 {
                    loop_count += 1;
                    for instruction in instructions {
                        self.execute_instruction(instruction)?;
                    }
                }

                // record loop iterations
                *self.loop_iterations.entry(self.loop_depth).or_insert(0) += loop_count;
                
                self.loop_depth -= 1;
                Ok(())
            },
            _ => Err("Invalid instruction".to_string()),
        };

        // record timing and stats
        let duration = start.elapsed();
        self.record_instruction(instruction, duration);

        if self.debug {
            // Show any changes after instruction execution
            println!("After execution:");
            println!("Memory around pointer: {:?}", self.get_memory_window());
        }
    
        result
    }

    // helper method for debug
    fn get_memory_window(&self) -> Vec<(usize, u8)> {
        // show 5 cells before and after pointer
        let start = self.pointer.saturating_sub(5);
        let end = self.pointer + 5.min(self.tape_size - 1);

        (start..=end)
        .map(|i| (i, self.memory[i]))
        .collect()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::AstNode;

    #[test]
    fn test_increment() {
        let mut interpreter = Interpreter::new();
        let program = AstNode::Program(vec![
            AstNode::Increment,
            AstNode::Increment,
        ]);
        interpreter.run(&program).unwrap();
        assert_eq!(interpreter.memory[0], 2);
    }

    #[test]
    fn test_decrement() {
        let mut interpreter = Interpreter::new();
        let program = AstNode::Program(vec![
            AstNode::Increment,
            AstNode::Increment,
            AstNode::Decrement,
        ]);
        interpreter.run(&program).unwrap();
        assert_eq!(interpreter.memory[0], 1);
    }

    #[test]
    fn test_move_pointer() {
        let mut interpreter = Interpreter::new();
        let program = AstNode::Program(vec![
            AstNode::Increment,
            AstNode::MoveRight,
            AstNode::Increment,
            AstNode::Increment,
        ]);
        interpreter.run(&program).unwrap();
        assert_eq!(interpreter.memory[0], 1);
        assert_eq!(interpreter.memory[1], 2);
    }

    #[test]
    fn test_simple_loop() {
        let mut interpreter = Interpreter::new();
        let program = AstNode::Program(vec![
            AstNode::Increment,
            AstNode::Increment,
            AstNode::Loop(vec![
                AstNode::Decrement,
            ]),
        ]);
        interpreter.run(&program).unwrap();
        assert_eq!(interpreter.memory[0], 0);
    }

    #[test]
    fn test_debug_mode() {
        let mut interpreter = Interpreter::new();
        interpreter.set_debug(true);
        let program = AstNode::Program(vec![
            AstNode::Increment,
            AstNode::MoveRight,
            AstNode::Add(3),
        ]);
        interpreter.run(&program).unwrap();
        assert_eq!(interpreter.memory[0], 1);
        assert_eq!(interpreter.memory[1], 3);
    }
}
    