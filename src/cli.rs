#[derive(Debug, PartialEq)]
pub(crate) enum Command {
    Init,
    List,
    Add(String),
    Get(String),
}

pub(crate) fn parse_command(args: &[String]) -> Result<Command, String> {
    match args {
        [name] => match name.as_str() {
            "init" => Ok(Command::Init),
            "list" => Ok(Command::List),
            _ => Err("Expected init, list, add <label>, or get <label>".to_string()),
        },
        [name, label] if matches!(name.as_str(), "add" | "get") && label.trim().is_empty() => {
            Err("Label must contain a non-whitespace character".to_string())
        }
        [name, label] => match name.as_str() {
            "get" => Ok(Command::Get(label.clone())),
            "add" => Ok(Command::Add(label.clone())),
            _ => Err("Expected init, list, add <label>, or get <label>".to_string()),
        }
        _ => Err("Invalid arguments".to_string()),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_command_successful_get() {
        let args = ["get".to_string(), " github ".to_string()];
        let command = parse_command(&args).expect("get with a nonblank label should parse");

        assert_eq!(command, Command::Get(" github ".to_string()));
    }

    #[test]
    fn parse_command_rejects_get_invalid_label() {
        let args = ["get".to_string(), " ".to_string()];
        assert!(parse_command(&args).is_err());
    }

    #[test]
    fn parse_command_successful_add() {
        let args = ["add".to_string(), " github ".to_string()];
        let command = parse_command(&args).expect("add with a nonblank label should parse");

        assert_eq!(command, Command::Add(" github ".to_string()));
    }

    #[test]
    fn parse_command_rejects_add_invalid_label() {
        let args = ["add".to_string(), " ".to_string()];
        assert!(parse_command(&args).is_err());
    }

    #[test]
    fn parse_command_successful_init() {
        let args = ["init".to_string()];
        let command = parse_command(&args).expect("init should parse");

        assert_eq!(command, Command::Init);
    }

    #[test]
    fn parse_command_successful_list() {
        let args = ["list".to_string()];
        let command = parse_command(&args).expect("list should parse");

        assert_eq!(command, Command::List);
    }

    #[test]
    fn parse_command_rejects_missing_command() {
        assert!(parse_command(&[]).is_err());
    }

    #[test]
    fn parse_command_rejects_unknown_command() {
        let args = ["open".to_string()];
        assert!(parse_command(&args).is_err());
    }

    #[test]
    fn parse_command_requires_label_get() {
        let args = ["get".to_string()];
        assert!(parse_command(&args).is_err());
    }

    #[test]
    fn parse_command_requires_label_add() {
        let args = ["add".to_string()];
        assert!(parse_command(&args).is_err());
    }

    #[test]
    fn parse_command_rejects_label_list() {
        let args = ["list".to_string(), "extra".to_string()];
        assert!(parse_command(&args).is_err());
    }

    #[test]
    fn parse_command_rejects_label_init() {
        let args = ["init".to_string(), "extra".to_string()];
        assert!(parse_command(&args).is_err());
    }

    #[test]
    fn parse_command_rejects_third_argument_get() {
        let args = ["get".to_string(), "extra".to_string(), "more_extra".to_string()];
        assert!(parse_command(&args).is_err());
    }

    #[test]
    fn parse_command_rejects_third_argument_add() {
        let args = ["add".to_string(), "extra".to_string(), "more_extra".to_string()];
        assert!(parse_command(&args).is_err());
    }
}