#[path = "./context/db_context.rs"]
mod context;


fn main() {
    println!("{}",context::test_print());
}