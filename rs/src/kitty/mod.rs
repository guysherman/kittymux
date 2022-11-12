mod command_executor;

use mockall::automock;

use self::command_executor::CommandExecutor;

#[automock]
pub trait KittyRemote {
    fn close_window(&self, id: i32) -> ();
    fn close_tab(&self, id: i32) -> ();
    fn set_window_title(&self, id: i32, new_title: &str) -> ();
    fn set_tab_title(&self, id: i32, new_title: &str) -> ();
    fn focus_window(&self, id: i32) -> ();
    fn focus_tab(&self, id: i32) -> ();
}

pub struct KittyConnector<'a> {
    executor: &'a dyn CommandExecutor,
}

impl KittyRemote for KittyConnector<'_> {
    fn close_window(&self, id: i32) -> () {
        self.executor
            .execute_command("close-window", &["-m", format!("id:{}", id).as_str()]);
    }

    fn close_tab(&self, id: i32) -> () {
        self.executor
            .execute_command("close-tab", &["-m", format!("id:{}", id).as_str()]);
    }

    fn set_window_title(&self, id: i32, new_title: &str) -> () {
        self.executor.execute_command(
            "set-window-title",
            &["-m", format!("id:{}", id).as_str(), new_title],
        );
    }

    fn set_tab_title(&self, id: i32, new_title: &str) -> () {
        self.executor.execute_command(
            "set-tab-title",
            &["-m", format!("id:{}", id).as_str(), new_title],
        );
    }

    fn focus_window(&self, id: i32) -> () {
        self.executor
            .execute_command("focus-window", &["-m", format!("id:{}", id).as_str()]);
    }

    fn focus_tab(&self, id: i32) -> () {
        self.executor
            .execute_command("focus-tab", &["-m", format!("id:{}", id).as_str()]);
    }
}

#[cfg(test)]
mod tests {
    use super::{command_executor::MockCommandExecutor, KittyConnector, KittyRemote};

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
}
