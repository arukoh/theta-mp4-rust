use std::convert::TryInto;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct RdtBox {
    pub _size: usize,
    pub number_of_entries: u32,
    pub sampling_rate: u16,
    pub sample_size: u16,
    pub endian: u16,
}

impl RdtBox {
    pub(crate) fn read(data: &[u8]) -> RdtBox {
        let number_of_entries = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let sampling_rate = u16::from_le_bytes(data[4..6].try_into().unwrap());
        let sample_size = u16::from_le_bytes(data[6..8].try_into().unwrap());
        let endian = u16::from_le_bytes(data[8..10].try_into().unwrap());
        RdtBox {
            _size: 16,
            number_of_entries,
            sampling_rate,
            sample_size,
            endian,
        }
    }

    pub(crate) fn read_be(data: &[u8]) -> RdtBox {
        let number_of_entries = u32::from_be_bytes(data[0..4].try_into().unwrap());
        let sampling_rate = u16::from_be_bytes(data[4..6].try_into().unwrap());
        let sample_size = u16::from_be_bytes(data[6..8].try_into().unwrap());
        let endian = u16::from_be_bytes(data[8..10].try_into().unwrap());
        RdtBox {
            _size: 16,
            number_of_entries,
            sampling_rate,
            sample_size,
            endian,
        }
    }
}
