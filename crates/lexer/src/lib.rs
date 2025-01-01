use std::{collections::HashMap, convert::Infallible, mem, str::CharIndices};

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
    // Input markers
    StartOfInput,
    EndOfInput,

    Comment,

    // Expression literals
    Ident,
    Number,

    // Various symbols
    LeftParen,
    RightParen,
    Semi,
    Colon,
    Equals,
    Comma,
    RArrow,

    // Math operators
    Multiply,
    Divide,
    Add,
    Subtract,

    // Keywords
    Func,
    End,
    If,
    Then,
    Else,

    UnknownTokenType,
}

pub type LalrpopToken = Result<(u32, TokenType, u32), Infallible>;

pub struct Lexer<'input> {
    incl_comments: bool,
    input: &'input str,
    char_iter: CharIndices<'input>,
    curr_char: Option<(usize, char)>,
    last_token: Option<TokenType>,
    keywords: HashMap<&'static str, TokenType>,
}

impl<'input> Lexer<'input> {
    const FUNC: &'static str = "func";
    const END: &'static str = "end";

    const IF: &'static str = "if";
    const THEN: &'static str = "then";
    const ELSE: &'static str = "else";

    pub fn new(input: &'input str, incl_comments: bool) -> Self {
        let mut keywords = HashMap::with_capacity(5);
        keywords.insert(Self::FUNC, TokenType::Func);
        keywords.insert(Self::END, TokenType::End);
        keywords.insert(Self::IF, TokenType::If);
        keywords.insert(Self::THEN, TokenType::Then);
        keywords.insert(Self::ELSE, TokenType::Else);

        Self {
            incl_comments,
            input,
            char_iter: input.char_indices(),
            curr_char: None,
            last_token: None,
            keywords,
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
            self.char_iter.next()
        }
    }

    fn scan_comment(&mut self, start_idx: usize) -> Option<LalrpopToken> {
        let mut len = 1;

        loop {
            match self.char_iter.next() {
                // End of comment - for now, we don't include the newline
                Some((idx, '\n')) => {
                    // Save this since not processed yet
                    self.curr_char = Some((idx, '\n'));
                    break;
                }
                // Valid comment char
                Some((_, char)) => len += char.len_utf8(),
                // EOI
                None => break,
            }
        }

        // Even if we are including comments, we don't save it as our last token so
        // new line semi colon logic works correctly
        if self.incl_comments {
            Some(Ok((
                start_idx as u32,
                TokenType::Comment,
                (start_idx + len) as u32,
            )))
        } else {
            None
        }
    }

    fn scan_number(&mut self, start_idx: usize) -> Option<LalrpopToken> {
        let mut len = 1;

        loop {
            match self.char_iter.next() {
                Some((idx, char)) => match char {
                    // ASCII digits - always length of 1
                    '0'..='9' => len += 1,
                    // If not a valid digit then don't consume and we are done
                    _ => {
                        // Save this since not processed yet
                        self.curr_char = Some((idx, char));
                        break;
                    }
                },
                // EOI
                None => break,
            }
        }

        self.emit_token(TokenType::Number, start_idx, len)
    }

