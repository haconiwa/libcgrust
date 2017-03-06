#![allow(non_snake_case, non_camel_case_types)]
#![crate_type = "dylib"]

#[no_mangle]
pub extern fn cgrust_sample(n: i32) {
    for i in 0..n {
        println!("Countup: {}", i);
    }
}
