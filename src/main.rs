use std::fs::File;
use std::io::{self};
use std::path::Path;
use zip::{CompressionMethod, ZipWriter};
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    println!("Enter the name of the output ZIP file (you don't need to type .zip):");
    let mut output_zip_file = String::new();
    io::stdin().read_line(&mut output_zip_file)?;

    println!("Enter the path to the directory to be zipped:");
    let mut input_directory = String::new();
    io::stdin().read_line(&mut input_directory)?;

    let output_zip_file = format!("{}.zip", output_zip_file.trim());
    let input_directory = input_directory.trim();

    let file = File::create(output_zip_file.clone())?;
    let mut zip = ZipWriter::new(file);

    for entry in WalkDir::new(input_directory).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let relative_path = path.strip_prefix(input_directory).unwrap();
            add_file_to_zip(relative_path, path, &mut zip)?;
        }
    }

    zip.finish()?;

    println!("Zip file created successfully: {}", output_zip_file);
    Ok(())
}

fn add_file_to_zip(
    relative_path: &Path,
    absolute_path: &Path,
    zip: &mut ZipWriter<File>,
) -> io::Result<()> {
    let options = zip::write::FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755);

    let mut file = File::open(absolute_path)?;
    zip.start_file(relative_path.to_string_lossy().into_owned(), options)?;
    io::copy(&mut file, zip)?;

    Ok(())
}
