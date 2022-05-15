#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn arithmetics(vec: &mut Vec<i32>, operator: &str) -> Result<(), String> {
    if vec.len() < 2 {
        return Err("Not enough operands".to_string());
    }

    let b = vec.pop().unwrap();
    let a = vec.pop().unwrap();

    match operator {
        "+" => vec.push(a + b),
        "-" => vec.push(a - b),
        "*" => vec.push(a * b),
        "/" => vec.push(a / b),
        _ => return Err("Operator not implemented".to_string()),
    }
    return Ok(());
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut vec = Vec::new();
    for input in inputs {
        let result = match input {
            CalculatorInput::Add => arithmetics(&mut vec, "+"),
            CalculatorInput::Subtract => arithmetics(&mut vec, "-"),
            CalculatorInput::Multiply => arithmetics(&mut vec, "*"),
            CalculatorInput::Divide => arithmetics(&mut vec, "/"),
            CalculatorInput::Value(value) => {
                vec.push(*value);
                Ok(())
            }
        };
        if result.is_err() {
            return None;
        }
    }
    if vec.len() == 1 {
        return Some(vec[0]);
    }
    return None;
}
