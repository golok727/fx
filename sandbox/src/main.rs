use fx_std::fs as fx_std_file;

fn main() {
    println!("Radhey Shyam\n");

    let filepath = "./unused/radha.fx";
    let source = fx_std_file::read_to_string(filepath).unwrap();

    println!("{source}");
}