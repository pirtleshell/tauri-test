use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DoSomethingPayload {
  state: String,
  data: u64,
}

#[derive(Debug, Clone)]
pub struct CommandError<'a> {
  message: &'a str,
}

impl<'a> CommandError<'a> {
  pub fn new(message: &'a str) -> Self {
    Self { message }
  }
}

impl<'a> std::fmt::Display for CommandError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl<'a> std::error::Error for CommandError<'a> {}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  // your custom commands
  // multiple arguments are allowed
  // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
  DoSomething {
    count: u64,
    payload: DoSomethingPayload,
    callback: String,
    error: String,
  },
}
