use super::rdt::RdtBox;
use serde::ser::{Serialize, Serializer};

#[derive(Debug, PartialEq, Clone)]
pub struct RdtlBox {
    base: RdtBox,
    data_table: Vec<DataEntry>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataEntry {
    pub timestamp: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

impl RdtlBox {
    pub fn get_entry(&self) -> Vec<DataEntry> {
        self.data_table.clone()
    }

    pub(crate) fn read(data: &[u8]) -> RdtlBox {
        let base = RdtBox::read_be(data);
        let number_of_entries = base.number_of_entries;
        let offset = base._size;
        let mut data_table = Vec::new();
        for i in 0..number_of_entries as usize {
            let entry_offset = offset + i * 32;
            let (timestamp, latitude, longitude, altitude) = match base.endian {
                0x0123 => (
                    f64::from_le_bytes(
                        data[entry_offset as usize..(entry_offset + 8) as usize]
                            .try_into()
                            .unwrap(),
                    ), // timestamp
                    f64::from_le_bytes(
                        data[(entry_offset + 8) as usize..(entry_offset + 16) as usize]
                            .try_into()
                            .unwrap(),
                    ), // latitude
                    f64::from_le_bytes(
                        data[(entry_offset + 16) as usize..(entry_offset + 24) as usize]
                            .try_into()
                            .unwrap(),
                    ), // longitude
                    f64::from_le_bytes(
                        data[(entry_offset + 24) as usize..(entry_offset + 32) as usize]
                            .try_into()
                            .unwrap(),
                    ), // altitude
                ),
                0x3210 => (
                    f64::from_be_bytes(
                        data[entry_offset as usize..(entry_offset + 8) as usize]
                            .try_into()
                            .unwrap(),
                    ), // timestamp
                    f64::from_be_bytes(
                        data[(entry_offset + 8) as usize..(entry_offset + 16) as usize]
                            .try_into()
                            .unwrap(),
                    ), // latitude
                    f64::from_be_bytes(
                        data[(entry_offset + 16) as usize..(entry_offset + 24) as usize]
                            .try_into()
                            .unwrap(),
                    ), // longitude
                    f64::from_be_bytes(
                        data[(entry_offset + 24) as usize..(entry_offset + 32) as usize]
                            .try_into()
                            .unwrap(),
                    ), // altitude
                ),
                _ => panic!("Unknown endian type"),
            };
            data_table.push(DataEntry {
                timestamp,
                latitude,
                longitude,
                altitude,
            });
        }
        RdtlBox { base, data_table }
    }
}

impl Serialize for RdtlBox {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entries: Vec<_> = self
            .data_table
            .iter()
            .map(|entry| {
                serde_json::json!({
                    "timestamp": entry.timestamp,
                    "latitude": entry.latitude,
                    "longitude": entry.longitude,
                    "altitude": entry.altitude
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
            0x00, 0x00, 0x00, 0x02, // number_of_entries
            0x00, 0x01, // sampling_rate
            0x00, 0x02, // sample_size
            0x01, 0x23, // endian (LE: 0x0123)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // reserve (6 bytes)
            // Data Table
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // timestamp
            0x78, 0xb7, 0xb2, 0x44, 0x67, 0xd7, 0x41, 0x40, // latitude
            0x14, 0x79, 0x92, 0x74, 0x4d, 0x78, 0x61, 0x40, // longitude
            0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x60, 0x40, // altitude
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x40, // timestamp
            0x48, 0x33, 0x16, 0x4d, 0x67, 0xd7, 0x41, 0x40, // latitude
            0x20, 0x9a, 0x79, 0x72, 0x4d, 0x78, 0x61, 0x40, // longitude
            0x33, 0x33, 0x33, 0x33, 0x33, 0x63, 0x60, 0x40, // altitude
        ]
    }

    #[test]
    fn test_rdtl_box_read() {
        let data: Vec<u8> = setup();
        // 10,0 to le bytes: [00, 00, 00, 00, 00, 00, 24, 40]
        // println!("{:?}", (10.0 as f64).to_le_bytes().iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>());

        let rdtl_box = RdtlBox::read(&data);
        assert_eq!(rdtl_box.data_table.len(), 2);

        assert_eq!(rdtl_box.data_table[0].timestamp, 0.0);
        assert_eq!(rdtl_box.data_table[0].latitude, 35.682839);
        assert_eq!(rdtl_box.data_table[0].longitude, 139.759455);
        assert_eq!(rdtl_box.data_table[0].altitude, 131.0);

        assert_eq!(rdtl_box.data_table[1].timestamp, 10.0);
        assert_eq!(rdtl_box.data_table[1].latitude, 35.682840);
        assert_eq!(rdtl_box.data_table[1].longitude, 139.759454);
        assert_eq!(rdtl_box.data_table[1].altitude, 131.1);
    }

    #[test]
    fn test_rdtl_box_to_json() {
        let data: Vec<u8> = setup();
        let rdtl_box = RdtlBox::read(&data);
        let json_output = serde_json::to_string_pretty(&rdtl_box).unwrap();

        let expected_json = r#"[
            {"timestamp": 0.0, "latitude": 35.682839, "longitude": 139.759455, "altitude": 131.0},
            {"timestamp": 10.0, "latitude": 35.682840, "longitude": 139.759454, "altitude": 131.1}
        ]"#;
        let expected_json: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        let actual_json: serde_json::Value = serde_json::from_str(&json_output).unwrap();
        assert_eq!(actual_json, expected_json);
    }
}
