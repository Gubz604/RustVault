pub struct Entry {
    label: String,
    username: String,
    password: String,
}

impl Entry {
    pub(crate) fn new(label: String, username: String, password: String) -> Result<Self, String> {
        if label.trim().len() < 1 {
            return Err("Label must contain at least 1 non-whitespace character".to_string());
        }

        Ok(Self {
            label,
            username,
            password,
        })
    }

    pub(crate) fn get_label(&self) -> &str {
        &self.label
    }

    pub(crate) fn get_username(&self) -> &str {
        &self.username
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::Entry;

    #[test]
    fn valid_entry() {
        let label: String   = String::from("example");
        let user: String    = String::from("user");
        let pass: String    = String::from(" pass ");

        let entry_example  = Entry::new(label, user, pass).expect("a nonblank label should be accepted");
        assert_eq!(entry_example.label, "example");
        assert_eq!(entry_example.username, "user");
        assert_eq!(entry_example.password, " pass ");
    }

    #[test]
    fn reject_empty_label() {
        let label: String   = String::from("");
        let user: String    = String::from("user");
        let pass: String    = String::from(" pass ");

        let entry_example  = Entry::new(label, user, pass);
        assert!(entry_example.is_err());
    }

    #[test]
    fn reject_whitespace_only_label() {
        let label: String   = String::from("\t");
        let user: String    = String::from("user");
        let pass: String    = String::from(" pass ");

        let entry_example  = Entry::new(label, user, pass);
        assert!(entry_example.is_err());
    }
}