use wasm_bindgen::prelude::*;

/// Perform the operation
#[wasm_bindgen]
pub fn operate(a: f64, b: f64, operation: String) -> Result<f64, String> {
    match &operation[..] {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => divide(a, b),
        _ => Err(format!(
            "Operation string `{}` does not match any allowed operation (+, -, *, /)",
            &operation[..]
        )),
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    // Preemptively catch the division by zero panic case.
    // See documentation for [Div](https://doc.rust-lang.org/std/ops/trait.Div.html)
    match b {
        0.0 => Err(String::from("Division by zero is not ok.")),
        _ => Ok(a / b),
    }
}

/// # State
///
/// Contains every value on the reverse polish notation calculator stack.
pub struct State {
    /// State handling for the "stack" uses a Vec that is treated as a stack.
    pub stack: Vec<f32>,
}

impl State {
    /// Creates a new instance of `State` with a stack of 4 zeros.
    pub fn new() -> State {
        State {
            stack: vec![0.0, 0.0, 0.0, 0.0],
        }
    }

    /// Pushes `value` to `State.stack` then creates a new instance of `State`
    /// using the appended to `stack`
    pub fn push(mut self, value: f32) -> State {
        self.stack.push(value);
        State { stack: self.stack }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
