#[cfg(feature = "parser")]
use winpoke::evaluate::eval;

#[cfg(feature = "parser")]
fn main() {
    let script = std::env::args()
        .nth(1)
        .expect("Please provide a script to evaluate.");

    #[cfg(feature = "parser")]
    match eval(&script) {
        Ok(_) => println!("Script executed successfully."),
        Err(e) => eprintln!("Error executing script: {:?}", e),
    }
}

#[cfg(not(feature = "parser"))]
fn main() {
    panic!("请启用 parser 特性");
}
