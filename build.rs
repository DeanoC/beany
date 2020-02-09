use cmake::Config;

fn main()
{
    let dst = Config::new("cuda_compute").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=cuda_compute");
}