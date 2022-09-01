fn main() {
    txt_writer::WriteData {}
        .drop_replace(
            "what you want to write to txt".to_string(),
            "src/data.txt".to_string(),
        )
        .expect("failed when writing");
    let data = txt_writer::ReadData {}
        .read("src/data.txt".to_string())
        .expect("failed when reading");
    for x in data {
        println!("{}", x);
    }
    let data = txt_writer::ReadData {}
        .read_one("src/data.txt".to_string())
        .expect("failed when reading");
    println!("{}", data);
}
