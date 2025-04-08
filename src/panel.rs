extern crate csv;
use crate::definition::AlleleDefinition;
use csv::ReaderBuilder;
use std::collections::hash_map;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

pub struct MicrohapPanel {
    definitions: HashMap<String, AlleleDefinition>,
}

impl MicrohapPanel {
    pub fn from_csv(csv_path: &PathBuf) -> Result<MicrohapPanel, Box<dyn Error>> {
        let mut reader = ReaderBuilder::new().from_path(csv_path)?;
        let mut definitions = HashMap::new();
        for result in reader.records() {
            let record = result?;
            let identifier = &record[0];
            let chrom = &record[1];
            let offset = record[2].parse::<u32>()?;
            let definition = definitions
                .entry(identifier.to_owned())
                .or_insert_with(|| AlleleDefinition::new(&chrom));
            definition.add_snp_offset(offset);
        }
        Ok(MicrohapPanel { definitions })
    }

    pub fn iter(&self) -> hash_map::Iter<String, AlleleDefinition> {
        self.definitions.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl MicrohapPanel {
        pub fn len(&self) -> usize {
            self.definitions.len()
        }

        pub fn get(&self, identifier: &str) -> Option<&AlleleDefinition> {
            self.definitions.get(identifier)
        }
    }

    #[test]
    fn test_panel_basic() {
        let panel = MicrohapPanel::from_csv(&PathBuf::from("testdata/twomh.csv"))
            .expect("issue parsing panel CSV");
        assert_eq!(panel.len(), 2);
        assert_eq!(panel.get("mh04WL-069").unwrap().extent(), 277);
        assert_eq!(panel.get("mh13KK-223.v1").unwrap().extent(), 154);
        assert!(panel.get("mh02KK-138.v2").is_none());
    }

    #[test]
    fn test_panel_iter() {
        let panel = MicrohapPanel::from_csv(&PathBuf::from("testdata/nimathree.csv"))
            .expect("issue parsing panel CSV");
        let id_def_pairs: Vec<(&String, &AlleleDefinition)> = panel.iter().collect();
        assert_eq!(id_def_pairs.len(), 3);
    }
}
