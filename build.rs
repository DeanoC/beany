use cmake::Config;

fn main()
{
    {
        let dst = Config::new("compute").build();
        println!("cargo:rustc-link-search=native={}", dst.display());

        println!("cargo:rustc-link-lib=static=compute_cuda");
        println!("cargo:rustc-link-lib=static=compute_sycl");

    }
 }