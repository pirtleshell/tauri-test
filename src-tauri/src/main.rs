#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod genes;
use gedcom_json::GedcomResponse;
use serde::Serialize;

#[derive(Serialize)]
struct Response<'a> {
  value: u64,
  data: GedcomResponse,
  message: &'a str,
}

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            DoSomething {
              count,
              payload,
              callback,
              error,
            } => tauri::execute_promise(
              _webview,
              move || {
                if count > 5 {
                  // TODO: allow data to live outside closure with mutex!
                  let data = GedcomResponse::new(genes::init());
                  let response = Response {
                    value: 5,
                    data,
                    message: "HEY! I'M FROM THE RUST BACKEND!!",
                  };
                  println!("yoooo {:?}", payload);
                  Ok(response)
                } else {
                  Err(cmd::CommandError::new("count should be > 5").into())
                }
              },
              callback,
              error,
            ),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
