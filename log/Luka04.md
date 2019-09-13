# Luka 04

Date: 11 & 12 September 2019

At the end of Luka 03, I wasn't sure whether to work on connecting the Rust and TypeScript using Wasm or organize how state is passed in the Vue application. I decided to take on the former first. I think I'm going to hit some interesting walls that might affect how I structure the Vue application.

## Finding References

I've connected Rust and JavaScript once in a tutorial, but I don't have a great understanding of how that connection works and how to set it up properly. That's part of the motivation for this project -- pushing myself to learn.  

Here are some resources I'm reading through to understand the structure:

- [Rust Wasm Official Website](https://rustwasm.github.io/)
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [Rust and WebAssembly GitHub](https://github.com/rustwasm)
- [Wasm Bindgen Book](https://rustwasm.github.io/docs/wasm-bindgen/)

I don't think I'll need `wasm-pack` as I am adding Wasm to an existing project, but I downloaded the latest version anyway (0.8.1). I know I will need `wasm-bindgen` as a dependency.

``` toml
[dependencies]
wasm-bindgen = "0.2.50"
```

I think we're off to a great start.

I see references to using `wee_alloc` to minimize the amount of Wasm that is generated. I think that sounds nice, but I won't start with that yet.

To the `lib.rs`, I'm adding the import for `wasm-bindgen` decorators. All the tests still pass so that's good. It turns out I do need `wasm-pack` to build the `.wasm` file (probably another way to do it, but this is what's showing up in the tutorial).

First attempt at building the `.wasm` gives a good, informational error message.

``` text
Error: crate-type must be cdylib to compile to wasm32-unknown-unknown. Add the following to your Cargo.toml file:

[lib]
crate-type = ["cdylib", "rlib"]
```

Ok. I added that bit to the `cargo.toml`.

It... worked. I mean, I didn't decorate any functions so it didn't generate any functions on the wasm side, but still. Wow.

Now to decorate the functions. I'm not sure if I have to decorate every function or if I can get away with just decorating the `struct` and `impl`...

Well it failed, but it failed because the `std::vec::Vec<f64>` doesn't implement `std::marker::Copy`. Looking at the Rust docs, this is true for all `Vec<T>` because it's managing some other resource as evidenced by its implementation of `Drop`.

I can't pass a `Vec<T>`? That's confusing because how do they do that in the `Universe` struct [here](https://rustwasm.github.io/docs/book/game-of-life/implementing.html)? I'm definitely not understanding something here.

If I truly can't pass a `Vec<T>`, I really only need to worry about the last two values of the stack in the Rust code at any given time if I have the TypeScript manage the full stack state. A workable solution could look like this:

1. TypeScript holds state `[4.5, 3, 99, 0.5]`.
2. The "+" button is pressed.
3. The last two values in the stack are passed to Rust `(99, 0.5)`.
4. Rust returns `(99.5)`.
5. TypeScript changes the stack to read `[4.5, 3, 99.5]`.

This actually seems like a better design. It minimizes the amount of state passing and gives the stack a clear owner.

## Have you tried rewriting this in Rust?

Instead of a `State` struct, everything in this new implementation is going to be a function. Every function will take in two numbers and return a `Result` type that will give the result of the numbers undergoing the operation or an error. For operations (+, -, *, /), there will be an `operate` function that takes in the two numbers and a string matching one of those operations then handing the state back to the TypeScript.

There is only one big problem. And it's one I would have had in the earlier implementation anyway.

The `+`, `-`, and `*` functions in Rust all return primitive types as if their input types are okay, there are no cases in which a primitive number cannot be returned. However, `/` has one horrific edge case: division by zero. If you are designing Rust, you have to make a choice. Either the `/` function is the only function that returns a `Result` or `Option` type, or the function panics if division by zero is attempted and the return type is primitive which matches the other functions ergonomically.

In the former case, handling division would be a whole mess:

``` Rust
//                                 Type
let a = 2                       // i32
let b = 0                       // i32

let added = 2 + 0               // i32
let subtracted = 2 - 0          // i32
let multiplied = 2 * 0          // i32
let divided = 2 / 0             // Err

match divided {
    Ok(quotient) => quotient,   // i32 (or some float maybe)
    Err(message) => message,    // String
}
```

I mean, what? You want to unpack the quotient every time you divide? I think they made the right decision that if you try to divide by zero, division `panics!`. However, I don't know what this will do across the Wasm boundary. Here is one way I _could_ solve it:

``` rust
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
```

Except matching on a float won't be possible in future versions of Rust. This code, on compilation gives the error: 

``` text
warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>
```

## Wrap Up

I don't want to write code that won't work one day, so I'm not going to. Instead, I will need to figure out how to handle the `panic!` and pass back an `Err` to Vue. For now, I opened up [this topic](https://users.rust-lang.org/t/how-should-i-handle-f64-division-by-zero-panic-elegantly/32485) on the Rust-lang forum and am hoping someone is nice enough to help me out.

Starting Commit: []()

Ending Commit: []()
