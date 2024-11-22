pub enum Instruction {
  Forward,
  Down,
  Up,
}

pub struct Command {
  pub instruction: Instruction,
  pub value: i32,
}
