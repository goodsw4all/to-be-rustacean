use std::ptr::NonNull;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn calculate(left: i32, right: i32, operator: &CalculatorInput) -> i32 {
    match operator {
        CalculatorInput::Add => left + right,
        CalculatorInput::Subtract => left - right,
        CalculatorInput::Multiply => left * right,
        CalculatorInput::Divide => left / right,
        _ => 0, // never happen
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut operands: Vec<i32> = Vec::new();

    for item in inputs {
        match item {
            CalculatorInput::Value(n) => operands.push(*n),
            _ => {
                let right = match operands.pop() {
                    Some(n) => n,
                    None => return None,
                };
                let left = match operands.pop() {
                    Some(n) => n,
                    None => return None,
                };
                operands.push(calculate(left, right, item));
            }
        }
    }
    if operands.len() == 1 {
        return Some(*operands.first().unwrap());
    }
    None

    // unimplemented!(
    // 	"Given the inputs: {:?}, evaluate them as though they were a Reverse Polish notation expression",
    // 	inputs,
    // );
}
