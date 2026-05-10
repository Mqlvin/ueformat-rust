use thiserror::Error;


#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Cursor error: {0}")]
    CursorError(std::io::Error),

    #[error("File error: {0}")]
    FileError(std::io::Error),

    #[error("Unsupported compression method: {0}")]
    UnsupportedCompression(String),

    #[error("The UEMODEL was missing magic bytes")]
    NoMagicBytes,

    #[error("The UEMODEL contained multiple LODs or LOD-level headers")]
    MultipleLODs(),

    #[error("No {0} found in mesh data")]
    MissingMeshData(String),
}
