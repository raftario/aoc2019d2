use std::{env, fs, path::Path};

fn parse_input_file() -> Vec<usize> {
    let contents = fs::read_to_string("input.txt").expect("Can't read `input.txt`");
    contents.split(",").filter_map(|n| n.parse().ok()).collect()
}

fn main() {
    let code = parse_input_file();
    let contents = format!("&{:#?}", code);
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("codegen.rs");
    fs::write(dest_path, contents).unwrap();
}
