# consteval
macro crimes to evaluate non-const stuff at compile time

Disclaimer: This hijacks the compilation process, so use it with caution and probably not for anything serious.

This only works on windows for now due to a hard coded dll-path, but you should be able to switch the .dll to something else for other platforms.


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
that compiles another proc-macro that includes your code.

After your token-tree is evaluated in the proc-macro, the result is converted into a TokenStream.
This stream can then be passed back to the caller of `const_eval!` - allowing
you to use an already evaluated value!

Because everything is compiled as proc-macros, everything is guaranteed to be evaluated at compile time.
This however means that you cannot use dependencies and custom types defined outside of the macro call.
I have some ideas around this I would like to explore, but until then you are limited to the code inside the macro.
