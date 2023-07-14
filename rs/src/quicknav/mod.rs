use serde::{Deserialize, Serialize};

pub mod persistence;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuickNavEntry {
    pub title: String,
    pub id: u32,
    pub key: char,
}

impl QuickNavEntry {
    pub fn new(title: String, key: char, id: u32) -> QuickNavEntry {
        QuickNavEntry { title, key, id }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuickNavDatabase {
    entries: Vec<QuickNavEntry>,
}

impl QuickNavDatabase {
    pub fn new() -> QuickNavDatabase {
        QuickNavDatabase {
            entries: Vec::new(),
        }
    }

    #[cfg(test)]
    pub fn from_entries(entries: Vec<QuickNavEntry>) -> QuickNavDatabase {
        QuickNavDatabase { entries }
    }

    pub fn add_entry(&mut self, entry: QuickNavEntry) {
        if let Some(existing_entry) = self.find_entry_by_id_mut(entry.id) {
            existing_entry.key = entry.key;
        } else {
            self.entries.push(entry);
        }
    }

    pub fn find_entries_by_key(&self, key: char) -> Vec<&QuickNavEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.key == key)
            .collect()
    }

    pub fn find_entry_by_id(&self, id: u32) -> Option<&QuickNavEntry> {
        self.entries.iter().find(|entry| entry.id == id)
    }

    fn find_entry_by_id_mut(&mut self, id: u32) -> Option<&mut QuickNavEntry> {
        self.entries.iter_mut().find(|entry| entry.id == id)
    }

    pub fn clean_up(&mut self, entries: Vec<(String, u32)>) {
        let mut entries_to_remove = Vec::new();
        for entry in &self.entries {
            if !entries
                .iter()
                .any(|(title, id)| entry.title == *title && entry.id == *id)
            {
                entries_to_remove.push(entry.id);
            }
        }
        self.entries
            .retain(|entry| !entries_to_remove.contains(&entry.id));
    }

    pub fn rename_entry(&mut self, id: u32, new_title: String) {
        if let Some(entry) = self.find_entry_by_id_mut(id) {
            entry.title = new_title;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::quicknav::{QuickNavDatabase, QuickNavEntry};

    #[test]
    fn deserialises_from_json() {
        let json_string = r###"{"title":"Foo","id":1,"key":"c"}"###;
        // generate code to deseralise jsonString and test its values
        let entry: QuickNavEntry = serde_json::from_str(json_string).unwrap();
        assert_eq!(entry.key, 'c');
        assert_eq!(entry.id, 1);
    }

    // generate a test to serialize a QuickNavEntry to json and verify the output
    #[test]
    fn serializes_to_json() {
        let entry = QuickNavEntry::new("Foo".to_string(), 'c', 1);
        let json_string = serde_json::to_string(&entry).unwrap();
        assert_eq!(json_string, r###"{"title":"Foo","id":1,"key":"c"}"###);
    }

    #[test]
    fn find_entry_by_id() {
        let mut db = QuickNavDatabase::new();
        let entry = QuickNavEntry::new("Foo".to_string(), 'c', 1);
        let entry2 = QuickNavEntry::new("Bar".to_string(), 'd', 2);
        db.add_entry(entry);
        db.add_entry(entry2);
        let result = db.find_entry_by_id(2);
        assert_eq!(result.unwrap().key, 'd');
    }

    // test for upsert
    #[test]
    fn when_title_exists_upsert_changes_key() {
        let mut db = QuickNavDatabase::new();
        let entry = QuickNavEntry::new("Foo".to_string(), 'c', 1);
        let entry2 = QuickNavEntry::new("Bar".to_string(), 'd', 1);
        db.add_entry(entry);
        db.add_entry(entry2);
        assert_eq!(db.entries.len(), 1);
        assert_eq!(db.entries[0].key, 'd');
    }

    // when title doesn't exist upsert adds entry
    #[test]
    fn when_title_does_not_exist_upsert_adds_entry() {
        let mut db = QuickNavDatabase::new();
        let entry = QuickNavEntry::new("Foo".to_string(), 'c', 1);
        db.add_entry(entry);
        assert_eq!(db.entries.len(), 1);
        assert_eq!(db.entries[0].key, 'c');
    }

    // test for clean_up
    #[test]
    fn clean_up_removes_missing_entries() {
        let mut db = QuickNavDatabase::new();
        db.add_entry(QuickNavEntry::new("Foo".to_string(), 'c', 1));
        db.add_entry(QuickNavEntry::new("Bar".to_string(), 'd', 2));
        db.add_entry(QuickNavEntry::new("Baz".to_string(), 'e', 3));
        db.add_entry(QuickNavEntry::new("Bag".to_string(), 'f', 4));

        let entries = vec![
            ("Foo".to_string(), 2),
            ("Bar".to_string(), 3),
            ("Bag".to_string(), 4),
        ];
        db.clean_up(entries);

        assert_eq!(db.entries.len(), 1);
        assert_eq!(db.entries[0].key, 'f');
    }
}
