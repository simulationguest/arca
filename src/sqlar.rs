use flate2::read::{DeflateDecoder, DeflateEncoder};
use flate2::Compression;
use rusqlite::Connection;
use std::fs::{File, Permissions};
use std::io;
use std::io::Write;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use std::time::{Duration, SystemTime};
use walkdir::WalkDir;

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

pub fn create_archive(dir: &Path, dest: &Path) -> Result<(), Error> {
    if std::fs::exists(dest)? {
        return Err(Error::FileAlreadyExists);
    }

    let mut conn = Connection::open(dest)?;

    let tx = conn.transaction()?;

    tx.execute(
        "CREATE TABLE sqlar(
      name TEXT PRIMARY KEY,
      mode INT,
      mtime INT,
      sz INT,
      data BLOB
    );",
        (),
    )?;

    let mut stmt = tx.prepare(
        "INSERT INTO sqlar (name, mode, mtime, sz, data) VALUES (?1, ?2, ?3, ?4, ZEROBLOB(?5))",
    )?;

    let files = WalkDir::new(dir).into_iter();

    let mut buf: Vec<u8> = Vec::new();

    for entry in files {
        buf.clear();

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

        let data = File::open(entry.path())?;

        let mut compress = DeflateEncoder::new(data, Compression::best());

        std::io::copy(&mut compress, &mut buf)?;

        let _ = stmt.execute((name, permissions, last_modified, size, buf.len()))?;

        let row_id = tx.last_insert_rowid();

        let mut blob =
            tx.blob_open(rusqlite::DatabaseName::Main, "sqlar", "data", row_id, false)?;

        blob.write_all(&buf)?;
    }

    drop(stmt);

    tx.commit()?;

    Ok(())
}

pub fn extract_archive(from: &Path, to: &Path) -> Result<(), Error> {
    let conn = Connection::open(from)?;

    let mut stmt = conn.prepare("SELECT name, mode, mtime, rowid FROM sqlar;")?;
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let name: String = row.get(0)?;
        let mode: u32 = row.get(1)?;
        let mtime: u64 = row.get(2)?;
        let row_id: i64 = row.get(3)?;

        let blob = conn.blob_open(rusqlite::DatabaseName::Main, "sqlar", "data", row_id, true)?;

        let mut decoder = DeflateDecoder::new(blob);

        let name = to.join(Path::new(name.as_str()));

        let mut dir = name.clone();
        dir.pop();
        std::fs::create_dir_all(&dir)?;

        let mut file = File::create_new(name)?;

        file.set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(mtime))?;
        file.set_permissions(Permissions::from_mode(mode))?;

        io::copy(&mut decoder, &mut file)?;
    }

    Ok(())
}
