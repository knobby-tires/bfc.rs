use crate::parser::AstNode;

pub struct CodeGenerator {
    indentation: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            indentation: 0
        }
    }

    pub fn generate(&mut self, ast: &AstNode) -> String {
        let mut code = String::from(
            "fn main() {\n\
             let mut memory = vec![0u8; 30000];\n\
             let mut pointer = 0;\n\n"
        );

        match ast {
            AstNode::Program(instructions) => {
                for instruction in instructions {
                    code.push_str(&self.generate_instruction(instruction));
                }
            }
            _ => panic!("Expected program node"),
        }

        code.push_str("}\n");
        code
    }

    fn generate_instruction(&mut self, instruction: &AstNode) -> String {
        match instruction {
            AstNode::Increment => "    memory[pointer] = memory[pointer].wrapping_add(1);\n".to_string(),
            AstNode::Decrement => "    memory[pointer] = memory[pointer].wrapping_sub(1);\n".to_string(),
            AstNode::MoveRight => "    pointer += 1;\n".to_string(),
            AstNode::MoveLeft => "    pointer -= 1;\n".to_string(),
            AstNode::Output => "    print!(\"{}\", memory[pointer] as char);\n".to_string(),
            AstNode::Input => "    memory[pointer] = std::io::stdin().bytes().next().unwrap().unwrap();\n".to_string(),
            AstNode::Loop(instructions) => {
                let mut loop_code = String::from("    while memory[pointer] != 0 {\n");
                self.indentation += 1;
                for instruction in instructions {
                    loop_code.push_str(&self.generate_instruction(instruction));
                }
                self.indentation -= 1;
                loop_code.push_str("    }\n");
                loop_code
            },
            _ => String::new(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::AstNode;

    #[test]
    fn test_simple_program() {
        let program = AstNode::Program(vec![
            AstNode::Increment,
            AstNode::MoveRight,
            AstNode::Decrement,
        ]);
        
        let mut generator = CodeGenerator::new();
        let code = generator.generate(&program);
        
        assert!(code.contains("wrapping_add(1)"));
        assert!(code.contains("pointer += 1"));
        assert!(code.contains("wrapping_sub(1)"));
    }

    #[test]
    fn test_loop_generation() {
        let program = AstNode::Program(vec![
            AstNode::Loop(vec![
                AstNode::Increment,
                AstNode::Decrement,
            ]),
        ]);
        
        let mut generator = CodeGenerator::new();
        let code = generator.generate(&program);
        
        assert!(code.contains("while memory[pointer] != 0"));
        assert!(code.contains("wrapping_add(1)"));
        assert!(code.contains("wrapping_sub(1)"));
    }
}