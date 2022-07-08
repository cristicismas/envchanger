// path decorator is included so importing from a sibling mod is possible.
#[path = "errors.rs"]
#[macro_use]
mod errors;

const MAX_COMMANDS_ALLOWED: usize = 3;

fn get_nth_arg(n: usize) -> Option<String> {
  std::env::args().nth(n)
}

#[derive(Debug)]
pub struct Args {
  pub command: String,
  pub folder_name: Option<String>,
}

impl Args {
  pub fn new() -> Self {
    if std::env::args().len() > MAX_COMMANDS_ALLOWED {
      equit!(
        "Too many arguments. Maximum number of arguments is {}.",
        MAX_COMMANDS_ALLOWED - 1,
      );
    }

    let command = match get_nth_arg(1) {
      Some(value) => value,
      None => {
        equit!("At least one argument is required. Use `envch help` to display usage info.");
      }
    };

    let folder_name = match get_nth_arg(2) {
      Some(value) => {
        if command.as_str() != "folder" {
          equit!("Cannot have more than one argument if the first argument is not `folder`.");
        } else {
          Some(value)
        }
      }
      None => None,
    };

    if command == "folder" && folder_name.is_none() {
      equit!("Cannot use command `folder` without a second argument (the folder's name).");
    }

    Args {
      command,
      folder_name,
    }
  }
}
