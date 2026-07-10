use crate::services::{cbz_parser::CbzReader, pdf_parser::PdfReader, epub_parser::EpubReader};

pub enum Document {
    Cbz(CbzReader),
    Pdf(PdfReader),
    Epub(EpubReader),
}

pub struct ReaderSettings {
    pub zoom_factor: f32,
    pub dark_mode: bool,
}
