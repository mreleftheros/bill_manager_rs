use std::io;

struct Bill {
    name: String,
    amount: f64
}

struct User {
    name: String,
    bills: Vec<Bill>
}

impl User {
    fn new(name: String) -> Self {
        Self {
            name,
            bills: Vec::with_capacity(4)
        }
    }
}

fn get_user_input() -> io::Result<String> {
    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf)?;

    Ok(buf)
}

fn get_user_name() -> String {
    println!(" === Bill Manager ===");
    println!("");
    println!("Enter your name:");
    let user_name: String = get_user_input().expect("Could not parse user name");
    user_name
}

enum MainMenuItem {
    Add,
    View,
    Edit,
    Remove,
    Total,
    Exit
}

fn show_mainmenu(user: &User) {
    println!(" === {}'s Bill Manager ===", user.name);
    println!("");
    let menu_items: Vec<&str> = vec!["Add bill", "View bills", "Edit bill", "Remove bill", "View total", "exit"];
}

fn main() {
    let user_name: String = get_user_name();

    let user: User = User::new(user_name);

    loop {
        show_mainmenu(&user);
    }
}