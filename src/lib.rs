use std::collections::HashMap;

pub mod util;
pub mod menu;

#[derive(Debug)]
pub struct App {
  user_name: String,
  bills: HashMap<String, i32>
}

impl App {
  pub fn new(user_name: String) -> Self {
    Self {
      user_name,
      bills: HashMap::with_capacity(4)
    }
  }

  pub fn render_menu(&self) {
    println!(" === {}'s Bill Manager ===", self.user_name);
    println!("");
    let menu_items: [&str; 6] = ["Add bill", "View bills", "Edit bill", "Remove bill", "View total", "exit"];
  
    for (i, item) in menu_items.iter().enumerate() {
      println!("{}. {}", i + 1, item);
    }
  }

  pub fn add_bill(&mut self, name: &str, amt: i32) {
    self.bills.entry(name.to_owned()).and_modify(|v| *v += amt).or_insert(amt);
    println!("Added {} to {}", amt, name);
  }

  pub fn view_bills(&self) {
    for (k, v) in &self.bills {
      println!("{} is owed to {}", v, k);
    }
  }

  pub fn edit(&mut self, name: &str, amt: i32) {
    match self.bills.get(name) {
      Some(v) => {
        self.bills.insert(name.to_owned(), amt);
        println!("User {} updated with total: {}", name, amt);
      },
      None => println!("User {} was not found", name)
    }
  }

  pub fn remove(&mut self, name: &str) {
    match self.bills.remove(name) {
      Some(_) => println!("User {} deleted", name),
      None => println!("User {} not found", name)
    }
  }

  pub fn total(&self) {
    let mut total = 0;
    
    for _ in &self.bills {
      total += 1;
    }

    println!("Total: {total}");
  }
}