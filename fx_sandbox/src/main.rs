use fx_lexer::tokenize;
use fx_std::fs as fx_std_file;

fn main() {
    println!("Radhey Shyam\n");

    let filepath = "./unused/radha.fx";
    let source = fx_std_file::read_to_string(filepath).unwrap();
    let tokens = tokenize(&source);

    for token in tokens {
        println!("{token}");
    }
}
