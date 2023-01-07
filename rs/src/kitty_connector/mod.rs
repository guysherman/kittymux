pub mod command_executor;

use self::command_executor::CommandExecutor;

pub struct KittyConnector<'a> {
    pub executor: &'a dyn CommandExecutor,
}

impl KittyConnector<'_> {
    pub fn close_window(&self, id: i32) -> () {
        self.executor
            .execute_command("close-window", &["-m", format!("id:{}", id).as_str()]);
    }

    pub fn close_tab(&self, id: i32) -> () {
        self.executor
            .execute_command("close-tab", &["-m", format!("id:{}", id).as_str()]);
    }

    pub fn set_window_title(&self, id: i32, new_title: &str) -> () {
        self.executor.execute_command(
            "set-window-title",
            &["-m", format!("id:{}", id).as_str(), new_title],
        );
    }

    pub fn set_tab_title(&self, id: i32, new_title: &str) -> () {
        self.executor.execute_command(
            "set-tab-title",
            &["-m", format!("id:{}", id).as_str(), new_title],
        );
    }

    pub fn focus_window(&self, id: u32) -> () {
        self.executor
            .execute_command("focus-window", &["-m", format!("id:{}", id).as_str()]);
    }

    pub fn focus_tab(&self, id: u32) -> () {
        self.executor
            .execute_command("focus-tab", &["-m", format!("id:{}", id).as_str()]);
    }

    pub fn ls(&self) -> String {
        self.executor.execute_command("ls", &[])
    }
}

#[cfg(test)]
mod tests {
    use super::{command_executor::MockCommandExecutor, KittyConnector};

    #[test]
    fn given_id_when_close_window_called_then_execute_command_called() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_execute_command()
            .withf(|cmd: &str, args: &[&str]| {
                cmd == "close-window" && args[0] == "-m" && args[1] == "id:5"
            })
            .times(1)
            .returning(|_cmd: &str, _args: &[&str]| "".to_string());

        let conn = KittyConnector { executor: &mock };
        conn.close_window(5);
    }

    #[test]
    fn given_id_when_close_tab_called_then_execute_command_called() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_execute_command()
            .withf(|cmd: &str, args: &[&str]| {
                cmd == "close-tab" && args[0] == "-m" && args[1] == "id:5"
            })
            .times(1)
            .returning(|_cmd: &str, _args: &[&str]| "".to_string());

        let conn = KittyConnector { executor: &mock };
        conn.close_tab(5);
    }

    #[test]
    fn given_id_when_set_window_title_called_then_execute_command_called() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_execute_command()
            .withf(|cmd: &str, args: &[&str]| {
                cmd == "set-window-title"
                    && args[0] == "-m"
                    && args[1] == "id:5"
                    && args[2] == "new title"
            })
            .times(1)
            .returning(|_cmd: &str, _args: &[&str]| "".to_string());

        let conn = KittyConnector { executor: &mock };
        conn.set_window_title(5, "new title");
    }

    #[test]
    fn given_id_when_set_tab_title_called_then_execute_command_called() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_execute_command()
            .withf(|cmd: &str, args: &[&str]| {
                cmd == "set-tab-title"
                    && args[0] == "-m"
                    && args[1] == "id:5"
                    && args[2] == "new title"
            })
            .times(1)
            .returning(|_cmd: &str, _args: &[&str]| "".to_string());

        let conn = KittyConnector { executor: &mock };
        conn.set_tab_title(5, "new title");
    }

    #[test]
    fn given_id_when_focus_window_called_then_execute_command_called() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_execute_command()
            .withf(|cmd: &str, args: &[&str]| {
                cmd == "focus-window" && args[0] == "-m" && args[1] == "id:5"
            })
            .times(1)
            .returning(|_cmd: &str, _args: &[&str]| "".to_string());

        let conn = KittyConnector { executor: &mock };
        conn.focus_window(5);
    }

    #[test]
    fn given_id_when_focus_tab_called_then_execute_command_called() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_execute_command()
            .withf(|cmd: &str, args: &[&str]| {
                cmd == "focus-tab" && args[0] == "-m" && args[1] == "id:5"
            })
            .times(1)
            .returning(|_cmd: &str, _args: &[&str]| "".to_string());

        let conn = KittyConnector { executor: &mock };
        conn.focus_tab(5);
    }

    #[test]
    fn when_ls_called_then_execute_command_called() {
        let ls_payload = "[]";
        let mut mock = MockCommandExecutor::new();
        mock.expect_execute_command()
            .withf(|cmd: &str, _args: &[&str]| cmd == "ls")
            .times(1)
            .returning(|_cmd: &str, _args: &[&str]| ls_payload.to_string());

        let conn = KittyConnector { executor: &mock };
        conn.ls();
    }
}
