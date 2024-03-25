#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() {
    let mut native_options = eframe::NativeOptions::default();

    #[cfg(target_os = "windows")]
    {
        // Windows-Only
        let format = image::ImageFormat::Ico;
        let bytes = include_bytes!("favicon.ico");
        if let Ok(icon) = load_icon_from_bytes(bytes, format) {
            native_options.icon_data = Some(icon);
        } else {
            // Failed to load the icon from bytes stored in the binary.
        }
    }

    eframe::run_native(
        "School Search",
        native_options,
        Box::new(|cc| Box::new(school_search::App::new(cc))),
    );
}

fn load_icon_from_bytes(
    bytes: &[u8],
    format: image::ImageFormat,
) -> Result<eframe::IconData, image::error::ImageError> {
    use image::GenericImageView;

    let (rgba, width, height) = {
        let image = image::load_from_memory_with_format(bytes, format)?;
        let (width, height) = image.dimensions();
        let rgba = image.into_bytes();
        (rgba, width, height)
    };

    Ok(eframe::IconData {
        rgba,
        width,
        height,
    })
}
