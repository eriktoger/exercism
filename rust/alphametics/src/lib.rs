use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Data {
    char_to_digit: HashMap<char, u32>,
    columns: Vec<Vec<char>>,
    chars: HashSet<char>,
    questions_columns: Vec<char>,
    leading_chars: HashSet<char>,
}

fn calc_q_chars_in_columns(columns: Vec<Vec<char>>) -> Vec<Vec<(char, u32)>> {
    let mut q_chars_in_columns = vec![];

    for column in columns.iter() {
        let mut hm = HashMap::new();
        for letter in column.iter() {
            match hm.get(letter) {
                Some(l) => hm.insert(*letter, l + 1),
                None => hm.insert(*letter, 1),
            };
        }
        let v: Vec<(char, u32)> = hm.into_iter().collect();
        q_chars_in_columns.push(v);
    }
    return q_chars_in_columns;
}

fn calc_brute_force_solution(mut data: Data) -> Option<HashMap<char, u8>> {
    let digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let length = data.char_to_digit.len();
    let q_chars_in_columns = calc_q_chars_in_columns(data.columns);

    'outer: for perm in digits.iter().permutations(length).unique() {
        for (idx, c) in data.chars.iter().enumerate() {
            let val = perm[idx].clone();
            if val == 0 && data.leading_chars.iter().any(|lc| *c == *lc) {
                continue 'outer;
            }
            data.char_to_digit.insert(*c, val);
        }

        let mut carry = 0;
        for (idx, cc) in q_chars_in_columns.iter().enumerate() {
            let sum = cc.iter().fold(carry, |acc, (c, i)| {
                acc + data.char_to_digit.get(c).unwrap() * i
            });

            if sum % 10
                != *data
                    .char_to_digit
                    .get(&data.questions_columns[idx])
                    .unwrap()
            {
                continue 'outer;
            }
            carry = sum / 10;
        }

        return Some(
            data.char_to_digit
                .iter()
                .map(|(c, val)| (c.clone(), *val as u8))
                .collect(),
        );
    }

    None
}

fn sort_data(answer: &str, questions: Vec<&str>) -> Data {
    let mut char_to_digit: HashMap<char, u32> = HashMap::new();
    let mut chars = HashSet::new();
    let mut columns = vec![];
    let mut questions_columns = vec![];
    let mut leading_chars = HashSet::new();

    for idx in 0..(answer.len()) {
        let mut column = vec![];
        for question in questions.iter() {
            match question.chars().rev().nth(idx) {
                Some(letter) => {
                    if question.len() == idx + 1 {
                        leading_chars.insert(letter);
                    }

                    column.push(letter);
                    char_to_digit.insert(letter, 0);
                    chars.insert(letter);
                }
                None => {}
            }
        }
        let letter = answer.chars().rev().nth(idx).unwrap();

        questions_columns.push(letter);
        if answer.len() == idx + 1 {
            leading_chars.insert(letter);
        }
        columns.push(column);

        char_to_digit.insert(letter, 0);
        chars.insert(letter);
    }
    return Data {
        char_to_digit,
        columns,
        chars,
        questions_columns,
        leading_chars,
    };
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let q_n_a: Vec<&str> = input.split(" == ").collect();
    let questions: Vec<&str> = q_n_a[0].split(" + ").collect();
    let answer = q_n_a[1];

    if questions.len() == 1 {
        return None;
    }

    for question in questions.iter() {
        if question.len() > answer.len() {
            return None;
        }
    }
    let data = sort_data(answer, questions);
    calc_brute_force_solution(data)
}
