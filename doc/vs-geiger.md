# utrace vs. cargo-geiger

This document explains the key differences
between `utrace` and [cargo-geiger](https://github.com/geiger-rs/cargo-geiger),
two tools designed for analyzing unsafe code in Rust projects.

## Overview

| Feature                    | utrace                                                                      | cargo-geiger                                                         |
|----------------------------|-----------------------------------------------------------------------------|----------------------------------------------------------------------|
| **Focus**                  | Detailed unsafe code analysis, including call traces                        | High-level overview of unsafe code                                   |
| **Analysis Basis**         | Compiler-based, using HIR (High-level Intermediate Representation)          | Source-based, using `syn` to parse AST (Abstract Syntax Tree)        |
| **Macro Expansion**        | Analyzes macro-expanded code, ensuring all unsafe code is detected          | Analyzes AST before macro expansion, may miss unsafe in macros       |
| **Classification Criteria**| Categorizes unsafe code by Rust’s Unsafe Keywords                           | Classifies code into five categories, including unsafe expressions   |

*(Ref: [Rust's Unsafe Keywords](https://doc.rust-lang.org/reference/unsafe-keyword.html))*

## Example Analysis: cargo-geiger vs. utrace

To provide a concrete comparison between `cargo-geiger` and `utrace`,
we analyzed [a Rust example](https://github.com/islet-project/utrace/blob/devel/examples/unsafe-keyword.rs) using both tools.

### Example Code

```rust
unsafe fn unsafe_fn() {}

fn fn_has_unsafe_block() {
    unsafe {
        println!("dummy");
    }
}

struct Foo;

impl Foo {
    unsafe fn unsafe_method(&self) {}

    fn method_has_unsafe_block(&self) {
        unsafe {
            println!("dummy");
        }
    }
}

unsafe trait Bar {
    unsafe fn unsafe_trait_fn1();
    unsafe fn unsafe_trait_fn2() {}

    fn trait_fn_has_unsafe_block() {
        unsafe {
            println!("dummy");
        }
    }
}

unsafe impl Bar for Foo {
    unsafe fn unsafe_trait_fn1() {}
    unsafe fn unsafe_trait_fn2() {}

    fn trait_fn_has_unsafe_block() {
        unsafe {
            println!("dummy");
        }
    }
}

macro_rules! create_unsafe_fn {
    ($fn1:ident, $fn2:ident) => {
        unsafe fn $fn1() {}
        fn $fn2() {
            unsafe {
                println!("dummy");
            }
        }
    };
}

create_unsafe_fn!(unsafe_macro_fn, macro_fn_unsafe_block);

struct Closures(Vec<Box<dyn Fn()>>);

fn hold_unsafe_closure() {
    let mut closures = Closures(Vec::new());
    closures.0.push(ret_unsafe_closure());
    closures.0.clear();
    Foo.method_has_unsafe_block();
}

fn ret_unsafe_closure() -> Box<dyn Fn()> {
    Box::new(|| {
        unsafe {
            println!("dummy in unsafe closure");
        }
    })
}
```

### Analysis Results

- **cargo-geiger**: Out of **15** unsafe instances, **11** were detected.
- **utrace**: Out of **15** unsafe instances, **15** were detected.

#### utrace

```
## Summary
Crate                Functions  Blocks     Impls      Traits
unsafe_keyword       7          6          1          1

## Unsafe Item List (unsafe_keyword)
- type: Block, id: unsafe_keyword::<Foo as Bar>::trait_fn_has_unsafe_block
- type: Block, id: unsafe_keyword::Bar::trait_fn_has_unsafe_block
- type: Block, id: unsafe_keyword::Foo::method_has_unsafe_block
- type: Block, id: unsafe_keyword::fn_has_unsafe_block
- type: Block, id: unsafe_keyword::macro_fn_unsafe_block
- type: Block, id: unsafe_keyword::ret_unsafe_closure
- type: Function, id: unsafe_keyword::<Foo as Bar>::unsafe_trait_fn1
- type: Function, id: unsafe_keyword::<Foo as Bar>::unsafe_trait_fn2
- type: Function, id: unsafe_keyword::Bar::unsafe_trait_fn1
- type: Function, id: unsafe_keyword::Bar::unsafe_trait_fn2
- type: Function, id: unsafe_keyword::Foo::unsafe_method
- type: Function, id: unsafe_keyword::unsafe_fn
- type: Function, id: unsafe_keyword::unsafe_macro_fn
- type: Impl, id: unsafe_keyword::<Foo as Bar>
- type: Trait, id: unsafe_keyword::Bar

## Unsafe Call Trace
- unsafe_keyword::<Foo as Bar>::trait_fn_has_unsafe_block (unsafe)
    - std::io::stdio::_print
- unsafe_keyword::<Foo as Bar>::unsafe_trait_fn1 (unsafe)
- unsafe_keyword::<Foo as Bar>::unsafe_trait_fn2 (unsafe)
- unsafe_keyword::Bar::trait_fn_has_unsafe_block (unsafe)
    - std::io::stdio::_print
- unsafe_keyword::Bar::unsafe_trait_fn2 (unsafe)
- unsafe_keyword::Foo::method_has_unsafe_block (unsafe)
    - std::io::stdio::_print
- unsafe_keyword::Foo::unsafe_method (unsafe)
- unsafe_keyword::fn_has_unsafe_block (unsafe)
    - std::io::stdio::_print
- unsafe_keyword::hold_unsafe_closure
    - unsafe_keyword::Closures::{constructor#0}
    - alloc::std::vec::Vec::<T, A>::push
    - unsafe_keyword::ret_unsafe_closure (unsafe)
        - std::io::stdio::_print
    - alloc::std::vec::Vec::<T, A>::clear
    - unsafe_keyword::Foo::method_has_unsafe_block (unsafe)
        - std::io::stdio::_print
- unsafe_keyword::macro_fn_unsafe_block (unsafe)
    - std::io::stdio::_print
- unsafe_keyword::main
- unsafe_keyword::ret_unsafe_closure (unsafe)
    - std::io::stdio::_print
- unsafe_keyword::safe_funtion
- unsafe_keyword::unsafe_fn (unsafe)
- unsafe_keyword::unsafe_macro_fn (unsafe)
```

#### cargo-geiger

```
Functions  Expressions  Impls  Traits  Methods  Dependency

1/1        5/5          1/1    1/1     3/3      ☢️ unsafe-ex 0.1.0

1/1        5/5          1/1    1/1     3/3
```

This comparison highlights the differences in detection capabilities between `cargo-geiger`.
