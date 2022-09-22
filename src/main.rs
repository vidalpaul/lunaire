fn main() {
    let config = lunaire::get_args().unwrap();
    lunaire::run(config).unwrap();
}
