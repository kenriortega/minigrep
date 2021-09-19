use std::env;

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args);
    minigrep::run(cfg);
}
