#[allow(unused_unsafe)]
#[allow(dead_code)]

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

fn safe_funtion() {

}

fn main() {}
