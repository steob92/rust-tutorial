# loops

Loops in Rust are straightforward and flexible. The `loop` keyword initiates an infinite loop, allowing code to execute repeatedly within a defined scope until explicitly interrupted by a `break` statement.

```rust
loop{
    // block of code

    if condition{
        break;
    }
}
```


Loops can also be labeled, this allows for breaking out of specific levels of the loop:
```rust
'parent : loop{
    'nested1: loop {
        'nested2: loop{

            if condition1 {
                break 'nested1;
            }
            if condition2 {
                break 'parent;
            }
        }
    }
}
```