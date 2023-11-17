# Match

`match` statements in Rust are akin to `switch` statements found in other programming languages. They enable pattern matching on variables, allowing for concise and comprehensive conditional branching.

Matching involves specifying the pattern to match against, which can either be a variable or a condition evaluation (e.g., `x > 10`). It commences with the keyword `match` and encloses different options within a set of `{}`. For each pattern, code branches to run are assigned using the `=>` syntax. 


``` rust
match x {
    pattern1 => {
        // block of code
    },
    pattern2 => {
        // block of code
    },
    _ => {
        // Default option
    }
}
```