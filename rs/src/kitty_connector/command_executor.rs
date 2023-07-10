use std::process::Command;
use std::str;

use mockall::automock;

#[automock]
pub trait CommandExecutor {
    fn execute_command<'a>(&self, cmd: &'a str, args: &'a [&'a str]) -> String;
}

pub struct KittyCommandExecutor {}

impl CommandExecutor for KittyCommandExecutor {
    fn execute_command<'a>(&self, cmd: &'a str, args: &[&'a str]) -> String {
        let mut command = Command::new("kitty");

        command
            .arg("@");

        if let Ok(kitty_remote_to) = std::env::var("KITTYMUX_REMOTE_TO") {
            command
                .arg("--to")
                .arg(kitty_remote_to);
        }

        command
            .arg(cmd)
            .args(args);

        let output = command.output()
            .expect("Command failed");

        str::from_utf8(&output.stdout).unwrap().to_string()
    }
}
