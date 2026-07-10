use std::fs::File;
use std::io::Read;
use zip::ZipArchive;

pub struct CbzParser {
    pub file_path: String,
    pub file_names: Vec<String>,
}

impl CbzParser {
    pub fn new(path: &str) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
        let mut archive =
            ZipArchive::new(file).map_err(|e| format!("Invalid zip archive: {}", e))?;
        let mut file_names = Vec::new();

        for i in 0..archive.len() {
            if let Ok(file) = archive.by_index(i) {
                if file.is_file() {
                    let name = file.name().to_lowercase();
                    if name.ends_with(".png")
                        || name.ends_with(".jpg")
                        || name.ends_with(".jpeg")
                        || name.ends_with(".webp")
                    {
                        file_names.push(file.name().to_owned());
                    }
                }
            }
        }

        file_names.sort_by(|a, b| human_sort::compare(a, b));

        if file_names.is_empty() {
            return Err("No supported images found inside the archive.".to_owned());
        }

        Ok(Self {
            file_path: path.to_owned(),
            file_names,
        })
    }

    pub fn get_page(&self, index: usize) -> Result<Vec<u8>, String> {
        if index >= self.file_names.len() {
            return Err("Page index out of bounds".to_owned());
        }
        let file = File::open(&self.file_path).map_err(|e| e.to_string())?;
        let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
        let target_name = &self.file_names[index];
        let mut internal_file = archive.by_name(target_name).map_err(|e| e.to_string())?;

        let mut buffer = Vec::new();
        internal_file
            .read_to_end(&mut buffer)
            .map_err(|e| e.to_string())?;
        Ok(buffer)
    }
}
