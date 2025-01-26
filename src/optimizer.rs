use crate::parser::AstNode;



pub struct Optimizer;

impl Optimizer {
   pub fn new() -> Self {
       Optimizer
   }

   pub fn optimize(&self, ast: &AstNode) -> AstNode {
       println!("Starting optimization...");
       let result = match ast {
           AstNode::Program(instructions) => {
               println!("Optimizing program with {} instructions", instructions.len());
               AstNode::Program(self.optimize_instructions(instructions))
           }
           _ => ast.clone(),
       };
       println!("Optimization complete.");
       result
   }

   fn optimize_instructions(&self, instructions: &[AstNode]) -> Vec<AstNode> {
       println!("Optimizing instruction block..."); 
       let mut optimized = Vec::new();
       let mut i = 0;

       while i < instructions.len() {
           println!("Processing instruction {}/{}", i, instructions.len());
           match &instructions[i] {
               AstNode::Increment => {
                   println!("Found increment at position {}", i);
                   // Count consecutive increments
                   let mut count = 1;
                   while i + count < instructions.len() {
                       if let AstNode::Increment = instructions[i + count] {
                           count += 1;
                       } else {
                           break;
                       }
                   }
                   if count > 1 {
                       println!("Optimizing {} increments into Add({})", count, count);
                       // create an optimized increment
                       optimized.push(AstNode::Add(count));
                       i += count;
                   } else {
                       optimized.push(instructions[i].clone());
                       i += 1;
                   }
               },
               AstNode::Decrement => {
                   println!("Found decrement at position {}", i);
                   // same for decrements
                   let mut count = 1;
                   while i + count < instructions.len() {
                       if let AstNode::Decrement = instructions[i + count] {
                           count += 1;
                       } else {
                           break;
                       }
                   }
                   if count > 1 {
                       println!("Optimizing {} decrements into Sub({})", count, count);
                       optimized.push(AstNode::Sub(count));
                       i += count;
                   } else {
                       optimized.push(instructions[i].clone());
                       i += 1;
                   }
               },
               _ => {
                   println!("Found other instruction at position {}", i);
                   optimized.push(instructions[i].clone());
                   i += 1;
               }
           }
       }
       println!("Block optimization complete");
       optimized
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_optimize_increments() {
       let program = AstNode::Program(vec![
           AstNode::Increment,
           AstNode::Increment,
           AstNode::Increment,
       ]);
       
       let optimizer = Optimizer::new();
       let optimized = optimizer.optimize(&program);
       
       if let AstNode::Program(instructions) = optimized {
           assert_eq!(instructions.len(), 1);
           assert!(matches!(instructions[0], AstNode::Add(3)));
       } else {
           panic!("Expected Program node");
       }
   }

   #[test]
   fn test_optimize_mixed() {
       let program = AstNode::Program(vec![
           AstNode::Increment,
           AstNode::Increment,
           AstNode::MoveRight,
           AstNode::Decrement,
           AstNode::Decrement,
       ]);
       
       let optimizer = Optimizer::new();
       let optimized = optimizer.optimize(&program);
       
       if let AstNode::Program(instructions) = optimized {
           assert_eq!(instructions.len(), 3);
           assert!(matches!(instructions[0], AstNode::Add(2)));
           assert!(matches!(instructions[1], AstNode::MoveRight));
           assert!(matches!(instructions[2], AstNode::Sub(2)));
       } else {
           panic!("Expected Program node");
       }
   }
}