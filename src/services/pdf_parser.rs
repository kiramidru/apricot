<<<<<<< HEAD
use pdf_oxide::{Error, PdfDocument};

pub fn parse_pdf_content(file_path: &str) -> Result<String, Error> {
    let doc = PdfDocument::open(file_path)?;
    let text = doc.extract_all_text()?;
    Ok(text)
}

=======
use pdfium_render::prelude::*;
use std::sync::{Arc, Mutex};

pub struct PdfParser {
    pub file_path: String,
    pub total_pages: usize,
    pdfium: Arc<Mutex<Pdfium>>,
}

impl PdfParser {
    pub fn new(path: &str) -> Result<Self, String> {
        let bindings = Pdfium::bind_to_system_library()
            .or_else(|_| Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name()))
            .map_err(|e| format!("Failed to find/bind Pdfium system library: {:?}", e))?;

        let pdfium = Pdfium::new(bindings);

        let document = pdfium
            .load_pdf_from_file(path, None)
            .map_err(|e| format!("Failed to read PDF file content: {:?}", e))?;

        let total_pages = document.pages().len() as usize;

        Ok(Self {
            file_path: path.to_owned(),
            total_pages,
            pdfium: Arc::new(Mutex::new(pdfium)),
        })
    }

    pub fn render_page_to_image(&self, index: usize) -> Result<(u32, u32, Vec<u8>), String> {
        let pdfium = self
            .pdfium
            .lock()
            .map_err(|_| "Failed to lock Pdfium instance")?;

        let document = pdfium
            .load_pdf_from_file(&self.file_path, None)
            .map_err(|e| e.to_string())?;

        let page = document
            .pages()
            .get(index as u16)
            .map_err(|e| format!("Page index bounds error: {:?}", e))?;

        // Render the vector page configuration into a bitmap image layout at 150 DPI
        let render_config = PdfRenderConfig::new()
            .set_target_width(800) // Standardizes horizontal constraint width
            .set_maximum_height(2000);

        let bitmap = page
            .render_with_config(&render_config)
            .map_err(|e| format!("Failed bitmap graphics pipeline generation: {:?}", e))?;

        let width = bitmap.width() as u32;
        let height = bitmap.height() as u32;

        let rgba_bytes = bitmap.as_rgba().to_vec();

        Ok((width, height, rgba_bytes))
    }
}
>>>>>>> 40d0dee (init commit)
