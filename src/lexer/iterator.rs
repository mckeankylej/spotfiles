use std::str::Chars;
use lexer::token::{Location, Token};

#[derive(Clone)]
struct CharsLoc<'a> {
    char_stream: Chars<'a>,
    chars:       &'a str,
    location:    Location,
    index:       usize
}

impl<'a> CharsLoc<'a> {
    fn new<'b>(input: &'b str) -> CharsLoc<'b> {
        CharsLoc {
            char_stream: input.chars(),
            chars:       input,
            location:    Location { line:   0,
                                    column: 0 },
            index:       0
        }
    }
    fn takeStr(&self, amount: usize) -> &'a str {
        &self.chars[(self.index - 1) .. amount]
    }
}

impl<'a> Iterator for CharsLoc<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let may_char = self.char_stream.next();
        match may_char {
            Some('\n') => {
                self.location.line += 1;
                self.location.column = 0;
                self.index += 1
            },
            Some(_) => {
                self.location.column += 1;
                self.index += 1;
            },
            None => ()
        }
        may_char
    }
}

pub struct TokenIterator<'a> {
    chars_loc:   CharsLoc<'a>,
    indentation: usize
}

fn is_reserved(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n' || c == '#'
}

impl<'a> TokenIterator<'a> {
    pub fn new<'b>(input: &'b str) -> TokenIterator<'b> {
        TokenIterator {
            chars_loc:   CharsLoc::new(input),
            indentation: 0
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars_loc.clone().next()
    }

    fn line_comment(&mut self) -> Token<'a> {
        let comment_loc = self.chars_loc.location;
        self.chars_loc.take_while(|c| c != &'\n');
        Token::newline(comment_loc)
    }

    fn newline(&mut self) -> Token<'a> {
        self.chars_loc.next();
        Token::newline(self.chars_loc.location)
    }

    fn name(&mut self) -> Token<'a> {
        let chars = self.chars_loc.chars;
        let len   = self.chars_loc.take_while(|&c| !is_reserved(c)).count();
        Token::name(self.chars_loc.location,self.chars_loc.takeStr(len))
    }

}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.peek_char() {
            Some(c) => match c {
                '#'  => Some(self.line_comment()),
                '\n' => Some(self.newline()),
                ' ' | '\t' => {
                    self.chars_loc.next();
                    self.next()
                },
                _    => Some(self.name())
            },
            None => None
        }
    }
}
