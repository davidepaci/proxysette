use std::fs;
// open zip and extract first file
// in order of: .tap, .t64, .prg, .d64
// return file (and extension), else return error
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use zip::read::{ZipArchive, ZipFile};

pub fn extract_first_image(zip_path: PathBuf) -> Result<(PathBuf, String), Error> {
    // Open the ZIP file
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    // Define the file extensions in order of priority
    let file_extensions = [".tap", ".t64", ".prg", ".d64"];

    // Iterate through the defined file extensions
    for extension in &file_extensions {
        // Find the first file with the current extension
        if let Some(index) = (0..archive.len()).find(|i| {
            let file = archive.by_index(*i).unwrap();
            file.name().to_lowercase().ends_with(extension)
        }) {
            // Extract the file's contents
            let mut file = archive.by_index(index).unwrap();
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;

            // Save file into tmp path
            fs::create_dir_all("/tmp/")?;

            if let Some(file_name) = file.enclosed_name() {
                let file_name_string = file_name.to_path_buf();

                let project_root = Path::new(".").canonicalize()?; // Get the canonical path of the project root

                let tmp_folder = project_root.join("tmp"); // Path to the "tmp" folder

                // Create the "converted" folder if it doesn't exist
                if !tmp_folder.exists() {
                    fs::create_dir(&tmp_folder)?;
                    println!("Non esiste");
                }

                let file_path = tmp_folder.join(file_name_string); // Path to the destination file

                println!("Found file extension: {}", extension);

                // Open a file handle
                let mut file = File::create(&file_path)?;

                // Write contents to the file
                file.write_all(&contents)?;

                // Turn path string into PathBuf
                let file_path = PathBuf::from(file_path);

                return Ok((file_path, extension.replace(".", "")));
            }
        }
    }

    return Err(Error::from(ErrorKind::InvalidInput));
}
