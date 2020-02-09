use cmake::Config;
use cuda_config::*;

fn main()
{
    println!("cargo:rerun-if-changed=build.rs");

    if cfg!(target_os = "windows") {
        println!(
            "cargo:rustc-link-search=native={}",
            find_cuda_windows().display()
        );
    } else {
        for path in find_cuda() {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
    };

    println!("cargo:rustc-link-lib=dylib=cuda");
    println!("cargo:rustc-link-lib=dylib=cudart");
    println!("cargo:rerun-if-env-changed=CUDA_LIBRARY_PATH");

    {
        let dst = Config::new("compute").build();
        println!("cargo:rustc-link-search=native={}", dst.display());
        println!("cargo:rustc-link-search=native=compute/out_libs/Debug");
        println!("cargo:rustc-link-lib=static=compute_cuda");
        println!("cargo:rustc-link-lib=static=al2o3_platform");
        println!("cargo:rustc-link-lib=static=al2o3_memory");

    }
 }