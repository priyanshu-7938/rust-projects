use figlet_rs::FIGfont;

fn main() {
    let font = FIGfont::standard().unwrap();
    let figure = font.convert("Hello Rust").unwrap();
    println!("{}", figure);
}
