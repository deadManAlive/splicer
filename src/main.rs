mod config;

fn main() {
    let cfg = config::Config::read();
    println!("{:#?}", cfg);
}
