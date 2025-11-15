// mojo-like-rust / src/lexer.rs

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident(String),
    Number(String),
    Fn,
    Let,
    Mut,
    Return,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Arrow,      // ->
    FatArrow,   // =>
    Colon,
    Comma,
    Indent,
    Dedent,
    Newline,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut indent_stack: Vec<usize> = vec![0];

    let mut line_no = 1;

    for line in input.lines() {
        let trimmed = line.trim_start();
        let indent_len = line.len() - trimmed.len();

        // Indent / Dedent
        match indent_len.cmp(indent_stack.last().unwrap()) {
            std::cmp::Ordering::Greater => {
                indent_stack.push(indent_len);
                tokens.push(Token { kind: TokenKind::Indent, line: line_no, col: 1 });
            }
            std::cmp::Ordering::Less => {
                while indent_len < *indent_stack.last().unwrap() {
                    indent_stack.pop();
                    tokens.push(Token { kind: TokenKind::Dedent, line: line_no, col: 1 });
                }
            }
            _ => {}
        }

        let mut i = 0;

        let chars: Vec<char> = trimmed.chars().collect();
        while i < chars.len() {
            let c = chars[i];

            if c.is_whitespace() {
                i += 1;
                continue;
            }

            // numbers
            if c.is_ascii_digit() {
                let start = i;
                while i < chars.len() && chars[i].is_numeric() {
                    i += 1;
                }
                let text: String = chars[start..i].iter().collect();
                tokens.push(Token { kind: TokenKind::Number(text), line: line_no, col: start+1 });
                continue;
            }

            // identifiers / keywords
            if c.is_ascii_alphabetic() || c == '_' {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let text: String = chars[start..i].iter().collect();

                let kind = match text.as_str() {
                    "fn" => TokenKind::Fn,
                    "let" => TokenKind::Let,
                    "mut" => TokenKind::Mut,
                    "return" => TokenKind::Return,
                    _ => TokenKind::Ident(text),
                };

                tokens.push(Token { kind, line: line_no, col: start+1 });
                continue;
            }

            // symbols
            match c {
                '(' => tokens.push(Token { kind: TokenKind::LParen, line: line_no, col: i+1 }),
                ')' => tokens.push(Token { kind: TokenKind::RParen, line: line_no, col: i+1 }),
                '{' => tokens.push(Token { kind: TokenKind::LBrace, line: line_no, col: i+1 }),
                '}' => tokens.push(Token { kind: TokenKind::RBrace, line: line_no, col: i+1 }),
                ':' => tokens.push(Token { kind: TokenKind::Colon, line: line_no, col: i+1 }),
                ',' => tokens.push(Token { kind: TokenKind::Comma, line: line_no, col: i+1 }),

                '-' => {
                    if i + 1 < chars.len() && chars[i+1] == '>' {
                        tokens.push(Token { kind: TokenKind::Arrow, line: line_no, col: i+1 });
                        i += 2;
                        continue;
                    }
                }
                '=' => {
                    if i + 1 < chars.len() && chars[i+1] == '>' {
                        tokens.push(Token { kind: TokenKind::FatArrow, line: line_no, col: i+1 });
                        i += 2;
                        continue;
                    }
                }
                _ => {}
            }

            i += 1;
        }

        tokens.push(Token { kind: TokenKind::Newline, line: line_no, col: 1 });
        line_no += 1;
    }

    tokens.push(Token { kind: TokenKind::Eof, line: line_no, col: 1 });
    tokens
                  }
