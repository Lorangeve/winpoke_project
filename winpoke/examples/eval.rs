use winpoke::evaluate::eval;

fn main() {
    let script = std::env::args()
        .nth(1)
        .expect("Please provide a script to evaluate.");

    match eval(&script) {
        Ok(_) => println!("Script executed successfully."),
        Err(e) => eprintln!("Error executing script: {:?}", e),
    }
}
