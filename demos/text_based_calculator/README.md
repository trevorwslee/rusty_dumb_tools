# DumbCalculator Text-based Demo

Simple text-based Calculator demo for `DumbCalculator` of `rusty_dumb_tools` **_Rust_** crate.


Assume you are building it from source code, you can run the demo like:

```
cargo run
```
or
```
cargo run -- rich
```


After running `cargo run`, the simple text-based calculator UI will be like

```
* arrow keys to move selected key
* space key to commit selected key
* can press corresponding keys directly
* note that 'c' is the same as 'C' and the enter key is the same as '='

        ===============
        |           0 |                                                                                                                                                          
        | 7 8 9 |  C  |                                                                                                                                                          
        | 4 5 6 | * / |                                                                                                                                                          
        | 1 2 3 | + - |                                                                                                                                                          
        | % 0 . |  =  |                                                                                                                                                          
        ===============
```

Running `cargo run -- rich` will give you a richer text-based calculator UI, with some richer calculator features
