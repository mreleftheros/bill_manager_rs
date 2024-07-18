use std::io;

pub fn get_input() -> io::Result<String> {
  let mut buf: String = String::new();
  
  io::stdin().read_line(&mut buf)?;

  Ok(buf.trim().to_owned())
}