    fn scan_ident_or_keyword(
        &mut self,
        start_idx: usize,
        start_len: usize,
    ) -> Option<LalrpopToken> {
        let mut len = start_len;

        loop {
            match self.char_iter.next() {
                // Underscore - always length of 1
                Some((_, '_')) => len += 1,
                // Alpha numberic
                // TODO: Since this matches ALL letters before numbers... it might
                // be slower than necessary for numbers. Pull out ASCII checking first?
                Some((_, char)) if char.is_alphanumeric() => len += char.len_utf8(),
                // Not valid ident char
                Some((idx, char)) => {
                    // Save this since not processed yet
                    self.curr_char = Some((idx, char));
                    break;
                }
                // EOI
                None => break,
            }
        }

        let ident = &self.input[start_idx..start_idx + len];

        match self.keywords.get(ident) {
            Some(token_type) => self.emit_token(*token_type, start_idx, len),
            None => self.emit_token(TokenType::Ident, start_idx, len),
        }
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
                        '#' => {
                            // If comments aren't included then skip
                            match self.scan_comment(idx) {
                                token @ Some(_) => token,
                                None => continue,
                            }
                        }
                        // TODO: Should we match other unicode whitespace chars?
                        // Whitespace
                        '\t' | ' ' | '\r' => continue,
                        '\n' => {
                            match self.last_token {
                                // If we ended with a special token, emit a semicolon
                                Some(
                                    TokenType::Ident | TokenType::Number | TokenType::RightParen,
                                ) => {
                                    // Semicolon actual token can be ';' OR '\n'
                                    self.emit_token(TokenType::Semi, idx, 1)
                                }
                                // In general, we do nothing
                                Some(_) | None => continue,
                            }
                        }
                        ':' => self.emit_token(TokenType::Colon, idx, 1),
                        '=' => self.emit_token(TokenType::Equals, idx, 1),
                        ',' => self.emit_token(TokenType::Comma, idx, 1),
                        '→' => self.emit_token(TokenType::RArrow, idx, '→'.len_utf8()),
                        ';' => self.emit_token(TokenType::Semi, idx, 1),
                        '(' => self.emit_token(TokenType::LeftParen, idx, 1),
                        ')' => self.emit_token(TokenType::RightParen, idx, 1),
                        '*' => self.emit_token(TokenType::Multiply, idx, 1),
                        '/' => self.emit_token(TokenType::Divide, idx, 1),
                        '+' => self.emit_token(TokenType::Add, idx, 1),
                        // Handle subtraction and right arrow disambiguation
                        '-' => match self.char_iter.next() {
                            Some((_, '>')) => self.emit_token(TokenType::RArrow, idx, 2),
                            Some((next_idx, char)) => {
                                // Save this since not processed yet
                                self.curr_char = Some((next_idx, char));
                                self.emit_token(TokenType::Subtract, idx, 1)
                            }
                            // EOI
                            None => self.emit_token(TokenType::Subtract, idx, 1),
                        },
                        // Start of integer literal
                        '1'..='9' => self.scan_number(idx),
                        // Next two are start of keyword or identifier
                        '_' => self.scan_ident_or_keyword(idx, 1),
                        // NOTE: This is last because if not ASCII, it might be slow determining if unicode alpha
                        _ if char.is_alphabetic() => {
                            self.scan_ident_or_keyword(idx, char.len_utf8())
                        }
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
    6 * 7 +  8 # This is a comment

    # This is another comment
    - 9 ;

    if then ifs else end
    _this_IS_an_Iß3NT

    func,:=->→
";

    #[test]
    fn test_lexer_no_comments() {
        test_lexer(false);
    }

    #[test]
    fn test_lexer_incl_comments() {
        test_lexer(true);
    }

    fn test_lexer(incl_comments: bool) {
        let mut lexer = Lexer::new(INPUT, incl_comments);
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
        if incl_comments {
            assert_eq!(lexer.next(), Some(Ok((25, TokenType::Comment, 44))));
        }
        // Special semi due to line ending in number (there is a space after the 8)
        assert_eq!(lexer.next(), Some(Ok((44, TokenType::Semi, 45))));

        if incl_comments {
            assert_eq!(lexer.next(), Some(Ok((50, TokenType::Comment, 75))));
        }
        assert_eq!(lexer.next(), Some(Ok((80, TokenType::Subtract, 81))));
        assert_eq!(lexer.next(), Some(Ok((82, TokenType::Number, 83))));
        assert_eq!(lexer.next(), Some(Ok((84, TokenType::Semi, 85))));

        assert_eq!(lexer.next(), Some(Ok((91, TokenType::If, 93))));
        assert_eq!(lexer.next(), Some(Ok((94, TokenType::Then, 98))));
        assert_eq!(lexer.next(), Some(Ok((99, TokenType::Ident, 102))));
        assert_eq!(lexer.next(), Some(Ok((103, TokenType::Else, 107))));
        assert_eq!(lexer.next(), Some(Ok((108, TokenType::End, 111))));

        assert_eq!(lexer.next(), Some(Ok((116, TokenType::Ident, 134))));
        // Special semi due to line ending in identifier
        assert_eq!(lexer.next(), Some(Ok((134, TokenType::Semi, 135))));

        assert_eq!(lexer.next(), Some(Ok((140, TokenType::Func, 144))));
        assert_eq!(lexer.next(), Some(Ok((144, TokenType::Comma, 145))));
        assert_eq!(lexer.next(), Some(Ok((145, TokenType::Colon, 146))));
        assert_eq!(lexer.next(), Some(Ok((146, TokenType::Equals, 147))));
        // ASCII right arrow
        assert_eq!(lexer.next(), Some(Ok((147, TokenType::RArrow, 149))));
        // Unicode  right arrow
        assert_eq!(lexer.next(), Some(Ok((149, TokenType::RArrow, 152))));

        assert_eq!(lexer.next(), Some(Ok((0, TokenType::EndOfInput, 0))));
        assert_eq!(lexer.next(), None);
        // It should keep returning None on successive attempts
        assert_eq!(lexer.next(), None);
    }
}
