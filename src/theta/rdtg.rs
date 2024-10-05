use super::rdt::RdtBox;
use serde::ser::{Serialize, Serializer};

#[derive(Debug, PartialEq, Clone)]
pub struct RdtgBox {
    base: RdtBox,
    data_table: Vec<DataEntry>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataEntry {
    pub timestamp: u64,
}

impl RdtgBox {
    pub fn get_entry(&self) -> Vec<DataEntry> {
        self.data_table.clone()
    }

    pub(crate) fn read(data: &[u8]) -> RdtgBox {
        let base = RdtBox::read(data);
        let number_of_entries = base.number_of_entries;
        let offset = base._size;
        let mut data_table = Vec::new();
        for i in 0..number_of_entries as usize {
            let entry_offset = offset + i * 8;
            let timestamp = match base.endian {
                0x0123 => u64::from_le_bytes(
                    data[entry_offset as usize..(entry_offset + 8) as usize]
                        .try_into()
                        .unwrap(),
                ),
                0x3210 => u64::from_be_bytes(
                    data[entry_offset as usize..(entry_offset + 8) as usize]
                        .try_into()
                        .unwrap(),
                ),
                _ => panic!("Unknown endian type"),
            };
            data_table.push(DataEntry { timestamp });
        }
        RdtgBox { base, data_table }
    }
}

impl Serialize for RdtgBox {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamps: Vec<u64> = self
            .data_table
            .iter()
            .map(|entry| entry.timestamp)
            .collect();
        serializer.serialize_some(&timestamps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn setup() -> Vec<u8> {
        vec![
            0x02, 0x00, 0x00, 0x00, // number_of_entries
            0x01, 0x00, // sampling_rate
            0x02, 0x00, // sample_size
            0x23, 0x01, // endian (LE: 0x0123)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // reserve (6 bytes)
            // Data Table
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // timestamp (1)
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // timestamp (4)
        ]
    }

    #[test]
    fn test_rdtg_box_read() {
        let data: Vec<u8> = setup();
        let rdtg_box = RdtgBox::read(&data);
        assert_eq!(rdtg_box.data_table.len(), 2);
        assert_eq!(rdtg_box.data_table[0].timestamp, 1);
        assert_eq!(rdtg_box.data_table[1].timestamp, 4);
    }

    #[test]
    fn test_rdtg_box_to_json() {
        let data: Vec<u8> = setup();
        let rdtg_box = RdtgBox::read(&data);
        let json_output = serde_json::to_string_pretty(&rdtg_box).unwrap();

        let expected_json = r#"[ 1, 4 ]"#;
        let expected_json: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        let actual_json: serde_json::Value = serde_json::from_str(&json_output).unwrap();
        assert_eq!(actual_json, expected_json);
    }
}
