use wasm_bindgen::prelude::*;

/// # State
///
/// Contains every value on the reverse polish notation calculator stack.
#[wasm_bindgen]
pub struct State {
    /// State handling for the "stack" uses a Vec that is treated as a stack.
    pub stack: Vec<f32>,
}

#[wasm_bindgen]
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

    /// Perform the operation passed as string on the last two values in the `stack`.  
    /// Expects a string matching one of the following: `+`, `-`, `*`, `/`.
    /// If a string is passed that doesn't match, the function will return the `State` unchanged.
    ///
    /// ``` rust
    /// use calculator::State;
    /// let state = State{ stack: vec![6.0, 2.0] };
    /// let state_prime = state.operate("/");
    /// assert_eq!(vec![3.0], state_prime.stack);
    /// ```
    ///
    /// > The trait takes a `&str` in order to process operations from TypeScript, through Wasm, by passing operations as a TypeScript `string`.
    ///
    pub fn operate(mut self, operation: &str) -> State {
        // Remove the last two values from `stack`
        let z = self.stack.pop().unwrap();
        let y = self.stack.pop().unwrap();

        // Return a `Vec<f32>` with the matching operation performed.
        let mut stack_tail = match operation {
            "+" => vec![y + z],
            "-" => vec![y - z],
            "*" => vec![y * z],
            "/" => vec![y / z],
            _ => vec![y, z],
        };
        // Return a new instance of `State` with the `stack_tail` appended.
        self.stack.append(&mut stack_tail);
        State { stack: self.stack }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let _ = State::new();
    }

    #[test]
    fn it_pushes() {
        // Given
        let expected_stack = vec![3.14159];
        let pushed_value = 3.14159;

        // When
        let state = State { stack: vec![] };
        let state_prime = state.push(pushed_value);

        // Then
        assert_eq!(expected_stack, state_prime.stack);
    }

    #[test]
    fn it_adds() {
        // Given
        let expected_stack = vec![6.0];
        let operation = "+";

        // When
        let state = State {
            stack: vec![2.0, 4.0],
        };
        let state_prime = state.operate(operation);

        // Then
        assert_eq!(expected_stack, state_prime.stack);
    }

    #[test]
    fn it_subtracts() {
        // Given
        let expected_stack = vec![-2.0];
        let operation = "-";

        // When
        let state = State {
            stack: vec![2.0, 4.0],
        };
        let state_prime = state.operate(operation);

        // Then
        assert_eq!(expected_stack, state_prime.stack);
    }

    #[test]
    fn it_multiplies() {
        // Given
        let expected_stack = vec![8.0];
        let operation = "*";

        // When
        let state = State {
            stack: vec![2.0, 4.0],
        };
        let state_prime = state.operate(operation);

        // Then
        assert_eq!(expected_stack, state_prime.stack);
    }

    #[test]
    fn it_divides() {
        // Given
        let expected_stack = vec![0.5];
        let operation = "/";

        // When
        let state = State {
            stack: vec![2.0, 4.0],
        };
        let state_prime = state.operate(operation);

        // Then
        assert_eq!(expected_stack, state_prime.stack);
    }

    #[test]
    fn it_passes() {
        // Given
        let expected_stack = vec![2.0, 4.0];
        let operation = "; drop tables *";

        // When
        let state = State {
            stack: vec![2.0, 4.0],
        };
        let state_prime = state.operate(operation);

        // Then
        assert_eq!(expected_stack, state_prime.stack);
    }
}
