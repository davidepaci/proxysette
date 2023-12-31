use crate::conversions::utils;
use std::path::PathBuf;
use std::process::Command;

pub fn convert(origin_path: PathBuf) {
    // get file stem from absolute path
    if let Some(file_stem) = origin_path.file_stem() {
        if let Some(stem_str) = file_stem.to_str() {
            println!("File stem: {}", stem_str);
            // get destination path with file stem
            match utils::get_destination_path(stem_str) {
                Ok(s) => {
                    let destination_path = s;
                    // feed origin and destination paths to program
                    let origin_path = origin_path.to_str().unwrap();
                    println!("{} {}", origin_path, destination_path);

                    // prg2wav -i -w [destination_path] [origin_path]
                    let output = Command::new("../../tools/wavprg/prg2wav")
                        .args(&["-i", "-w", &destination_path, origin_path])
                        .output();

                    match output {
                        Ok(output) => {
                            // Handle the output of the command if needed
                            if output.status.success() {
                                println!("Command executed successfully!");
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                println!("Output: {}", stdout);
                                // Sign wav
                                /*match sign_wav(&destination_path) {
                                    Ok(..) => {
                                        println!("WAV signed successfully!");
                                    }
                                    Err(e) => eprintln!("Error: {}", e)
                                }*/
                            } else {
                                let stderr = String::from_utf8_lossy(&output.stderr);
                                eprintln!("Error: {}", stderr);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to execute command: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e); // Handle the error from Err variant
                }
            };
        } else {
            eprintln!("Error: Invalid UTF-8 in file stem");
        }
    } else {
        eprintln!("Error: Unable to extract file stem");
    }
}
