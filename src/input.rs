use std::io;

pub fn get_user_input() -> Option<String> {
  let mut input = String::new();

  while io::stdin().read_line(&mut input).is_err() {
    println!("Please enter your input again");
  }

  input = input.trim().to_owned();
  
  match input.as_str() {
    "" => None,
    _ => Some(input)
  }
}