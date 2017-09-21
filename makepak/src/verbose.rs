
use std::cell::Cell;

thread_local!(static IS_VERBOSE: Cell<bool> = Cell::new(false));

pub fn set_verbose(is_verbose: bool) {
    IS_VERBOSE.with(|a| {
        a.set(is_verbose);
    });
}

pub fn print_verbose<F: FnOnce() -> String>(f: F) {
    let is_verbose = IS_VERBOSE.with(|a| { a.get() });
    if !is_verbose { return; }
    
    let s = f();
    println!("{}", s);
}

