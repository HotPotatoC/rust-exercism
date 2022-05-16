#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut result_stack: Vec<i32> = Vec::new();

    for input in inputs {
        match input {
            CalculatorInput::Value(n) => {
                result_stack.push(*n);
            }
            _ => {
                if result_stack.len() < 2 {
                    return None;
                }

                let (y, x) = (result_stack.pop().unwrap(), result_stack.pop().unwrap());

                match input {
                    CalculatorInput::Add => result_stack.push(x + y),
                    CalculatorInput::Subtract => result_stack.push(x - y),
                    CalculatorInput::Multiply => result_stack.push(x * y),
                    CalculatorInput::Divide => result_stack.push(x / y),
                    _ => {}
                }
            }
        }
    }

    if result_stack.len() != 1 {
        return None;
    }

    result_stack.pop()
}
