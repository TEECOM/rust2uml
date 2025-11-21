fn main() {
    // Only add rpath if the user enabled our cfg flag
    if std::env::var("CARGO_CFG_BUILD_WITH_RPATH").is_ok() {
        let sysroot = std::process::Command::new("rustc")
            .args(["--print", "sysroot"])
            .output()
            .expect("failed to get rustc sysroot")
            .stdout;
        let _sysroot = String::from_utf8(sysroot).unwrap().trim().to_owned();

        // HACK: The linker is currently throwing errors at execution time
        // when these args are present:
        //
        // note: ld: unknown options: -rpath=~/.rustup/nightly-2025-03-15-aarch64-apple-darwin/lib
        //   clang: error: linker command failed with exit code 1 (use -v to see invocation)
        // println!("cargo:rustc-link-arg=-Wl,-rpath={}/lib", sysroot);
    }
}
