//! module for performing lexical analysis on BrainFuck source code

use std::str;
use std::iter::Peekable;
use std::str::Chars;
use serde::{Serialize, Deserialize};

// tokenizer
// represents any valid token in the BrainFuck programming language.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
#[serde(tag = "type")]
pub enum Token {
   IncrementPtr, // >
   DecrementPtr, // 
   Increment,    // +
   Decrement,    // - 
   LoopStart,    // [
   LoopEnd,      // ]
   Input,        // ,
   Output,       // .
}

pub struct Lexer<'a> {
   input: Peekable<Chars<'a>>, // peekable iterator
   position: usize,            // tracks current position in the input
}

impl<'a> Lexer<'a> {
   // creates a new lexer instance from input string
   pub fn new(input: &'a str) -> Self {
       Lexer {
           // convert input string into peekable character iterator
           input: input.chars().peekable(),
           position: 0,
       }
   }

   pub fn next_token(&mut self) -> Option<Token> {
       while let Some(ch) = self.input.next() {
           self.position += 1;

           // match only valid BrainFuck commands
           let token = match ch {
               '+' => Some(Token::Increment),
               '-' => Some(Token::Decrement),
               '<' => Some(Token::DecrementPtr),
               '>' => Some(Token::IncrementPtr),
               '[' => Some(Token::LoopStart),
               ']' => Some(Token::LoopEnd),
               ',' => Some(Token::Input),
               '.' => Some(Token::Output),
               // ignore any other character
               _ => None,
           };

           if token.is_some() {
               return token;
           }
           // continue to next character if current char is a comment
       }
       None
   }

   // collect all tokens into a Vec
   pub fn tokenize(&mut self) -> Vec<Token> {
       let mut tokens = Vec::new();
       while let Some(token) = self.next_token() {
           tokens.push(token);
       }
       tokens
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_basic_tokens() {
       let mut lexer = Lexer::new("+-<>[].,");
       let tokens = lexer.tokenize();
       assert_eq!(tokens, vec![
           Token::Increment,
           Token::Decrement,
           Token::DecrementPtr,
           Token::IncrementPtr,
           Token::LoopStart,
           Token::LoopEnd,
           Token::Output,    // for .
           Token::Input     // for ,
       ]);
   }

   #[test]
   fn test_with_comments() {
       let mut lexer = Lexer::new("Hello + World - This is a comment! >");
       let tokens = lexer.tokenize();
       assert_eq!(tokens, vec![
           Token::Increment,
           Token::Decrement,
           Token::IncrementPtr,
       ]);
   }

   #[test]
   fn test_empty_input() {
       let mut lexer = Lexer::new("");
       let tokens = lexer.tokenize();
       assert_eq!(tokens.len(), 0);
   }
}