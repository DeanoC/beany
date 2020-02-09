#[link(name="compute_cuda", kind="static")]
extern {
    fn testcall(v: f32);
}

/*#[link(name="compute_sycl", kind="static")]
extern {
    fn testcall2(v: f32);
}*/

fn main() {
    println!("Hello, world!");

    // calling the function from foo library
    unsafe {
        testcall(3.14159);
    };
}
