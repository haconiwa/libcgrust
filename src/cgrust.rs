#![crate_type = "dylib"]

#[no_mangle]
pub extern fn cgrust_sample(r:i32) {
    for i in 0..r {
        println!("Countup: {}", i);
    }
}
