pub mod input;

fn main() {
    println!("Hello, world!");
    let result = input::get_user_input();
    if let Some(v) = result {
        println!("you typed {}", v);
    }
}
