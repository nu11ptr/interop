use std::{convert::Infallible, mem, str::CharIndices};

pub struct Token {
    pub token_type: TokenType,
    start: u32,
    end: u32,
}

impl Token {
    pub fn new(token_type: TokenType, start: u32, end: u32) -> Self {
        Self {
            token_type,
            start,
            end,
        }
    }

    pub fn as_str_slice<'input>(&self, input: &'input str) -> &'input str {
        &input[self.start as usize..self.end as usize]
    }

    pub fn start_row_col(&self, input: &str) -> (u32, u32) {
        Self::row_col(self.start as usize, input)
    }

    pub fn end_row_col(&self, input: &str) -> (u32, u32) {
        Self::row_col(self.end as usize, input)
    }

    fn row_col(start_or_end: usize, input: &str) -> (u32, u32) {
        let mut line = 1;
        let mut col = 1;

        for (i, c) in input.char_indices() {
            if i == start_or_end {
                break;
            }

            if c == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }

        (line, col)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    StartOfInput,
    EndOfInput,

    Number,

    LeftParen,
    RightParen,
    Semi,

    Multiply,
    Divide,
    Add,
    Subtract,

    UnknownTokenType,
}

pub type LalrpopToken = Result<(u32, TokenType, u32), Infallible>;

pub struct Lexer<'input> {
    chars: CharIndices<'input>,
    curr_char: Option<(usize, char)>,
    last_token: Option<TokenType>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            chars: input.char_indices(),
            curr_char: None,
            last_token: None,
        }
    }

    fn emit_token(
        &mut self,
        token_type: TokenType,
        start_idx: usize,
        len: usize,
    ) -> Option<LalrpopToken> {
        self.last_token = Some(token_type);
        Some(Ok((start_idx as u32, token_type, (start_idx + len) as u32)))
    }

    fn next_char(&mut self) -> Option<(usize, char)> {
        if self.curr_char.is_some() {
            mem::take(&mut self.curr_char)
        } else {
            self.chars.next()
        }
    }

    fn make_number(&mut self, start_idx: usize) -> Option<LalrpopToken> {
        let mut len = 1;

        loop {
            match self.chars.next() {
                Some((idx, char)) => match char {
                    '0'..='9' => len += 1,
                    // If not a valid digit then don't consume and we are done
                    _ => {
                        // Save this since not processed yet
                        self.curr_char = Some((idx, char));
                        break;
                    }
                },
                None => break,
            }
        }

        self.emit_token(TokenType::Number, start_idx, len)
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = LalrpopToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last_token.is_none() {
            // There is no actual token or location. SOI is fully virtual.
            return self.emit_token(TokenType::StartOfInput, 0, 0);
        }

        loop {
            match self.next_char() {
                Some((idx, char)) => {
                    return match char {
                        // TODO: Should we match other unicode whitespace chars?
                        // Whitespace
                        '\t' | ' ' | '\r' => continue,
                        '\n' => {
                            match self.last_token {
                                // If we ended with a special token, emit a semicolon
                                Some(TokenType::Number | TokenType::RightParen) => {
                                    // Semicolon actual token can be ';' OR '\n'
                                    self.emit_token(TokenType::Semi, idx, 1)
                                }
                                // In general, we do nothing
                                Some(_) | None => continue,
                            }
                        }
                        ';' => self.emit_token(TokenType::Semi, idx, 1),
                        '(' => self.emit_token(TokenType::LeftParen, idx, 1),
                        ')' => self.emit_token(TokenType::RightParen, idx, 1),
                        '*' => self.emit_token(TokenType::Multiply, idx, 1),
                        '/' => self.emit_token(TokenType::Divide, idx, 1),
                        '+' => self.emit_token(TokenType::Add, idx, 1),
                        '-' => self.emit_token(TokenType::Subtract, idx, 1),
                        // Start of integer literal
                        '1'..='9' => self.make_number(idx),
                        _ => self.emit_token(TokenType::UnknownTokenType, idx, char.len_utf8()),
                    };
                }
                // EOF - output EOI token just once
                None => {
                    return match self.last_token {
                        Some(TokenType::EndOfInput) => None,
                        // There is no actual token or location. EOI is purely virtual.
                        _ => return self.emit_token(TokenType::EndOfInput, 0, 0),
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, TokenType};

    const INPUT: &str = r" 123;(45)
    6 * 7 +  8 
    - 9 ";

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new(INPUT);
        assert_eq!(lexer.next(), Some(Ok((0, TokenType::StartOfInput, 0))));

        assert_eq!(lexer.next(), Some(Ok((1, TokenType::Number, 4))));
        assert_eq!(lexer.next(), Some(Ok((4, TokenType::Semi, 5))));
        assert_eq!(lexer.next(), Some(Ok((5, TokenType::LeftParen, 6))));
        assert_eq!(lexer.next(), Some(Ok((6, TokenType::Number, 8))));
        assert_eq!(lexer.next(), Some(Ok((8, TokenType::RightParen, 9))));
        // Special semi due to line ending in right parent
        assert_eq!(lexer.next(), Some(Ok((9, TokenType::Semi, 10))));

        assert_eq!(lexer.next(), Some(Ok((14, TokenType::Number, 15))));
        assert_eq!(lexer.next(), Some(Ok((16, TokenType::Multiply, 17))));
        assert_eq!(lexer.next(), Some(Ok((18, TokenType::Number, 19))));
        assert_eq!(lexer.next(), Some(Ok((20, TokenType::Add, 21))));
        assert_eq!(lexer.next(), Some(Ok((23, TokenType::Number, 24))));
        // Special semi due to line ending in number (there is a space after the 8)
        assert_eq!(lexer.next(), Some(Ok((25, TokenType::Semi, 26))));

        assert_eq!(lexer.next(), Some(Ok((30, TokenType::Subtract, 31))));
        assert_eq!(lexer.next(), Some(Ok((32, TokenType::Number, 33))));

        assert_eq!(lexer.next(), Some(Ok((0, TokenType::EndOfInput, 0))));
        assert_eq!(lexer.next(), None);
        // It should keep returning None on successive attempts
        assert_eq!(lexer.next(), None);
    }
}
