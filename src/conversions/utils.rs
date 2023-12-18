use id3::{Error, ErrorKind, Frame, Tag, TagLike, Version};
use std::path::{Path, PathBuf};
use std::{fs, io};

/*pub fn get_file_name_from_path(path: PathBuf) -> Option<PathBuf> {
    path.file_stem()
        .map(|f| PathBuf::from(f))
}*/

// usage:
/*
match save_to_converted_folder(filename, content) {
        Ok(()) => println!("File saved successfully!"),
        Err(e) => eprintln!("Error saving the file: {}", e),
    }
*/
// Function to save a file to a "converted" folder in the root project directory
/*pub fn save_to_converted_folder(file_name: &str, content: &[u8]) -> std::io::Result<()> {
    let project_root = Path::new(".").canonicalize()?; // Get the canonical path of the project root

    let converted_folder = project_root.join("converted"); // Path to the "converted" folder

    // Create the "converted" folder if it doesn't exist
    if !converted_folder.exists() {
        fs::create_dir(&converted_folder)?;
    }

    let destination_file_path = converted_folder.join(file_name); // Path to the destination file

    let mut file = File::create(&destination_file_path)?; // Create or open the file

    file.write_all(content)?; // Write content to the file

    Ok(())
}*/

pub fn get_destination_path(file_stem: &str) -> Result<String, io::Error> {
    let project_root = Path::new(".").canonicalize()?; // Get the canonical path of the project root

    let converted_folder = project_root.join("converted"); // Path to the "converted" folder

    // Create the "converted" folder if it doesn't exist
    if !converted_folder.exists() {
        fs::create_dir(&converted_folder)?;
    }

    let destination_filename = format!("{}.wav", file_stem); // Append ".wav" to the filename

    let destination_file_path = converted_folder.join(destination_filename); // Path to the destination file

    let destination_file_path_string = destination_file_path.to_string_lossy().to_string();

    Ok(destination_file_path_string)
}

pub fn sign_wav(destination_file_path_string: &str) -> Result<(), Box<dyn std::error::Error>> {
    // convert destination path string to Path
    let destination_file_path = PathBuf::from(destination_file_path_string);
    // Open the WAV file
    let mut tag: id3::Tag = match Tag::read_from_path(destination_file_path.clone()) {
        Ok(tag) => tag,
        Err(_) => Tag::new(),
    };

    tag.set_artist("Proxysette 0.1.0"); // get semver automatically in the future

    // Set the album the hard way.
    tag.add_frame(id3::Frame::text("TALB", "album"));

    tag.write_to_path(destination_file_path.clone(), Version::Id3v24)?;
    Ok(())
}

pub fn strip_path_prefix(string: String) -> String {
    string[4..].to_string()
}