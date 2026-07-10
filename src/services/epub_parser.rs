use epub::doc::EpubDoc;
use std::fs::File;

pub struct EpubParser {
    pub file_path: String,
    pub title: String,
    pub total_chapters: usize,
}

impl EpubParser {
    pub fn new(path: &str) -> Result<Self, String> {
        let doc = EpubDoc::new(path).map_err(|e| format!("Failed to parse EPUB: {}", e))?;
        let title = doc
            .mdata("title")
            .unwrap_or_else(|| "Untitled Book".to_owned());
        let total_chapters = doc.resources.len();

        Ok(Self {
            file_path: path.to_owned(),
            title,
            total_chapters,
        })
    }

    pub fn get_chapter(&self, index: usize) -> Result<String, String> {
        let file = File::open(&self.file_path).map_err(|e| e.to_string())?;
        let mut doc = EpubDoc::new(file).map_err(|e| e.to_string())?;

        doc.set_current_page(index)
            .map_err(|_| format!("Chapter index {} out of bounds", index))?;

        let (content_bytes, mime_type) = doc.get_current_mime().map_err(|e| e.to_string())?;

        if mime_type.contains("xhtml") || mime_type.contains("html") || mime_type.contains("xml") {
            String::from_utf8(content_bytes)
                .map_err(|e| format!("Failed to parse chapter content as UTF-8: {}", e))
        } else {
            Err(format!(
                "Resource at index {} is an asset ({}), not text.",
                index, mime_type
            ))
        }
    }
}
