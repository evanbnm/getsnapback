use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum ProcessorError {
    #[error("IO error on {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Image error on {path}: {source}")]
    Image {
        path: PathBuf,
        #[source]
        source: image::ImageError,
    },

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Tool '{tool}' failed (exit {code}):\n{stderr}")]
    Sidecar {
        tool: String,
        code: i32,
        stderr: String,
    },

    #[error("Tool not found: {0}")]
    SidecarNotFound(String),

    #[error("Input not found: {0}")]
    InputNotFound(PathBuf),

    #[error("Cancelled by user")]
    Cancelled,
}

impl ProcessorError {
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io { path: path.into(), source }
    }

    pub fn image(path: impl Into<PathBuf>, source: image::ImageError) -> Self {
        Self::Image { path: path.into(), source }
    }
}
