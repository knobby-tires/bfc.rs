use crate::lexer::Token;

pub fn parse(tokens: Vec<Token>) -> Result<AstNode, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

// Define AST node types 
#[derive(Debug, Clone, PartialEq)]

// defines what our abstract syntax tree looks like 
// each node can be a basic instruciton or a container
pub enum AstNode {
   Program(Vec<AstNode>), // root node containing all instrutions
   Loop(Vec<AstNode>),    // loop with its body instructions
   Increment,             // +
   Decrement,             // -
   MoveRight,             // >
   MoveLeft,              // < 
   Input,                 // ,
   Output,                // .
   Add(usize),    // optimized multiple increments
   Sub(usize),    // optimized multiple decrements
}

pub struct Parser {
   tokens: Vec<Token>, // input tokens from lexer 
   position: usize,    // current position in token stream 
}

impl Parser {
   pub fn new(tokens: Vec<Token>) -> Self {
       Parser {
           tokens,
           position: 0,
       }
   }

   // entry point for parsing
   pub fn parse(&mut self) -> Result<AstNode, String> {
       self.parse_program()
   }

   // parses entire program
   fn parse_program(&mut self) -> Result<AstNode, String> {
       let mut instructions = Vec::new();
       
       while !self.is_at_end() {
           match self.peek() {
               None => {
                   if self.looking_for_loop_end() {
                       return Err("Unexpected end of input - unclosed loop".to_string());
                   }
                   return Err("Unexpected end of input".to_string());
               }
               Some(token) => {
                   match *token {
                       Token::Increment => {
                           instructions.push(AstNode::Increment);
                           self.advance();
                       },
                       Token::Decrement => {
                           instructions.push(AstNode::Decrement);
                           self.advance();
                       },
                       Token::IncrementPtr => {
                           instructions.push(AstNode::MoveRight);
                           self.advance();
                       },
                       Token::DecrementPtr => {
                           instructions.push(AstNode::MoveLeft);
                           self.advance();
                       },
                       Token::Input => {
                           instructions.push(AstNode::Input);
                           self.advance();
                       },
                       Token::Output => {
                           instructions.push(AstNode::Output);
                           self.advance();
                       },
                       Token::LoopStart => {
                        self.advance(); // move past [ character
                        let loop_body = self.parse_program()?;
                        let body_instructions = match loop_body {
                            AstNode::Program(nodes) => {
                                if nodes.is_empty() {
                                    Vec::new()
                                } else {
                                    nodes
                                }
                            },
                            _ => return Err("Expected program node from loop body".to_string())
                        };
                        instructions.push(AstNode::Loop(body_instructions));
                    },
                       Token::LoopEnd => {
                           self.advance(); // move past ] character
                           return Ok(AstNode::Program(instructions));
                       }
                   }
               }
           }
       }
       
       if self.looking_for_loop_end() {
           return Err("Unclosed loop - missing ]".to_string());
       }
       Ok(AstNode::Program(instructions))
   }

   // helper to check if we are at the end
   fn is_at_end(&self) -> bool {
       self.position >= self.tokens.len()
   }

   // helper to peek at current token 
   fn peek(&self) -> Option<&Token> {
       self.tokens.get(self.position)
   }

   // helper to advance to next token
   fn advance(&mut self) -> Option<&Token> {
       if !self.is_at_end() {
           self.position += 1;
       }
       self.tokens.get(self.position - 1)
   }

   // helper to check if we're in a loop
   fn looking_for_loop_end(&self) -> bool {
       let mut depth = 0;
       for i in 0..self.position {
           match self.tokens[i] {
               Token::LoopStart => depth += 1,
               Token::LoopEnd => depth -= 1,
               _ => {}
           }
       }
       depth > 0
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use crate::lexer::Lexer;

   #[test]
   fn test_basic_loop() {
       let input = "+[-]";
       let mut lexer = Lexer::new(input);
       let tokens = lexer.tokenize();
       let mut parser = Parser::new(tokens);
       
       let ast = parser.parse().unwrap();
       assert!(matches!(ast, AstNode::Program(_)));
       if let AstNode::Program(instructions) = ast {
           assert_eq!(instructions.len(), 2);
           assert_eq!(instructions[0], AstNode::Increment);
           assert!(matches!(instructions[1], AstNode::Loop(_)));
           if let AstNode::Loop(loop_body) = &instructions[1] {
               assert_eq!(loop_body.len(), 1);
               assert_eq!(loop_body[0], AstNode::Decrement);
           }
       }
   }

   #[test]
fn test_nested_loops() {
   let input = "+[[-]]";
   let mut lexer = Lexer::new(input);
   let tokens = lexer.tokenize();
   let mut parser = Parser::new(tokens);
   
   let ast = parser.parse().unwrap();
   assert!(matches!(ast, AstNode::Program(_)));
   if let AstNode::Program(instructions) = ast {
       assert_eq!(instructions.len(), 2);
       assert_eq!(instructions[0], AstNode::Increment);
       assert!(matches!(instructions[1], AstNode::Loop(_)));
       if let AstNode::Loop(outer_loop) = &instructions[1] {
           assert_eq!(outer_loop.len(), 1);
           assert!(matches!(outer_loop[0], AstNode::Loop(_)));
           if let AstNode::Loop(inner_loop) = &outer_loop[0] {
               assert_eq!(inner_loop.len(), 1);
               assert_eq!(inner_loop[0], AstNode::Decrement);
           }
       }
   }
}

   #[test]
   fn test_unmatched_brackets() {
       let input = "+[[-]";  // Missing closing bracket
       let mut lexer = Lexer::new(input);
       let tokens = lexer.tokenize();
       let mut parser = Parser::new(tokens);
       
       let result = parser.parse();
       assert!(result.is_err());
   }

   #[test]
   fn test_empty_program() {
       let input = "";
       let mut lexer = Lexer::new(input);
       let tokens = lexer.tokenize();
       let mut parser = Parser::new(tokens);
       
       let ast = parser.parse().unwrap();
       if let AstNode::Program(instructions) = ast {
           assert_eq!(instructions.len(), 0);
       }
   }

   #[test]
   fn test_multiple_instructions() {
       let input = "+-><.,";
       let mut lexer = Lexer::new(input);
       let tokens = lexer.tokenize();
       let mut parser = Parser::new(tokens);
       
       let ast = parser.parse().unwrap();
       if let AstNode::Program(instructions) = ast {
           assert_eq!(instructions.len(), 6);
           assert_eq!(instructions[0], AstNode::Increment);
           assert_eq!(instructions[1], AstNode::Decrement);
           assert_eq!(instructions[2], AstNode::MoveRight);
           assert_eq!(instructions[3], AstNode::MoveLeft);
           assert_eq!(instructions[4], AstNode::Output);
           assert_eq!(instructions[5], AstNode::Input);
       }
   }
}