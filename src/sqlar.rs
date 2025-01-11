use rusqlite::Connection;
use std::fs::File;
use std::io;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::time::SystemTime;
use walkdir::WalkDir;

fn create_table() -> &'static str {
    "CREATE TABLE sqlar(
      name TEXT PRIMARY KEY,
      mode INT,
      mtime INT,
      sz INT,
      data BLOB
    );"
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("file already exists")]
    FileAlreadyExists,

    #[error("walkdir error: {0}")]
    WalkDir(#[from] walkdir::Error),

    #[error("failed to convert string")]
    FailedToConvertString,
}

pub fn create_archive(dir: &str, dest: &str) -> Result<(), Error> {
    if std::fs::exists(dest)? {
        return Err(Error::FileAlreadyExists);
    }

    let mut conn = Connection::open(dest)?;
    conn.execute(create_table(), ())?;

    let tx = conn.transaction()?;

    let mut stmt = tx.prepare(
        "INSERT INTO sqlar (name, mode, mtime, sz, data) VALUES (?1, ?2, ?3, ?4, ZEROBLOB(?5))",
    )?;

    let files = WalkDir::new(dir).into_iter();

    for entry in files {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        let name = entry
            .path()
            .as_os_str()
            .to_str()
            .ok_or(Error::FailedToConvertString)?;

        let metadata = entry.metadata()?;

        let last_modified = metadata
            .modified()?
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let permissions = metadata.permissions().mode();

        let size = metadata.size();

        let _ = stmt.execute((name, permissions, last_modified, size, size))?;

        let row_id = tx.last_insert_rowid();

        let mut blob =
            tx.blob_open(rusqlite::DatabaseName::Main, "sqlar", "data", row_id, false)?;

        let mut data = File::open(entry.path())?;

        std::io::copy(&mut data, &mut blob)?;
    }

    drop(stmt);

    tx.commit()?;

    Ok(())
}
