pub fn code() {
    println!("Hello from dep_50");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_50");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_50: {t}");
}

pub fn foo() {
    dep_4::code();
    dep_4::code_inlined();
    dep_4::code_generic(1u32);
    dep_7::code();
    dep_7::code_inlined();
    dep_7::code_generic(1u32);
    dep_9::code();
    dep_9::code_inlined();
    dep_9::code_generic(1u32);
    dep_8::code();
    dep_8::code_inlined();
    dep_8::code_generic(1u32);
}
