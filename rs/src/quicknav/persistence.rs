#[cfg(test)]
use mockall::automock;

use crate::error::KittyMuxError;

use super::QuickNavDatabase;

#[cfg_attr(test, automock)]
pub trait QuickNavPersistence {
    fn load(&self) -> Result<QuickNavDatabase, KittyMuxError>; 
    fn save(&self, entries: &QuickNavDatabase) -> Result<(), KittyMuxError>;
}

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

pub struct ConfigFileQuickNavPersistence {
    file_path: String,
}

impl ConfigFileQuickNavPersistence {
    pub fn new(file_path: String) -> ConfigFileQuickNavPersistence {
        ConfigFileQuickNavPersistence { file_path }
    }
}

impl QuickNavPersistence for ConfigFileQuickNavPersistence {
    fn load(&self) -> Result<QuickNavDatabase, KittyMuxError> {
        let file = std::fs::File::open(&self.file_path);
        if let Ok(file) = file {
            let reader = std::io::BufReader::new(file);
            let entries: QuickNavDatabase = serde_json::from_reader(reader)?;
            Ok(entries)
        } else {
            Ok(QuickNavDatabase::new())
        }
    }

    fn save(&self, entries: &QuickNavDatabase) -> Result<(), KittyMuxError>{
        let file = std::fs::File::create(&self.file_path);
        if let Ok(file) = file {
            let writer = std::io::BufWriter::new(file);
            serde_json::to_writer(writer, entries)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::quicknav::QuickNavEntry;

    use super::*;

    fn get_test_entries() -> QuickNavDatabase {
        QuickNavDatabase::from_entries(vec![
         QuickNavEntry::new("Foo".to_string(), 'c', 1),
         QuickNavEntry::new("Bar".to_string(), 'd', 2), 
        ])
    }


    #[test]
    fn saves_to_json() {
        let entries = get_test_entries();
        let persistence = super::ConfigFileQuickNavPersistence::new("savetest.json".to_string());
        persistence.save(&entries).expect("Failed to save QuickNavDatabase");
        let file = std::fs::File::open("savetest.json");
        assert!(file.is_ok());
        
        let file_contents = std::fs::read_to_string("savetest.json").unwrap();
        assert_eq!(file_contents, r###"{"entries":[{"title":"Foo","id":1,"key":"c"},{"title":"Bar","id":2,"key":"d"}]}"###);
        std::fs::remove_file("savetest.json").unwrap();
    }

    #[test]
    fn loads_from_json() {
        let text = r###"{"entries":[{"title":"Foo","id":1,"key":"c"},{"title":"Bar","id":2,"key":"d"}]}"###;
        std::fs::write("loadtest.json", text).unwrap();
        let expected = get_test_entries();

        let persistence = super::ConfigFileQuickNavPersistence::new("loadtest.json".to_string());
        let loaded_entries = persistence.load().expect("Failed to load a QuickNavDatabase");
        assert_eq!(expected, loaded_entries);

        std::fs::remove_file("loadtest.json").unwrap();
    }
}
