pub struct Token {
    // "keyword", "identifier", "comment", "oparen", "cparen", "obrace", "cbrace", "obracket", "cbracket", "comma", "semicolon", "string", "number", "operato"
    pub token_type: String,
    // e.g. "function", "foo", "bar", "1", "+", ";"
    pub value: String,
}

impl Token {
    pub fn new(token_type: String, value: String) -> Self {
        Self { token_type, value }
    }
}
