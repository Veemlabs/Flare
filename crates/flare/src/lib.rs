pub mod error;
pub mod lexer;
pub mod parser;

pub use crate::lexer::token::Token;
pub use error::FlareError;
pub use lexer::lexer::Lexer;

pub struct Flare;

impl Flare {
    pub fn compile_from_string(source: &str) -> Result<(), FlareError> {
        let lexer = Lexer::<Token>::new(source);
        let _tokens: Result<Vec<_>, FlareError> = lexer.collect();
        Ok(())
    }
}
