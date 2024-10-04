use super::rdt::RdtBox;
use serde::ser::{Serialize, Serializer};

#[derive(Debug, PartialEq, Clone)]
pub struct Rdl2Box {
    base: RdtBox,
    data_table: Vec<DataEntry>,
}

#[derive(Debug, PartialEq, Clone)]
struct DataEntry {
    timestamp: f64,
    gps_fix_type: i16,
    latitude: f64,
    longitude: f64,
    altitude: f32,
    horizontal_accuracy: f32,
    vertical_accuracy: f32,
    velocity_east: f32,
    velocity_north: f32,
    velocity_up: f32,
    speed_accuracy: f32,
}

impl Rdl2Box {
    pub fn read(data: &[u8]) -> Rdl2Box {
        let base = RdtBox::read(data);
        let number_of_entries = base.number_of_entries;
        let offset = base._size;
        let mut data_table = Vec::new();
        for i in 0..number_of_entries as usize {
            let entry_offset = offset + i * 54;
            let (timestamp, gps_fix_type, latitude, longitude, altitude, horizontal_accuracy, vertical_accuracy, velocity_east, velocity_north, velocity_up, speed_accuracy) = match base.endian {
                0x0123 => (
                    f64::from_le_bytes(data[entry_offset as usize..(entry_offset + 8) as usize].try_into().unwrap()), // timestamp
                    i16::from_le_bytes(data[entry_offset as usize + 8..entry_offset as usize + 10].try_into().unwrap()), // gps_fix_type
                    f64::from_le_bytes(data[entry_offset as usize + 10..entry_offset as usize + 18].try_into().unwrap()), // latitude
                    f64::from_le_bytes(data[entry_offset as usize + 18..entry_offset as usize + 26].try_into().unwrap()), // longitude
                    f32::from_le_bytes(data[entry_offset as usize + 26..entry_offset as usize + 30].try_into().unwrap()), // altitude
                    f32::from_le_bytes(data[entry_offset as usize + 30..entry_offset as usize + 34].try_into().unwrap()), // horizontal_accuracy
                    f32::from_le_bytes(data[entry_offset as usize + 34..entry_offset as usize + 38].try_into().unwrap()), // vertical_accuracy
                    f32::from_le_bytes(data[entry_offset as usize + 38..entry_offset as usize + 42].try_into().unwrap()), // velocity_east
                    f32::from_le_bytes(data[entry_offset as usize + 42..entry_offset as usize + 46].try_into().unwrap()), // velocity_north
                    f32::from_le_bytes(data[entry_offset as usize + 46..entry_offset as usize + 50].try_into().unwrap()), // velocity_up
                    f32::from_le_bytes(data[entry_offset as usize + 50..entry_offset as usize + 54].try_into().unwrap()), // speed_accuracy
                ),
                0x3210 => (
                    f64::from_be_bytes(data[entry_offset as usize..(entry_offset + 8) as usize].try_into().unwrap()), // timestamp
                    i16::from_be_bytes(data[entry_offset as usize + 8..entry_offset as usize + 10].try_into().unwrap()), // gps_fix_type
                    f64::from_be_bytes(data[entry_offset as usize + 10..entry_offset as usize + 18].try_into().unwrap()), // latitude
                    f64::from_be_bytes(data[entry_offset as usize + 18..entry_offset as usize + 26].try_into().unwrap()), // longitude
                    f32::from_be_bytes(data[entry_offset as usize + 26..entry_offset as usize + 30].try_into().unwrap()), // altitude
                    f32::from_be_bytes(data[entry_offset as usize + 30..entry_offset as usize + 34].try_into().unwrap()), // horizontal_accuracy
                    f32::from_be_bytes(data[entry_offset as usize + 34..entry_offset as usize + 38].try_into().unwrap()), // vertical_accuracy
                    f32::from_be_bytes(data[entry_offset as usize + 38..entry_offset as usize + 42].try_into().unwrap()), // velocity_east
                    f32::from_be_bytes(data[entry_offset as usize + 42..entry_offset as usize + 46].try_into().unwrap()), // velocity_north
                    f32::from_be_bytes(data[entry_offset as usize + 46..entry_offset as usize + 50].try_into().unwrap()), // velocity_up
                    f32::from_be_bytes(data[entry_offset as usize + 50..entry_offset as usize + 54].try_into().unwrap()), // speed_accuracy
                ),
                _ => panic!("Unknown endian type"),
            };
            data_table.push(DataEntry {
                timestamp,
                gps_fix_type,
                latitude,
                longitude,
                altitude,
                horizontal_accuracy,
                vertical_accuracy,
                velocity_east,
                velocity_north,
                velocity_up,
                speed_accuracy,
            });
        }

        Rdl2Box { base, data_table }
    }
}

