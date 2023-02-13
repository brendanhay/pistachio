fn main() {
    lalrpop::Configuration::new()
        .always_use_colors()
        .process_current_dir()
        .expect("failed to setup lalrpop")
}
