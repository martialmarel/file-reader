use file_reader::process;

fn main() {
    match process() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
