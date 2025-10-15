use crate::config;
use crate::i18n::{tr, tr_with};
use crate::utils::{print_err, print_ok};
use chrono::Local;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

use crate::utils::icons::ERR;
#[cfg(not(target_os = "windows"))]
use flate2::{Compression, write::GzEncoder};
#[cfg(not(target_os = "windows"))]
use tar::Builder as TarBuilder;

pub fn handle_backup(_conn: &rusqlite::Connection, compress: bool) -> io::Result<()> {
    let fail_mess = &tr("app.config.load_failed");
    let conf = config::load_or_init().expect(format!("{}{}", ERR, &fail_mess).as_str());
    let db_path = PathBuf::from(&conf.database);

    if !db_path.exists() {
        print_err(&tr("backup.error.not_found"));
        return Ok(());
    }

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let backup_dir = db_path.parent().unwrap().join("backups");
    fs::create_dir_all(&backup_dir)?;

    if compress {
        // --- compressed backup
        #[cfg(target_os = "windows")]
        {
            let zip_name = format!("librius_backup_{}.zip", timestamp);
            let zip_path = backup_dir.join(&zip_name);
            compress_zip(&db_path, &zip_path)?;
            print_ok(
                &tr_with(
                    "backup.ok.compressed",
                    &[("path", &zip_path.display().to_string())],
                ),
                true,
            );
        }

        #[cfg(not(target_os = "windows"))]
        {
            let tar_name = format!("librius_backup_{}.tar.gz", timestamp);
            let tar_path = backup_dir.join(&tar_name);
            compress_tar_gz(&db_path, &tar_path)?;
            print_ok(
                &tr_with(
                    "backup.ok.compressed",
                    &[("path", &tar_path.display().to_string())],
                ),
                true,
            );
        }
    } else {
        // --- plain copy
        let backup_name = format!("librius_backup_{}.sqlite", timestamp);
        let backup_path = backup_dir.join(&backup_name);
        fs::copy(&db_path, &backup_path)?;
        print_ok(
            &tr_with(
                "backup.ok.plain",
                &[("path", &backup_path.display().to_string())],
            ),
            true,
        );
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn compress_zip(src: &PathBuf, dest_zip: &PathBuf) -> io::Result<()> {
    let file = File::create(dest_zip)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::<()>::default().compression_method(CompressionMethod::Deflated);

    let db_name = src.file_name().unwrap().to_string_lossy();
    zip.start_file(&db_name, options)?;
    let data = fs::read(src)?;
    zip.write_all(&data)?;
    zip.finish()?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn compress_tar_gz(src: &PathBuf, dest_tar_gz: &PathBuf) -> io::Result<()> {
    let tar_gz = File::create(dest_tar_gz)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = TarBuilder::new(enc);
    tar.append_path(src)?;
    tar.finish()?;
    Ok(())
}
