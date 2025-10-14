use crate::error::FlareError;
use crate::lexer::token::TokenInfo;
use logos::{Lexer as LogosLexer, Logos};

pub struct Lexer<'src, T: Logos<'src, Source = str, Extras = ()>> {
    input: &'src str,
    inner: LogosLexer<'src, T>,
}

impl<'src, T: Logos<'src, Source = str, Extras = ()>> Lexer<'src, T> {
    pub fn new(input: &'src str) -> Self {
        Self {
            input,
            inner: T::lexer(input),
        }
    }
}

impl<'src, T: Logos<'src, Source = str, Extras = ()>> Iterator for Lexer<'src, T> {
    type Item = Result<TokenInfo<'src, T>, FlareError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next()? {
            Ok(kind) => {
                let span = self.inner.span();
                let text = &self.input[span.clone()];
                Some(Ok(TokenInfo { kind, span, text }))
            }
            Err(e) => {
                let span = self.inner.span();
                Some(Err(FlareError::InvalidToken {
                    error: format!("{:?}", e),
                    span,
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::Token;

    #[test]
    fn test_basic_tokens() {
        let source = "kernel matmul { }";
        let lexer = Lexer::<Token>::new(source);
        let tokens: Result<Vec<_>, FlareError> = lexer.collect();
        let tokens = tokens.unwrap();

        assert_eq!(tokens[0].kind, Token::Kernel);
        assert_eq!(tokens[1].kind, Token::Identifier("matmul".to_string()));
        assert_eq!(tokens[2].kind, Token::LeftBrace);
        assert_eq!(tokens[3].kind, Token::RightBrace);
    }
}