impl Serialize for Rdl2Box {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entries: Vec<_> = self.data_table.iter().map(|entry| {
            serde_json::json!({
                "timestamp": entry.timestamp,
                "gps_fix_type": entry.gps_fix_type,
                "latitude": entry.latitude,
                "longitude": entry.longitude,
                "altitude": entry.altitude.to_string().parse::<f64>().unwrap(),
                "horizontal_accuracy": entry.horizontal_accuracy.to_string().parse::<f64>().unwrap(),
                "vertical_accuracy": entry.vertical_accuracy.to_string().parse::<f64>().unwrap(),
                "velocity_east": entry.velocity_east.to_string().parse::<f64>().unwrap(),
                "velocity_north": entry.velocity_north.to_string().parse::<f64>().unwrap(),
                "velocity_up": entry.velocity_up.to_string().parse::<f64>().unwrap(),
                "speed_accuracy": entry.speed_accuracy.to_string().parse::<f64>().unwrap(),
            })
        }).collect();
        serializer.serialize_some(&entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn setup() -> Vec<u8> {
        vec![
            0x02, 0x00, 0x00, 0x00, // number_of_entries
            0x01, 0x00,             // sampling_rate
            0x02, 0x00,             // sample_size
            0x23, 0x01,             // endian (LE: 0x0123)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // reserve (6 bytes)

            // Data Table
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // timestamp
            0x01, 0x00,             // gps_fix_type = 1
            0x78, 0xb7, 0xb2, 0x44, 0x67, 0xd7, 0x41, 0x40, // latitude
            0x14, 0x79, 0x92, 0x74, 0x4d, 0x78, 0x61, 0x40, // longitude
            0x00, 0x00, 0x03, 0x43, // altitude
            0x00, 0x00, 0x80, 0x3f, // horizontal_accuracy = 1.0
            0xcd, 0xcc, 0x8c, 0x3f, // vertical_accuracy = 1.1
            0x9a, 0x99, 0x99, 0x3f, // velocity_east = 1.2
            0x66, 0x66, 0xa6, 0x3f, // velocity_north = 1.3
            0x33, 0x33, 0xb3, 0x3f, // velocity_up = 1.4
            0x00, 0x00, 0xc0, 0x3f, // speed_accuracy = 1.5
            
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x40, // timestamp
            0x02, 0x00,             // gps_fix_type = 2
            0x48, 0x33, 0x16, 0x4d, 0x67, 0xd7, 0x41, 0x40, // latitude
            0x20, 0x9a, 0x79, 0x72, 0x4d, 0x78, 0x61, 0x40, // longitude
            0x9a, 0x19, 0x03, 0x43, // altitude
            0x00, 0x00, 0x00, 0x40, // horizontal_accuracy = 2.0
            0x66, 0x66, 0x06, 0x40, // vertical_accuracy = 2.1
            0xcd, 0xcc, 0x0c, 0x40, // velocity_east = 2.2
            0x33, 0x33, 0x13, 0x40, // velocity_north = 2.3
            0x9a, 0x99, 0x19, 0x40, // velocity_up = 2.4
            0x00, 0x00, 0x20, 0x40, // speed_accuracy = 2.5
        ]
    }

    #[test]
    fn test_rdl2box_read() {
        let data: Vec<u8> = setup();

        let rdl2_box = Rdl2Box::read(&data);
        assert_eq!(rdl2_box.data_table.len(), 2);

        assert_eq!(rdl2_box.data_table[0].timestamp, 0.0);
        assert_eq!(rdl2_box.data_table[0].gps_fix_type, 1);
        assert_eq!(rdl2_box.data_table[0].latitude, 35.682839);
        assert_eq!(rdl2_box.data_table[0].longitude, 139.759455);
        assert_eq!(rdl2_box.data_table[0].altitude, 131.0);
        assert_eq!(rdl2_box.data_table[0].horizontal_accuracy, 1.0);
        assert_eq!(rdl2_box.data_table[0].vertical_accuracy, 1.1);
        assert_eq!(rdl2_box.data_table[0].velocity_east, 1.2);
        assert_eq!(rdl2_box.data_table[0].velocity_north, 1.3);
        assert_eq!(rdl2_box.data_table[0].velocity_up, 1.4);
        assert_eq!(rdl2_box.data_table[0].speed_accuracy, 1.5);
        
        assert_eq!(rdl2_box.data_table[1].timestamp, 10.0);
        assert_eq!(rdl2_box.data_table[1].gps_fix_type, 2);
        assert_eq!(rdl2_box.data_table[1].latitude, 35.682840);
        assert_eq!(rdl2_box.data_table[1].longitude, 139.759454);
        assert_eq!(rdl2_box.data_table[1].altitude, 131.1);
        assert_eq!(rdl2_box.data_table[1].horizontal_accuracy, 2.0);
        assert_eq!(rdl2_box.data_table[1].vertical_accuracy, 2.1);
        assert_eq!(rdl2_box.data_table[1].velocity_east, 2.2);
        assert_eq!(rdl2_box.data_table[1].velocity_north, 2.3);
        assert_eq!(rdl2_box.data_table[1].velocity_up, 2.4);
        assert_eq!(rdl2_box.data_table[1].speed_accuracy, 2.5);

    }
    
    #[test]
    fn test_rdl2_box_to_json() {
        let data: Vec<u8> = setup();
        let rdl2_box = Rdl2Box::read(&data);
        let json_output = serde_json::to_string_pretty(&rdl2_box).unwrap();
        
        
        let a = f32::from_le_bytes(vec![0x9a, 0x99, 0x99, 0x3f].try_into().unwrap());
        print!("{}", a.to_string());
        println!("{:?}", (1.0 as f32).to_le_bytes().iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>());
        println!("{:?}", (1.2 as f32).to_le_bytes().iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<String>>());

        let expected_json = r#"[
            {
                "timestamp": 0.0,
                "gps_fix_type": 1,
                "latitude": 35.682839,
                "longitude": 139.759455,
                "altitude": 131.0,
                "horizontal_accuracy": 1.0,
                "vertical_accuracy": 1.1,
                "velocity_east": 1.2,
                "velocity_north": 1.3,
                "velocity_up": 1.4,
                "speed_accuracy": 1.5
            },
            {
                "timestamp": 10.0,
                "gps_fix_type": 2,
                "latitude": 35.682840,
                "longitude": 139.759454,
                "altitude": 131.1,
                "horizontal_accuracy": 2.0,
                "vertical_accuracy": 2.1,
                "velocity_east": 2.2,
                "velocity_north": 2.3,
                "velocity_up": 2.4,
                "speed_accuracy": 2.5
            }
        ]"#;
        let expected_json: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        let actual_json: serde_json::Value = serde_json::from_str(&json_output).unwrap();
        assert_eq!(actual_json, expected_json);
    }
}
