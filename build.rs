use cmake::Config;

fn main() {
    let tdjson_libs = [
        "tdjson_static",
        "tdjson_private",
        "tdclient",
        "tdcore",
        "tdapi",
        "tdactor",
        "tddb",
        "tdsqlite",
        "tdnet",
        "tdutils",
    ];

    let dst = Config::new("td")
        .profile("Release")
        .very_verbose(true)
        .build();
    println!("cargo:rustc-link-search={}/lib", dst.display());

    for lib in tdjson_libs.iter() {
        println!("cargo:rustc-link-lib=static={}", lib);
    }
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=ssl");
    println!("cargo:rustc-link-lib=dylib=crypto");
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=z");
    println!("cargo:rustc-link-lib=dylib=m");
}
