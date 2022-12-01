pub fn create_chunks(input: &str) -> Vec<&str> {
    input.trim().split("\n\n").collect()
}
