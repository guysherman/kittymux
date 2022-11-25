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
        let output = Command::new("kitty")
            .arg("@")
            .arg(cmd)
            .args(args)
            .output()
            .expect("Command failed");

        str::from_utf8(&output.stdout).unwrap().to_string()
    }
}
