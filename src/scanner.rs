const C_DQUOTE: char = '"';

enum State {
    Bareword(usize),
    Begin,
    DQuote(usize),
    Whitespace,
}

pub struct Scanner<'a> {
    input: &'a str,
    lexemes: Vec<String>,
    state: State,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            lexemes: Vec::new(),
            state: State::Begin,
        }
    }

    pub fn scan(mut self) -> Vec<String> {
        for (i, c) in self.input.chars().enumerate() {
            match self.state {
                State::Bareword(word_start) if self.is_eol(i) => self.handle_eol(i, word_start),
                State::Bareword(word_start) => self.handle_bareword(c, i, word_start),
                State::Begin => self.handle_begin(c, i),
                State::DQuote(quote_start) => self.handle_dquote(c, i, quote_start),
                State::Whitespace => self.handle_whitespace(c, i),
            }
        }

        self.lexemes
    }

    fn handle_bareword(&mut self, c: char, i: usize, word_start: usize) {
        if c.is_whitespace() {
            self.lexemes.push(self.input[word_start..i].to_string());
            self.state = State::Whitespace;
        }
    }

    /// Handle transitions starting from state `Begin`.
    ///
    /// ```txt
    /// Begin -> Bareword ~ matching eg. foo
    ///                                  ^
    /// Begin -> DQuote ~ matching eg. "foo bar"
    ///                                ^
    /// Begin -> Whitespace
    /// ```
    ///
    /// NOTE `Bareword` and `DQuote` states carry the index to the start of the next
    /// lexeme slice.
    fn handle_begin(&mut self, c: char, i: usize) {
        if c.is_ascii_alphabetic() {
            self.state = State::Bareword(i);
        } else if c == C_DQUOTE {
            self.state = State::DQuote(i);
        } else if c.is_whitespace() {
            self.state = State::Whitespace;
        }
    }

    fn handle_dquote(&mut self, c: char, i: usize, quote_start: usize) {
        if c == C_DQUOTE {
            self.lexemes
                .push(self.input[quote_start..i + 1].to_string());
            self.state = State::Begin;
        }
    }

    fn handle_eol(&mut self, i: usize, word_start: usize) {
        self.lexemes.push(self.input[word_start..i + 1].to_string());
        self.state = State::Begin;
    }

    fn handle_whitespace(&mut self, c: char, i: usize) {
        if c.is_ascii_alphabetic() {
            self.state = State::Bareword(i);
        } else if c == C_DQUOTE {
            self.state = State::DQuote(i);
        }
    }

    /// Returns true if provided index points to the end of input.
    fn is_eol(&self, i: usize) -> bool {
        self.input.len() - 1 == i
    }
}
