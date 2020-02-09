#[link(name="cuda_compute", kind="static")]
extern {
    // this is rustified prototype of the function from our C library
    fn testcall(v: f32);
}

fn main() {
    println!("Hello, world!");

    // calling the function from foo library
    unsafe {
        testcall(3.14159);
    };
}
