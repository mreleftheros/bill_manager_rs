#[derive(Debug)]
pub enum MenuOption {
    Add,
    View,
    Edit,
    Remove,
    Total,
    Exit
}

impl MenuOption {
  pub fn from(option: String) -> Option<Self> {
    match option.as_str() {
      "1" => Some(Self::Add),
      "2" => Some(Self::View),
      "3" => Some(Self::Edit),
      "4" => Some(Self::Remove),
      "5" => Some(Self::Total),
      "6" => Some(Self::Exit),
      _ => None
    }
  }
}