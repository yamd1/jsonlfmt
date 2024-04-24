fn main() {
    if let Err(e) = jsonlfmt::get_args().and_then(jsonlfmt::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
