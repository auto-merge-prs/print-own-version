pub fn print_own_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}
