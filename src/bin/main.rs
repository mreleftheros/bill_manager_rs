use std::io;
use bill_manager_rs::{menu::MenuOption, util::get_input, App};

fn main() -> io::Result<()> {
    println!(" === Bill Manager ===");
    println!("");
    println!("Enter your name:");

    let user_name= get_input()?;
    let mut app = App::new(user_name);

    loop {
        app.render_menu();
        let option_input = match get_input() {
            Ok(v) => v,
            Err(_) => continue
        };

        let opt = match MenuOption::from(option_input) {
            Some(o) => o,
            None => {
                println!("Invalid option");
                continue
            }
        };

        use MenuOption::*;
        match opt {
            Add => {
                println!("Enter the name owed to:");
                let name = get_input().expect("Should be able to parse name");
                
                println!("Enter ammount:");
                let amt = get_input().expect("Problem parsing amount input");
                let amt = amt.parse::<i32>().expect("Not a number");

                app.add_bill(&name, amt)
            },
            View => app.view_bills(),
            Edit => {
                println!("Enter the name owed to:");
                let name = get_input().expect("Should be able to parse name");
                
                println!("Enter ammount:");
                let amt = get_input().expect("Problem parsing amount input");
                let amt = amt.parse::<i32>().expect("Not a number");
                
                app.edit(&name, amt);
            },
            Remove => {
                let name = get_input().expect("Can't read input");
                app.remove(&name);
            },
            Total => app.total(),
            Exit => {
                println!("Goodbye");
                break
            }
            _ => panic!("Invalid option"),
        }
    }

    Ok(())
}
