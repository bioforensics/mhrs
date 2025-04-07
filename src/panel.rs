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
            let identifier = record[0].to_owned();
            let chrom = record[1].to_owned();
            let offset = record[2].parse::<u32>()?;
            let definition = definitions
                .entry(identifier)
                .or_insert_with(|| AlleleDefinition::new(chrom));
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

        pub fn get(&self, identifier: &String) -> Option<&AlleleDefinition> {
            self.definitions.get(identifier)
        }
    }

    #[test]
    fn test_panel_basic() {
        let panel = MicrohapPanel::from_csv(&PathBuf::from("testdata/twomh.csv"))
            .expect("issue parsing panel CSV");
        assert_eq!(panel.len(), 2);

        let id1 = "mh04WL-069".to_string();
        let def1 = panel.get(&id1).unwrap();
        assert_eq!(def1.extent(), 277);

        let id2 = "mh13KK-223.v1".to_string();
        let def2 = panel.get(&id2).unwrap();
        assert_eq!(def2.extent(), 154);

        let id3 = "mh02KK-138.v2".to_string();
        let def3 = panel.get(&id3);
        assert!(def3.is_none());
    }

    #[test]
    fn test_panel_iter() {
        let panel = MicrohapPanel::from_csv(&PathBuf::from("testdata/nimathree.csv"))
            .expect("issue parsing panel CSV");
        let id_def_pairs: Vec<(&String, &AlleleDefinition)> = panel.iter().collect();
        assert_eq!(id_def_pairs.len(), 3);
    }
}
