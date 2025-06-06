pub fn code() {
    println!("Hello from dep_32");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_32");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_32: {t}");
}

pub fn foo() {
    dep_0::code();
    dep_0::code_inlined();
    dep_0::code_generic(1u32);
}
