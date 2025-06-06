pub fn code() {
    println!("Hello from dep_581");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_581");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_581: {t}");
}

pub fn foo() {
    dep_324::code();
    dep_324::code_inlined();
    dep_324::code_generic(1u32);
    dep_362::code();
    dep_362::code_inlined();
    dep_362::code_generic(1u32);
}
