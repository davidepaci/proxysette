use std::path::PathBuf;
use std::{env, fs};
mod conversions;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure that the program receives exactly one argument
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    // Get the filename from the command line
    let mut file_path = PathBuf::from(&args[1]);

    // Convert to absolute if needed
    if !file_path.is_absolute() {
        file_path = env::current_dir().unwrap().join(file_path);
    }

    // Get the file extension
    if let Some(extension) = file_path.extension() {
        if let Some(ext_str) = extension.to_str() {
            println!("File extension: {}", ext_str);
            match ext_str {
                "tap" => {
                    // audiotap conversion
                    conversions::tap::convert(file_path);
                }
                "t64" | "prg" => {
                    // wavprg conversion
                    conversions::t64::convert(file_path);
                }
                "d64" => {
                    // extract prg then convert
                }
                "zip" => {
                    // find first file in order of (.tap, .t64, .prg, .d64) then convert to wav
                    // save file in tmp folder
                    match conversions::zip::extract_first_image(file_path) {
                        Ok((mut path, extension)) => {
                            println!("File extracted successfully with extension {}", extension);
                            // convert
                            match extension.as_str() {
                                "tap" => conversions::tap::convert(path.clone()),
                                "t64" | "prg" => conversions::t64::convert(path.clone()),
                                "d64" => {}
                                _ => {}
                            }
                            // delete tmp file + folder
                            path.pop();
                            println!("Removing");
                            fs::remove_dir_all(path.clone()).expect("TODO: panic message");
                        }
                        Err(err) => {
                            println!("Error: {}", err);
                            // Handle the error condition here
                        }
                    }
                }
                _ => {
                    eprintln!("Error: Unsupported file extension '{}'", ext_str);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Error: Invalid UTF-8 in file extension");
            std::process::exit(1);
        }
    } else {
        eprintln!("Error: The file has no extension");
        std::process::exit(1);
    }
}
