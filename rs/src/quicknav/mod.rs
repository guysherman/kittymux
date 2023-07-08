use serde::{Deserialize, Serialize};

use crate::error::KittyMuxError;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickNavEntry {
    pub title: String,
    pub key: char,
}

impl QuickNavEntry {
    pub fn new(title: String, key: char) -> QuickNavEntry {
        QuickNavEntry { title, key }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuickNavDatabase {
    entries: Vec<QuickNavEntry>,
}

impl QuickNavDatabase {
    pub fn new() -> QuickNavDatabase {
        QuickNavDatabase {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: QuickNavEntry) {
        if let Some(existing_entry) = self.find_entry_by_title_mut(&entry.title) {
            existing_entry.key = entry.key;
        } else {
            self.entries.push(entry);
        }
        self.save();
    }

    pub fn load() -> QuickNavDatabase {
        let filename = get_quicknav_file_path();
        QuickNavDatabase::from_file(&filename).unwrap_or(QuickNavDatabase::new())
    }

    fn from_file(filename: &str) -> Result<QuickNavDatabase, KittyMuxError> {
        let json = std::fs::read_to_string(filename)?;
        Ok(QuickNavDatabase::from_json(&json)?)
    }

    fn from_json(json: &str) -> Result<QuickNavDatabase, serde_json::Error> {
        serde_json::from_str::<QuickNavDatabase>(json)
    }

    pub fn save(&self) {
        let filename = get_quicknav_file_path();
        self.to_file(&filename);
    }

    fn to_file(&self, filename: &str) {
        std::fs::write(filename, self.to_json()).unwrap();
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    // generate a function which finds an entry by its title field
    pub fn find_entry_by_title(&self, title: &str) -> Option<&QuickNavEntry> {
        self.entries.iter().find(|entry| entry.title == title)
    }

    fn find_entry_by_title_mut(&mut self, title: &str) -> Option<&mut QuickNavEntry> {
        self.entries.iter_mut().find(|entry| entry.title == title)
    }

    pub fn find_entries_by_key(&self, key: char) -> Vec<&QuickNavEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.key == key)
            .collect()
    }
}

// generate a function that returns a file path based on the presence of the following environment
// variables: KITTYMUX_STATE_DIR, XDG_STATE_HOME, HOME. The file path should be:
// $KITTYMUX_STATE_DIR/quicknav-rs.json
// $XDG_STATE_HOME/kittymux/quicknav-rs.json
// $HOME/.local/state/kittymux/quicknav-rs.json
// If none of those environment variables are set, use the current directory
// If the file doesn't exist, create it
pub fn get_quicknav_file_path() -> String {
    if let Ok(kittymux_state_dir) = std::env::var("KITTYMUX_STATE_DIR") {
        format!("{}/quicknav-rs.json", kittymux_state_dir)
    } else if let Ok(xdg_state_home) = std::env::var("XDG_STATE_HOME") {
        format!("{}/kittymux/quicknav-rs.json", xdg_state_home)
    } else if let Ok(home) = std::env::var("HOME") {
        format!("{}/.local/state/kittymux/quicknav-rs.json", home)
    } else {
        "quicknav-rs.json".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::quicknav::{QuickNavDatabase, QuickNavEntry};

    #[test]
    fn deserialises_from_json() {
        let json_string = r###"{"title":"Foo","key":"c"}"###;
        // generate code to deseralise jsonString and test its values
        let entry: QuickNavEntry = serde_json::from_str(json_string).unwrap();
        assert_eq!(entry.title, "Foo");
        assert_eq!(entry.key, 'c');
    }

    // generate a test to serialize a QuickNavEntry to json and verify the output
    #[test]
    fn serializes_to_json() {
        let entry = QuickNavEntry::new("Foo".to_string(), 'c');
        let json_string = serde_json::to_string(&entry).unwrap();
        assert_eq!(json_string, r###"{"title":"Foo","key":"c"}"###);
    }

    #[test]
    fn from_json() {
        let json_string = r###"{"entries":[{"title":"Foo","key":"c"}]}"###;
        let db = QuickNavDatabase::from_json(json_string)
            .expect("Could not deserialize a QuickNavDatabase");
        assert_eq!(db.entries.len(), 1);
    }

    #[test]
    fn to_json() {
        let mut db = QuickNavDatabase::new();
        let entry = QuickNavEntry::new("Foo".to_string(), 'c');
        db.add_entry(entry);
        let json_string = db.to_json();
        assert_eq!(
            json_string,
            r###"{"entries":[{"title":"Foo","key":"c"}]}"###
        );
    }

    #[test]
    fn save() {
        let mut db = QuickNavDatabase::new();
        let entry = QuickNavEntry::new("Foo".to_string(), 'c');
        db.add_entry(entry);
        db.to_file("test.json");
        let json_string = std::fs::read_to_string("test.json").unwrap();
        assert_eq!(
            json_string,
            r###"{"entries":[{"title":"Foo","key":"c"}]}"###
        );
        std::fs::remove_file("test.json").unwrap();
    }

    // generate a test for QuikNavDatabase::load
    #[test]
    fn loads_from_file() {
        let mut db = QuickNavDatabase::new();
        let entry = QuickNavEntry::new("Foo".to_string(), 'c');
        db.add_entry(entry);
        db.to_file("load_test.json");
        db.clear();
        db = QuickNavDatabase::from_file("load_test.json")
            .expect("Failed to load a QuickNavDatabase from file");
        assert_eq!(db.entries.len(), 1);
        std::fs::remove_file("load_test.json").unwrap();
    }

    #[test]
    fn loading_a_non_existent_file_returns_an_error() {
        let qdb = QuickNavDatabase::from_file("null_load_test.json");
        assert_eq!(qdb.is_err(), true);
    }

    #[test]
    fn clear() {
        let mut db = QuickNavDatabase::new();
        let entry = QuickNavEntry::new("Foo".to_string(), 'c');
        db.add_entry(entry);
        db.clear();
        assert_eq!(db.entries.len(), 0);
    }

    // generate a test for find_entry_by_title
    #[test]
    fn find_entry_by_title() {
        let mut db = QuickNavDatabase::new();
        let entry = QuickNavEntry::new("Foo".to_string(), 'c');
        let entry2 = QuickNavEntry::new("Bar".to_string(), 'd');
        db.add_entry(entry);
        db.add_entry(entry2);
        let result = db.find_entry_by_title("Foo");
        assert_eq!(result.unwrap().key, 'c');
    }

    // test for upsert
    #[test]
    fn when_title_exists_upsert_changes_key() {
        let mut db = QuickNavDatabase::new();
        let entry = QuickNavEntry::new("Foo".to_string(), 'c');
        let entry2 = QuickNavEntry::new("Foo".to_string(), 'd');
        db.add_entry(entry);
        db.add_entry(entry2);
        assert_eq!(db.entries.len(), 1);
        assert_eq!(db.entries[0].key, 'd');
    }

    // when title doesn't exist upsert adds entry
    #[test]
    fn when_title_does_not_exist_upsert_adds_entry() {
        let mut db = QuickNavDatabase::new();
        let entry = QuickNavEntry::new("Foo".to_string(), 'c');
        db.add_entry(entry);
        assert_eq!(db.entries.len(), 1);
        assert_eq!(db.entries[0].key, 'c');
    }
}
