use std::{collections::HashMap, mem, str::CharIndices};

pub struct PosResolver<'input> {
    line: u32,
    col: u32,
    char_iter: CharIndices<'input>,
}

impl<'input> PosResolver<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            line: 1,
            col: 1,
            char_iter: input.char_indices(),
        }
    }

    pub fn pos_to_line_col(&mut self, pos: Pos) -> Option<(u32, u32)> {
        for (idx, c) in self.char_iter.by_ref() {
            let idx = idx as u32;
            let pos = pos.0;

            // Found the position
            if idx == pos {
                return Some((self.line, self.col));
            }

            // Oops.. out of order position passed in
            if idx > pos {
                return None;
            }

            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }

        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos(pub u32);

#[derive(Clone, Debug, PartialEq)]
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
    NumberLit,
    // true if it has escaped chars
    StringLit(bool),
    // true if it has escaped chars
    CharLit(bool),

    // Various symbols
    LeftParen,
    RightParen,
    Semi,
    Colon,
    Assign,
    Dot,
    Comma,
    RArrow,

    // Math operators
    Multiply,
    Divide,
    Plus,
    Minus,

    // Keywords
    Or,
    And,
    Not,
    True,
    False,
    Func,
    End,
    If,
    Then,
    Else,

    Error(TokenErrorKind),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenErrorKind {
    UnknownType,
    InvalidString(StringErrorKind),
    InvalidChar(CharErrorKind),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StringErrorKind {
    Unterminated,
    InvalidChar,
    InvalidEscape,
    InvalidHexEscape,
    InvalidUnicodeEscape,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CharErrorKind {
    Unterminated,
    CharTooLong,
    InvalidChar,
    InvalidEscape,
    InvalidHexEscape,
    InvalidUnicodeEscape,
}

pub type LalrpopToken = Result<(u32, TokenType, u32), &'static str>;

pub struct Lexer<'input> {
    incl_comments: bool,
    gen_input_markers: bool,
    input: &'input str,
    char_iter: CharIndices<'input>,
    curr_char: Option<(usize, char)>,
    last_token: Option<TokenType>,
    keywords: HashMap<&'static str, TokenType>,
}

impl<'input> Lexer<'input> {
    const TRUE: &'static str = "true";
    const FALSE: &'static str = "false";

    const OR: &'static str = "or";
    const AND: &'static str = "and";
    const NOT: &'static str = "not";

    const FUNC: &'static str = "func";
    const END: &'static str = "end";

    const IF: &'static str = "if";
    const THEN: &'static str = "then";
    const ELSE: &'static str = "else";

    pub fn new(input: &'input str, incl_comments: bool, gen_input_markers: bool) -> Self {
        let mut keywords = HashMap::with_capacity(7);
        keywords.insert(Self::TRUE, TokenType::True);
        keywords.insert(Self::FALSE, TokenType::False);
        keywords.insert(Self::OR, TokenType::Or);
        keywords.insert(Self::AND, TokenType::And);
        keywords.insert(Self::NOT, TokenType::Not);
        keywords.insert(Self::FUNC, TokenType::Func);
        keywords.insert(Self::END, TokenType::End);
        keywords.insert(Self::IF, TokenType::If);
        keywords.insert(Self::THEN, TokenType::Then);
        keywords.insert(Self::ELSE, TokenType::Else);

        Self {
            incl_comments,
            gen_input_markers,
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

        self.emit_token(TokenType::NumberLit, start_idx, len)
    }

    fn scan_hex_or_unicode_escape(
        &mut self,
        invalid_error_kind: TokenErrorKind,
        unterm_error_kind: TokenErrorKind,
        desired_len: usize,
    ) -> (usize, Option<TokenType>) {
        // The escape char + 'u' or 'x' char
        let mut len = 2;

        loop {
            match self.char_iter.next() {
                Some((_, char)) => match char {
                    '0'..='9' | 'a'..='f' | 'A'..='F' => {
                        // ASCII hex digits - always length of 1
                        len += 1;

                        // We need exactly desired_len - 2 hex digits after the 'u' or 'x'
                        if len == desired_len {
                            break;
                        }
                    }
                    // If not a valid hex digit then don't consume and we are done
                    _ => {
                        // Consume the invalid char (which could be unicode)
                        len += char.len_utf8();
                        return (len, Some(TokenType::Error(invalid_error_kind)));
                    }
                },
                // EOI
                None => {
                    return (len, Some(TokenType::Error(unterm_error_kind)));
                }
            }
        }

        (len, None)
    }

    fn scan_escape(
        &mut self,
        quote: char,
        invalid_error_kind: TokenErrorKind,
        invalid_hex_error_kind: TokenErrorKind,
        invalid_unicode_error_kind: TokenErrorKind,
        unterm_error_kind: TokenErrorKind,
    ) -> (usize, Option<TokenType>) {
        match self.char_iter.next() {
            Some((_, char)) => match char {
                // ASCII escaped chars - always length of 1
                '\\' | 'n' | 'r' | 't' | '0' => (2, None),
                // ASCII hex escape
                'x' => {
                    self.scan_hex_or_unicode_escape(invalid_hex_error_kind, unterm_error_kind, 4)
                }
                // Unicode escape
                'u' => self.scan_hex_or_unicode_escape(
                    invalid_unicode_error_kind,
                    unterm_error_kind,
                    8,
                ),
                // Escaped quote
                _ if char == quote => (2, None),
                // Invalid escape
                _ => (
                    char.len_utf8() + 1,
                    Some(TokenType::Error(invalid_error_kind)),
                ),
            },
            // Unexpected EOI
            None => (1, Some(TokenType::Error(unterm_error_kind))),
        }
    }

    fn scan_string(&mut self, start_idx: usize) -> Option<LalrpopToken> {
        // The opening quote
        let mut len = 1;
        let mut token = TokenType::StringLit(false);

        loop {
            match self.char_iter.next() {
                Some((_, char)) => match char {
                    // Escape start
                    '\\' => {
                        let (esc_len, token_err) = self.scan_escape(
                            '"',
                            TokenErrorKind::InvalidString(StringErrorKind::InvalidEscape),
                            TokenErrorKind::InvalidString(StringErrorKind::InvalidHexEscape),
                            TokenErrorKind::InvalidString(StringErrorKind::InvalidUnicodeEscape),
                            TokenErrorKind::InvalidString(StringErrorKind::Unterminated),
                        );
                        len += esc_len;

                        // Did we have a token error?
                        if let Some(token_err) = token_err {
                            token = token_err;
                        // We have seen an escaped char, but only set if not error
                        } else if let TokenType::StringLit(false) = token {
                            token = TokenType::StringLit(true);
                        }
                    }
                    // Closing quote
                    '"' => {
                        // ASCII quote - always length of 1
                        len += 1;
                        break;
                    }
                    // Newline and carriage return are not allowed in strings (must be escaped)
                    // TODO: Alternative, allow '\n' and strip '\r' later?
                    '\n' | '\r' => {
                        // We include this as part of string because otherwise it would be
                        // subject to the line ending semi colon logic. This might cause
                        // issues as it would be looking at the invalid string chars to decide
                        // if it should emit a semicolon.
                        //
                        // ASCII newline or carriage return - always length of 1
                        len += 1;

                        // We never saw closing quote
                        token = TokenType::Error(TokenErrorKind::InvalidString(
                            StringErrorKind::InvalidChar,
                        ));
                    }
                    // Valid char, add to string. Could be unicode, so we don't know the length
                    _ => len += char.len_utf8(),
                },
                // Unexpected EOI
                None => {
                    // We never saw closing quote
                    token = TokenType::Error(TokenErrorKind::InvalidString(
                        StringErrorKind::Unterminated,
                    ));
                    break;
                }
            }
        }

        self.emit_token(token, start_idx, len)
    }

    fn scan_char(&mut self, start_idx: usize) -> Option<LalrpopToken> {
        // The opening quote
        let mut len = 1;
        let mut token = TokenType::CharLit(false);
        let mut complete = false;

        loop {
            match self.char_iter.next() {
                Some((_, char)) => {
                    match char {
                        // Escape start
                        '\\' => {
                            let (esc_len, token_err) = self.scan_escape(
                                '\'',
                                TokenErrorKind::InvalidChar(CharErrorKind::InvalidEscape),
                                TokenErrorKind::InvalidChar(CharErrorKind::InvalidHexEscape),
                                TokenErrorKind::InvalidChar(CharErrorKind::InvalidUnicodeEscape),
                                TokenErrorKind::InvalidChar(CharErrorKind::Unterminated),
                            );
                            len += esc_len;

                            // Did we have a token error?
                            if let Some(token_err) = token_err {
                                token = token_err;
                            // We have seen an escaped char, but only set if not error
                            } else if let TokenType::CharLit(false) = token {
                                token = TokenType::CharLit(true);
                            }
                        }
                        // Closing quote
                        '\'' => {
                            // ASCII quote - always length of 1
                            len += 1;
                            break;
                        }
                        // Newline and carriage return are not allowed in chars (must be escaped)
                        // TODO: Alternative, allow '\n' and strip '\r' later?
                        '\n' | '\r' => {
                            // We include this as part of char because otherwise it would be
                            // subject to the line ending semi colon logic. This might cause
                            // issues as it would be looking at the invalid char to decide
                            // if it should emit a semicolon.
                            //
                            // ASCII newline or carriage return - always length of 1
                            len += 1;

                            // We never saw closing quote
                            token = TokenType::Error(TokenErrorKind::InvalidChar(
                                CharErrorKind::InvalidChar,
                            ));
                        }
                        // Valid. Could be unicode, so we don't know the length
                        _ => len += char.len_utf8(),
                    }

                    if !complete {
                        complete = true;
                    } else {
                        token = TokenType::Error(TokenErrorKind::InvalidChar(
                            CharErrorKind::CharTooLong,
                        ));
                    }
                }
                // Unexpected EOI
                None => {
                    // We never saw closing quote
                    token =
                        TokenType::Error(TokenErrorKind::InvalidChar(CharErrorKind::Unterminated));
                    break;
                }
            }
        }

        self.emit_token(token, start_idx, len)
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
            // Keyword
            Some(token_type) => self.emit_token(*token_type, start_idx, len),
            // Identifier
            None => self.emit_token(TokenType::Ident, start_idx, len),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = LalrpopToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.gen_input_markers && self.last_token.is_none() {
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
                                    TokenType::Ident
                                    | TokenType::NumberLit
                                    | TokenType::RightParen
                                    | TokenType::End,
                                ) => {
                                    // Semicolon actual token can be ';' OR '\n'
                                    self.emit_token(TokenType::Semi, idx, 1)
                                }
                                // In general, we do nothing
                                Some(_) | None => continue,
                            }
                        }
                        ':' => self.emit_token(TokenType::Colon, idx, 1),
                        '=' => self.emit_token(TokenType::Assign, idx, 1),
                        '.' => self.emit_token(TokenType::Dot, idx, 1),
                        ',' => self.emit_token(TokenType::Comma, idx, 1),
                        ';' => self.emit_token(TokenType::Semi, idx, 1),
                        '(' => self.emit_token(TokenType::LeftParen, idx, 1),
                        ')' => self.emit_token(TokenType::RightParen, idx, 1),
                        '*' => self.emit_token(TokenType::Multiply, idx, 1),
                        '/' => self.emit_token(TokenType::Divide, idx, 1),
                        '+' => self.emit_token(TokenType::Plus, idx, 1),
                        // Handle subtraction and right arrow disambiguation
                        '-' => match self.char_iter.next() {
                            Some((_, '>')) => self.emit_token(TokenType::RArrow, idx, 2),
                            Some((next_idx, char)) => {
                                // Save this since not processed yet
                                self.curr_char = Some((next_idx, char));
                                self.emit_token(TokenType::Minus, idx, 1)
                            }
                            // EOI
                            None => self.emit_token(TokenType::Minus, idx, 1),
                        },
                        '"' => self.scan_string(idx),
                        '\'' => self.scan_char(idx),
                        // Start of integer literal
                        '1'..='9' => self.scan_number(idx),
                        // Next two are start of keyword or identifier
                        '_' => self.scan_ident_or_keyword(idx, 1),
                        // NOTE: This is last because if not ASCII, it might be slow determining if unicode alpha
                        _ if char.is_alphabetic() => {
                            self.scan_ident_or_keyword(idx, char.len_utf8())
                        }
                        _ => self.emit_token(
                            TokenType::Error(TokenErrorKind::UnknownType),
                            idx,
                            char.len_utf8(),
                        ),
                    };
                }
                // EOF - output EOI token just once
                None if self.gen_input_markers => {
                    return match self.last_token {
                        Some(TokenType::EndOfInput) => None,
                        // There is no actual token or location. EOI is purely virtual.
                        _ => return self.emit_token(TokenType::EndOfInput, 0, 0),
                    };
                }
                None => return None,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{StringErrorKind, TokenErrorKind};

    use super::{Lexer, TokenType};

    fn lexer_single_token_test(input: &str, token_type: TokenType, start: u32, end: u32) {
        let mut lexer = Lexer::new(input, false, false);
        assert_eq!(lexer.next(), Some(Ok((start, token_type, end))));
        assert_eq!(lexer.next(), None);
    }

    // *** Single Token Tests ***

    #[test]
    fn string_basic() {
        lexer_single_token_test(r#""hello""#, TokenType::StringLit(false), 0, 7);
    }

    #[test]
    fn string_basic_unicode() {
        lexer_single_token_test(r#""helloß""#, TokenType::StringLit(false), 0, 9);
    }

    #[test]
    fn string_escapes() {
        lexer_single_token_test(
            r#" "\\\t\n\r\0\" \u012789 \uaBcDeF \x09 \xaF" "#,
            TokenType::StringLit(true),
            1,
            43,
        );
    }

    #[test]
    fn string_unterminated() {
        let tt = TokenType::Error(TokenErrorKind::InvalidString(StringErrorKind::Unterminated));
        lexer_single_token_test(r#""hello"#, tt, 0, 6);
        // In simple escape
        lexer_single_token_test(r#""\"#, tt, 0, 2);
        // In unicode escape
        lexer_single_token_test(r#""\u"#, tt, 0, 3);
        // In hex escape
        lexer_single_token_test(r#""\x"#, tt, 0, 3);
    }

    #[test]
    fn string_invalid_char() {
        let tt = TokenType::Error(TokenErrorKind::InvalidString(StringErrorKind::InvalidChar));
        let mut buffer = String::with_capacity(3);

        buffer.push('"');
        buffer.push('\n');
        buffer.push('"');
        lexer_single_token_test(&buffer, tt, 0, 3);

        buffer.clear();
        buffer.push('"');
        buffer.push('\r');
        buffer.push('"');
        lexer_single_token_test(&buffer, tt, 0, 3);
    }

    #[test]
    fn string_invalid_escape() {
        let tt = |kind| TokenType::Error(TokenErrorKind::InvalidString(kind));
        // In simple escape
        lexer_single_token_test(r#""\|""#, tt(crate::StringErrorKind::InvalidEscape), 0, 4);

        // In unicode escape
        lexer_single_token_test(
            r#""\u|""#,
            tt(crate::StringErrorKind::InvalidUnicodeEscape),
            0,
            5,
        );

        // In hex escape
        lexer_single_token_test(
            r#""\x|""#,
            tt(crate::StringErrorKind::InvalidHexEscape),
            0,
            5,
        );
    }

    // *** Char Tests ***

    #[test]
    fn char_basic() {
        lexer_single_token_test(r#"'a'"#, TokenType::CharLit(false), 0, 3);
    }

    #[test]
    fn char_basic_unicode() {
        lexer_single_token_test(r#"'ß'"#, TokenType::CharLit(false), 0, 4);
    }

    #[test]
    fn char_escapes() {
        lexer_single_token_test(r#"'\t'"#, TokenType::CharLit(true), 0, 4);
        lexer_single_token_test(r#"'\u019aEf'"#, TokenType::CharLit(true), 0, 10);
        lexer_single_token_test(r#"'\x0F'"#, TokenType::CharLit(true), 0, 6);
    }

    #[test]
    fn char_unterminated() {
        let tt = TokenType::Error(TokenErrorKind::InvalidChar(
            crate::CharErrorKind::Unterminated,
        ));
        lexer_single_token_test(r#"'a"#, tt, 0, 2);
        // In simple escape
        lexer_single_token_test(r#"'\"#, tt, 0, 2);
        // In unicode escape
        lexer_single_token_test(r#"'\u0"#, tt, 0, 4);
        // In hex escape
        lexer_single_token_test(r#"'\xF"#, tt, 0, 4);
    }

    #[test]
    fn char_invalid_char() {
        let tt = TokenType::Error(TokenErrorKind::InvalidChar(
            crate::CharErrorKind::InvalidChar,
        ));
        let mut buffer = String::with_capacity(3);

        buffer.push('\'');
        buffer.push('\n');
        buffer.push('\'');
        lexer_single_token_test(&buffer, tt, 0, 3);

        buffer.clear();
        buffer.push('\'');
        buffer.push('\r');
        buffer.push('\'');
        lexer_single_token_test(&buffer, tt, 0, 3);
    }

    #[test]
    fn char_invalid_escape() {
        let tt = |kind| TokenType::Error(TokenErrorKind::InvalidChar(kind));
        // In simple escape
        lexer_single_token_test(r#"'\|'"#, tt(crate::CharErrorKind::InvalidEscape), 0, 4);

        // In unicode escape
        lexer_single_token_test(
            r#"'\u|'"#,
            tt(crate::CharErrorKind::InvalidUnicodeEscape),
            0,
            5,
        );

        // In hex escape
        lexer_single_token_test(r#"'\x|'"#, tt(crate::CharErrorKind::InvalidHexEscape), 0, 5);
    }

    #[test]
    fn char_too_long() {
        let tt = TokenType::Error(TokenErrorKind::InvalidChar(
            crate::CharErrorKind::CharTooLong,
        ));
        lexer_single_token_test(r#"'ab'"#, tt, 0, 4);
    }

    // *** Bool Literal Tests ***

    #[test]
    fn bool_true() {
        lexer_single_token_test("true", TokenType::True, 0, 4);
    }

    #[test]
    fn bool_false() {
        lexer_single_token_test("false", TokenType::False, 0, 5);
    }

    // *** Boolean Op Tests ***

    #[test]
    fn bool_op() {
        lexer_single_token_test("or", TokenType::Or, 0, 2);
        lexer_single_token_test("and", TokenType::And, 0, 3);
        lexer_single_token_test("not", TokenType::Not, 0, 3);
    }

    // *** Full Lexer Tests ***

    #[test]
    fn lexer_full_no_comments() {
        lexer_full(false, true);
    }

    #[test]
    fn lexer_full_incl_comments() {
        lexer_full(true, true);
    }

    #[test]
    fn lexer_full_no_comments_or_markers() {
        lexer_full(false, false);
    }

    #[test]
    fn lexer_full_incl_comments_but_no_markers() {
        lexer_full(true, false);
    }

    fn lexer_full(incl_comments: bool, gen_input_markers: bool) {
        const INPUT: &str = r#" 123;(45)
    6 * 7 +  8 # This is a comment

    # This is another comment
    - 9 ;

    if then ifs else end
    _this_IS_an_Iß3NT

    func,:=->
    "This is a string!\n"
    "#;

        let mut lexer = Lexer::new(INPUT, incl_comments, gen_input_markers);
        if gen_input_markers {
            assert_eq!(lexer.next(), Some(Ok((0, TokenType::StartOfInput, 0))));
        }

        assert_eq!(lexer.next(), Some(Ok((1, TokenType::NumberLit, 4))));
        assert_eq!(lexer.next(), Some(Ok((4, TokenType::Semi, 5))));
        assert_eq!(lexer.next(), Some(Ok((5, TokenType::LeftParen, 6))));
        assert_eq!(lexer.next(), Some(Ok((6, TokenType::NumberLit, 8))));
        assert_eq!(lexer.next(), Some(Ok((8, TokenType::RightParen, 9))));
        // Special semi due to line ending in right parent
        assert_eq!(lexer.next(), Some(Ok((9, TokenType::Semi, 10))));

        assert_eq!(lexer.next(), Some(Ok((14, TokenType::NumberLit, 15))));
        assert_eq!(lexer.next(), Some(Ok((16, TokenType::Multiply, 17))));
        assert_eq!(lexer.next(), Some(Ok((18, TokenType::NumberLit, 19))));
        assert_eq!(lexer.next(), Some(Ok((20, TokenType::Plus, 21))));
        assert_eq!(lexer.next(), Some(Ok((23, TokenType::NumberLit, 24))));
        if incl_comments {
            assert_eq!(lexer.next(), Some(Ok((25, TokenType::Comment, 44))));
        }
        // Special semi due to line ending in number (there is a space after the 8)
        assert_eq!(lexer.next(), Some(Ok((44, TokenType::Semi, 45))));

        if incl_comments {
            assert_eq!(lexer.next(), Some(Ok((50, TokenType::Comment, 75))));
        }
        assert_eq!(lexer.next(), Some(Ok((80, TokenType::Minus, 81))));
        assert_eq!(lexer.next(), Some(Ok((82, TokenType::NumberLit, 83))));
        assert_eq!(lexer.next(), Some(Ok((84, TokenType::Semi, 85))));

        assert_eq!(lexer.next(), Some(Ok((91, TokenType::If, 93))));
        assert_eq!(lexer.next(), Some(Ok((94, TokenType::Then, 98))));
        assert_eq!(lexer.next(), Some(Ok((99, TokenType::Ident, 102))));
        assert_eq!(lexer.next(), Some(Ok((103, TokenType::Else, 107))));
        assert_eq!(lexer.next(), Some(Ok((108, TokenType::End, 111))));
        assert_eq!(lexer.next(), Some(Ok((111, TokenType::Semi, 112))));

        assert_eq!(lexer.next(), Some(Ok((116, TokenType::Ident, 134))));
        // Special semi due to line ending in identifier
        assert_eq!(lexer.next(), Some(Ok((134, TokenType::Semi, 135))));

        assert_eq!(lexer.next(), Some(Ok((140, TokenType::Func, 144))));
        assert_eq!(lexer.next(), Some(Ok((144, TokenType::Comma, 145))));
        assert_eq!(lexer.next(), Some(Ok((145, TokenType::Colon, 146))));
        assert_eq!(lexer.next(), Some(Ok((146, TokenType::Assign, 147))));
        assert_eq!(lexer.next(), Some(Ok((147, TokenType::RArrow, 149))));

        assert_eq!(
            lexer.next(),
            Some(Ok((154, TokenType::StringLit(true), 175)))
        );

        if gen_input_markers {
            assert_eq!(lexer.next(), Some(Ok((0, TokenType::EndOfInput, 0))));
        }
        assert_eq!(lexer.next(), None);
        // It should keep returning None on successive attempts
        assert_eq!(lexer.next(), None);
    }
}
