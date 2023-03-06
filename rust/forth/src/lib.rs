use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, String>,
    words_counter: HashMap<String, i32>,
}

struct Word {
    pub adding: bool,
    pub key: String,
    pub value: String,
    pub original_key: String,
}
impl Word {
    fn new() -> Word {
        Word {
            adding: false,
            key: "".to_string(),
            value: "".to_string(),
            original_key: "".to_string(),
        }
    }
    fn concat_value(&mut self, new_value: &str) {
        if self.value != "" {
            self.value += " ";
        }

        self.value += new_value;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

const ADDITION: &str = "+";
const SUBTRAKTION: &str = "-";
const MULITPLICATION: &str = "*";
const DIVISION: &str = "/";
const KEYDELIMITER: &str = "€";
const STARTADDINGWORD: &str = ":";
const STOPADDINGWORD: &str = ";";

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            words: HashMap::new(),
            words_counter: HashMap::new(),
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let tokens = input.split(" ");
        let mut new_word = Word::new();

        for token in tokens.map(str::to_lowercase).into_iter() {
            match token.as_str() {
                STARTADDINGWORD => {
                    new_word.adding = true;
                    continue;
                }
                STOPADDINGWORD => {
                    self.insert_new_word(new_word);
                    new_word = Word::new();
                    continue;
                }
                _ => {}
            }

            if new_word.adding && new_word.key.is_empty() {
                if token.parse::<Value>().is_ok() {
                    return Err(Error::InvalidWord);
                }

                let idx_i = match self.words_counter.get(&token) {
                    Some(i) => *i,
                    None => 0,
                };

                new_word.key = self.generate_key(&token, idx_i);
                new_word.original_key = token;
                continue;
            }

            if new_word.adding {
                let value = match self.words_counter.get(&token) {
                    Some(i) => self.generate_key(&token, i - 1),
                    None => token,
                };

                new_word.concat_value(&value);
                continue;
            }

            match self.words_counter.get(&token) {
                Some(count) => {
                    let key = self.generate_key(&token, count - 1);
                    let maybe_word = self.words.get(&key);
                    self.eval(&maybe_word.unwrap().clone())?;
                    continue;
                }
                _ => {}
            }

            match self.words.get(&token) {
                Some(word) => {
                    self.eval(&word.clone())?;
                    continue;
                }
                _ => {}
            }

            match token.as_str() {
                ADDITION | SUBTRAKTION | MULITPLICATION | DIVISION => {
                    self.handle_arithmetic(&token)?;
                }
                _ => match token.parse::<Value>() {
                    Ok(n) => self.stack.push(n),
                    Err(_) => self.handle_manipulation(&token)?,
                },
            }
        }

        if new_word.adding {
            return Err(Error::InvalidWord);
        }

        Ok(())
    }

    fn insert_new_word(&mut self, new_word: Word) {
        self.words
            .insert(new_word.key.clone(), new_word.value.clone());

        let org_key = &new_word.original_key;
        match self.words_counter.get(org_key) {
            Some(i) => {
                self.words_counter.insert(org_key.clone(), i + 1).unwrap();
            }
            None => {
                self.words_counter.insert(org_key.clone(), 1);
            }
        };
    }

    fn generate_key(&self, key: &str, index: i32) -> String {
        KEYDELIMITER.to_owned() + &index.to_string() + KEYDELIMITER + key
    }

    fn get_two(&mut self) -> std::result::Result<(Value, Value), Error> {
        let a = match self.stack.pop() {
            Some(n) => n,
            None => return Err(Error::StackUnderflow),
        };
        let b = match self.stack.pop() {
            Some(n) => n,
            None => return Err(Error::StackUnderflow),
        };
        Ok((a, b))
    }

    fn handle_arithmetic(&mut self, op: &str) -> Result {
        let (a, b) = self.get_two()?;

        match op {
            ADDITION => self.stack.push(a + b),
            SUBTRAKTION => self.stack.push(b - a),
            MULITPLICATION => self.stack.push(a * b),
            DIVISION => {
                if a == 0 {
                    return Err(Error::DivisionByZero);
                }
                self.stack.push(b / a);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_manipulation(&mut self, word: &str) -> Result {
        match word.to_lowercase().as_str() {
            "dup" => {
                match self.stack.last() {
                    Some(last) => {
                        self.stack.push(*last);
                    }
                    None => {
                        return Err(Error::StackUnderflow);
                    }
                };
            }
            "drop" => {
                match self.stack.pop() {
                    Some(_) => {}
                    None => {
                        return Err(Error::StackUnderflow);
                    }
                };
            }
            "swap" => {
                let (a, b) = self.get_two()?;
                self.stack.push(a);
                self.stack.push(b);
            }
            "over" => {
                let (a, b) = self.get_two()?;
                self.stack.push(b);
                self.stack.push(a);
                self.stack.push(b);
            }
            _ => {
                return Err(Error::UnknownWord);
            }
        }
        Ok(())
    }
}
