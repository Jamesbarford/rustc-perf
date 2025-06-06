pub fn code() {
    println!("Hello from dep_312");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_312");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_312: {t}");
}

pub fn foo() {
    dep_94::code();
    dep_94::code_inlined();
    dep_94::code_generic(1u32);
}
