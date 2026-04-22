fn main() {
    // Grab the project path before maverick_os sees the args
    if let Some(path) = std::env::args().nth(1) {
        std::env::set_var("RAMP_PROJECT", path);
    }
    
    #[cfg(not(target_arch="wasm32"))]
    {
        main::maverick_main()
    }
}