use super::rdt::RdtBox;
use serde::ser::{Serialize, Serializer};

#[derive(Debug, PartialEq, Clone)]
pub struct RdtbBox {
    base: RdtBox,
    data_table: Vec<DataEntry>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataEntry {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub reserve: f32,
    pub timestamp: u64,
}

impl RdtbBox {
    pub fn get_entry(&self) -> Vec<DataEntry> {
        self.data_table.clone()
    }

    pub(crate) fn read(data: &[u8]) -> RdtbBox {
        let base = RdtBox::read(data);
        let number_of_entries = base.number_of_entries;
        let offset = base._size;
        let mut data_table = Vec::new();
        for i in 0..number_of_entries as usize {
            let entry_offset = offset + i * 24;
            let (x, y, z, reserve, timestamp) = match base.endian {
                0x0123 => (
                    f32::from_le_bytes(data[entry_offset..entry_offset + 4].try_into().unwrap()), // x
                    f32::from_le_bytes(
                        data[entry_offset + 4..entry_offset + 8].try_into().unwrap(),
                    ), // y
                    f32::from_le_bytes(
                        data[entry_offset + 8..entry_offset + 12]
                            .try_into()
                            .unwrap(),
                    ), // z
                    f32::from_le_bytes(
                        data[entry_offset + 12..entry_offset + 16]
                            .try_into()
                            .unwrap(),
                    ), // reserve
                    u64::from_le_bytes(
                        data[entry_offset + 16..entry_offset + 24]
                            .try_into()
                            .unwrap(),
                    ), // timestamp
                ),
                0x3210 => (
                    f32::from_be_bytes(data[entry_offset..entry_offset + 4].try_into().unwrap()), // x
                    f32::from_be_bytes(
                        data[entry_offset + 4..entry_offset + 8].try_into().unwrap(),
                    ), // y
                    f32::from_be_bytes(
                        data[entry_offset + 8..entry_offset + 12]
                            .try_into()
                            .unwrap(),
                    ), // z
                    f32::from_be_bytes(
                        data[entry_offset + 12..entry_offset + 16]
                            .try_into()
                            .unwrap(),
                    ), // reserve
                    u64::from_be_bytes(
                        data[entry_offset + 16..entry_offset + 24]
                            .try_into()
                            .unwrap(),
                    ), // timestamp
                ),
                _ => panic!("Unknown endian type"),
            };
            data_table.push(DataEntry {
                x,
                y,
                z,
                reserve,
                timestamp,
            });
        }
        RdtbBox { base, data_table }
    }
}

impl Serialize for RdtbBox {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entries: Vec<_> = self
            .data_table
            .iter()
            .map(|entry| {
                serde_json::json!({
                    "x": entry.x,
                    "y": entry.y,
                    "z": entry.z,
                    "timestamp": entry.timestamp
                })
            })
            .collect();
        serializer.serialize_some(&entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Vec<u8> {
        vec![
            0x02, 0x00, 0x00, 0x00, // number_of_entries
            0x01, 0x00, // sampling_rate
            0x02, 0x00, // sample_size
            0x23, 0x01, // endian (LE: 0x0123)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // reserve (6 bytes)
            // Data Table
            0x00, 0x00, 0x20, 0x41, // x = 10.0
            0x00, 0x00, 0x40, 0x41, // y = 12.0
            0x00, 0x00, 0x60, 0x41, // z = 14.0
            0x00, 0x00, 0x00, 0x00, // reserve
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // timestamp = 1
            0x00, 0x00, 0xa0, 0x41, // x = 20.0
            0x00, 0x00, 0xb0, 0x41, // y = 22.0
            0x00, 0x00, 0xc0, 0x41, // z = 24.0
            0x00, 0x00, 0x00, 0x00, // reserve
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // timestamp = 2
        ]
    }

    #[test]
    fn test_rdtbbox_read() {
        let data: Vec<u8> = setup();
        // 10,0 to le bytes: [00, 00, 24, 40]
        // println!("{:?}", (10.0 as f32).to_le_bytes().iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>());

        let rdtb_box = RdtbBox::read(&data);
        assert_eq!(rdtb_box.data_table.len(), 2);

        assert_eq!(rdtb_box.data_table[0].x, 10.0);
        assert_eq!(rdtb_box.data_table[0].y, 12.0);
        assert_eq!(rdtb_box.data_table[0].z, 14.0);
        assert_eq!(rdtb_box.data_table[0].timestamp, 1);

        assert_eq!(rdtb_box.data_table[1].x, 20.0);
        assert_eq!(rdtb_box.data_table[1].y, 22.0);
        assert_eq!(rdtb_box.data_table[1].z, 24.0);
        assert_eq!(rdtb_box.data_table[1].timestamp, 2);
    }

    #[test]
    fn test_rdtb_box_to_json() {
        let data: Vec<u8> = setup();
        let rdtb_box = RdtbBox::read(&data);
        let json_output = serde_json::to_string_pretty(&rdtb_box).unwrap();

        let expected_json = r#"[
            {"x": 10.0, "y": 12.0, "z": 14.0, "timestamp": 1},
            {"x": 20.0, "y": 22.0, "z": 24.0, "timestamp": 2}
        ]"#;
        let expected_json: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        let actual_json: serde_json::Value = serde_json::from_str(&json_output).unwrap();
        assert_eq!(actual_json, expected_json);
    }
}
