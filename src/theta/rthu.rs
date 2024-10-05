use std::{ffi::OsStr, fs::File, io::Write, path::Path};

#[derive(Debug, PartialEq, Clone)]
pub struct RthuBox {
    pub data: Vec<u8>,
}

impl RthuBox {
    pub fn write_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
        let path = Path::new(filename);
        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        let stem = path.file_stem().unwrap_or_else(|| OsStr::new("output"));
        let thumbnail_filename = dir.join(format!("{}_thumbnail.jfif", stem.to_string_lossy()));

        let mut file = File::create(&thumbnail_filename)?;
        file.write_all(&self.data)?;

        Ok(())
    }
}
