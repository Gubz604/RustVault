use super::entry::Entry;

pub struct Vault {
    entries: Vec<Entry>,
}

impl Vault {
    pub(crate) fn new() -> Self {
        Self {
            entries: Vec::new()
        }
    }

    pub(crate) fn add(&mut self, entry: Entry) -> Result<(), String> {
        if self.get(entry.get_label()).is_some() {
            return Err("An entry with that label already exists".to_string());
        }

        self.entries.push(entry);
        Ok(())
    }

    pub(crate) fn get(&self, label: &str) -> Option<&Entry> {
        self.entries.iter().find(|existing| existing.get_label() == label)
    }
}

#[cfg(test)]
mod tests {

    use super::Vault;
    use super::Entry;

    #[test]
    fn new_vault_starts_empty() {
        let vault = Vault::new();
        assert!(vault.entries.is_empty());
    }

    #[test]
    fn add_entries_with_different_labels_succeeds() {
        let mut vault = Vault::new();

        let entry_1 = Entry::new("label1".to_string(), "user1".to_string(), "pass1".to_string()).expect("a nonblank label should be accepted");
        let entry_2 = Entry::new("label2".to_string(), "user2".to_string(), "pass2".to_string()).expect("a nonblank label should be accepted");

        vault.add(entry_1).expect("Each entry label must be unique");
        assert!(vault.add(entry_2).is_ok());
        assert_eq!(vault.entries.len(), 2);
        assert_eq!(vault.entries[0].get_label(), "label1");
        assert_eq!(vault.entries[1].get_label(), "label2");
    }

    #[test]
    fn add_entries_with_same_label_rejected() {
        let mut vault = Vault::new();

        let entry_1 = Entry::new("label1".to_string(), "user1".to_string(), "pass1".to_string()).expect("a nonblank label should be accepted");
        let entry_2 = Entry::new("label1".to_string(), "user2".to_string(), "pass2".to_string()).expect("a nonblank label should be accepted");

        vault.add(entry_1).expect("Each entry label must be unique");
        assert!(vault.add(entry_2).is_err());

        assert_eq!(vault.entries.len(), 1);
        assert_eq!(vault.entries[0].get_username(), "user1");
    }

    #[test]
    fn can_retrieve_second_entry_and_not_return_first() {
        let mut vault = Vault::new();

        let entry_1 = Entry::new("label1".to_string(), "user1".to_string(), "pass1".to_string()).expect("a nonblank label should be accepted");
        let entry_2 = Entry::new("label2".to_string(), "user2".to_string(), "pass2".to_string()).expect("a nonblank label should be accepted");

        vault.add(entry_1).expect("Each entry label must be unique");
        assert!(vault.add(entry_2).is_ok());
        assert_eq!(vault.entries.len(), 2);

        let found = vault.get("label2").expect("label2 should be in the vault");
        assert_eq!(found.get_username(), "user2");
    }

    #[test]
    fn labels_are_case_sensitive() {
        let mut vault = Vault::new();

        let entry_1 = Entry::new("github".to_string(), "user1".to_string(), "pass1".to_string()).expect("a nonblank label should be accepted");

        vault.add(entry_1).expect("Each entry label must be unique");
        assert_eq!(vault.entries.len(), 1);
        assert!(vault.get("GitHub").is_none());
    }
}