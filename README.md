# consteval
* Macro crimes to evaluate non-const stuff at compile time *

### Example
```rust
let v = const_eval!(
    let mut res = 0;
    for i in 0..200 {
        if i % 3 == 0 {
            res += 1;
        }
    }
    res.to_string().parse::<i32>().unwrap()
);
assert_eq!(v, 67);
```

### How it works
The token-tree passed to `const_eval!` is passed to a proc-macro
that compiles another proc-macro during runtime.
This newly compiled proc-macro includes your token-tree.
After your token-tree is evaluated in the proc-macro; the result is converted into a TokenStream.
This stream can then be passed back to the caller of `const_eval!` - allowing
you to use an already evaluated value!