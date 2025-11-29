fn main() {
    
    let path = std::env::current_dir().unwrap();
    println!("Current dir: {}", path.display());
    
    minigit::cli_main();
}
