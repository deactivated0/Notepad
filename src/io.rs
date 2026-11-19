use std::path::PathBuf;

/// All supported file extensions grouped by category.
pub const ALL_SUPPORTED_EXTENSIONS: &[&str] = &[
    // Text
    "txt", "text", "log", // Markdown
    "md", "markdown", // Configuration
    "json", "xml", "yaml", "yml", "toml", "ini", "conf", // Code
    "rs", "py", "js", "ts", "html", "css", "c", "cpp", "java", "sh", // Data
    "csv", "tsv", "sql", // Other
    "bat", "ps1", "svg",
];

pub const TEXT_EXTENSIONS: &[&str] = &["txt", "text", "log"];
pub const MARKDOWN_EXTENSIONS: &[&str] = &["md", "markdown"];
pub const CONFIG_EXTENSIONS: &[&str] = &["json", "xml", "yaml", "yml", "toml", "ini", "conf"];
pub const CODE_EXTENSIONS: &[&str] = &[
    "rs", "py", "js", "ts", "html", "css", "c", "cpp", "java", "sh",
];
pub const DATA_EXTENSIONS: &[&str] = &["csv", "tsv", "sql"];
pub const OTHER_EXTENSIONS: &[&str] = &["bat", "ps1", "svg"];

/// Result of an async file open operation (works on both Native & WASM).
#[derive(Debug, Clone)]
pub struct LoadedFile {
    pub path: Option<PathBuf>,
    pub name: String,
    pub contents: String,
}

/// Result of an async file save operation.
#[derive(Debug, Clone)]
pub struct SavedFile {
    pub path: Option<PathBuf>,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum IoError {
    DialogClosed,
    ReadFailed(String),
    WriteFailed(String),
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoError::DialogClosed => f.write_str("File dialog cancelled"),
            IoError::ReadFailed(e) => write!(f, "Failed to read file: {e}"),
            IoError::WriteFailed(e) => write!(f, "Failed to save file: {e}"),
        }
    }
}

fn attach_filters(dialog: rfd::AsyncFileDialog) -> rfd::AsyncFileDialog {
    dialog
        .add_filter("All Supported Files", ALL_SUPPORTED_EXTENSIONS)
        .add_filter("Text (*.txt, *.text, *.log)", TEXT_EXTENSIONS)
        .add_filter("Markdown (*.md, *.markdown)", MARKDOWN_EXTENSIONS)
        .add_filter(
            "Configuration (*.json, *.xml, *.yaml, *.yml, *.toml, *.ini, *.conf)",
            CONFIG_EXTENSIONS,
        )
        .add_filter(
            "Code (*.rs, *.py, *.js, *.ts, *.html, *.css, *.c, *.cpp, *.java, *.sh)",
            CODE_EXTENSIONS,
        )
        .add_filter("Data (*.csv, *.tsv, *.sql)", DATA_EXTENSIONS)
        .add_filter("Other (*.bat, *.ps1, *.svg)", OTHER_EXTENSIONS)
        .add_filter("All Files", &["*"])
}

/// Opens a native or browser file picker and reads UTF-8 text asynchronously.
pub async fn open_file_dialog() -> Result<LoadedFile, IoError> {
    let handle = attach_filters(rfd::AsyncFileDialog::new().set_title("Open"))
        .pick_file()
        .await
        .ok_or(IoError::DialogClosed)?;

    let name = handle.file_name();
    let bytes = handle.read().await;
    let contents = String::from_utf8(bytes)
        .unwrap_or_else(|e| String::from_utf8_lossy(&e.into_bytes()).into_owned());

    #[cfg(not(target_arch = "wasm32"))]
    let path = Some(handle.path().to_path_buf());
    #[cfg(target_arch = "wasm32")]
    let path = None;

    Ok(LoadedFile {
        path,
        name,
        contents,
    })
}

/// Loads a specific path on native platforms (e.g., drag-and-drop or CLI arg).
#[cfg(not(target_arch = "wasm32"))]
pub async fn read_path(path: PathBuf) -> Result<LoadedFile, IoError> {
    let bytes = tokio::fs::read(&path)
        .await
        .map_err(|e| IoError::ReadFailed(e.to_string()))?;
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "Untitled.txt".to_string());
    let contents = String::from_utf8(bytes)
        .unwrap_or_else(|e| String::from_utf8_lossy(&e.into_bytes()).into_owned());
    Ok(LoadedFile {
        path: Some(path),
        name,
        contents,
    })
}

/// Saves text to an existing path or opens a Save As dialog (Native) / triggers download (WASM).
pub async fn save_file(
    existing_path: Option<PathBuf>,
    suggested_name: String,
    contents: String,
) -> Result<SavedFile, IoError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let target_path = match existing_path {
            Some(path) => path,
            None => {
                let handle = attach_filters(
                    rfd::AsyncFileDialog::new()
                        .set_title("Save As")
                        .set_file_name(&suggested_name),
                )
                .save_file()
                .await
                .ok_or(IoError::DialogClosed)?;
                handle.path().to_path_buf()
            }
        };

        tokio::fs::write(&target_path, contents.as_bytes())
            .await
            .map_err(|e| IoError::WriteFailed(e.to_string()))?;

        let name = target_path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or(suggested_name);

        Ok(SavedFile {
            path: Some(target_path),
            name,
        })
    }

    #[cfg(target_arch = "wasm32")]
    {
        let _ = existing_path;
        trigger_browser_download(&suggested_name, &contents).map_err(IoError::WriteFailed)?;
        Ok(SavedFile {
            path: None,
            name: suggested_name,
        })
    }
}

#[cfg(target_arch = "wasm32")]
fn mime_type_for_filename(filename: &str) -> &'static str {
    let ext = filename
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "md" | "markdown" => "text/markdown;charset=utf-8",
        "json" => "application/json;charset=utf-8",
        "xml" => "application/xml;charset=utf-8",
        "yaml" | "yml" => "application/x-yaml;charset=utf-8",
        "toml" => "application/toml;charset=utf-8",
        "html" => "text/html;charset=utf-8",
        "css" => "text/css;charset=utf-8",
        "js" => "text/javascript;charset=utf-8",
        "ts" => "text/typescript;charset=utf-8",
        "csv" => "text/csv;charset=utf-8",
        "tsv" => "text/tab-separated-values;charset=utf-8",
        "sql" => "application/sql;charset=utf-8",
        "svg" => "image/svg+xml;charset=utf-8",
        _ => "text/plain;charset=utf-8",
    }
}

/// Triggers a clean client-side Blob download on WebAssembly browsers.
#[cfg(target_arch = "wasm32")]
fn trigger_browser_download(filename: &str, contents: &str) -> Result<(), String> {
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No browser document")?;

    let parts = js_sys::Array::new();
    parts.push(&wasm_bindgen::JsValue::from_str(contents));

    let bag = BlobPropertyBag::new();
    bag.set_type(mime_type_for_filename(filename));

    let blob = Blob::new_with_str_sequence_and_options(&parts, &bag)
        .map_err(|e| format!("{e:?}"))?;
    let url = Url::create_object_url_with_blob(&blob).map_err(|e| format!("{e:?}"))?;

    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .map_err(|e| format!("{e:?}"))?
        .dyn_into()
        .map_err(|_| "Failed to create anchor element")?;
    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.click();

    let _ = Url::revoke_object_url(&url);
    Ok(())
}
