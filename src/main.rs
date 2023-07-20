use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::thread;
use std::time::Duration;

use termion::color;

fn monitor_file_changes(file_path: &str) {
    // Initial file state
    let mut previous_contents = String::new();

    loop {
        // Read the current file contents
        let mut current_contents = String::new();
        if let Ok(mut file) = File::open(file_path) {
            file.read_to_string(&mut current_contents).unwrap();
        } else {
            eprintln!("Failed to open file: {}", file_path);
        }

        // Compare current and previous contents
        if current_contents != previous_contents {
            // Log the file change
            println!(
                "{}File changed: {}{}",
                color::Fg(color::Red),
                file_path,
                color::Fg(color::Reset)
            );

            // Show the changes
            let diff = difference::Changeset::new(&previous_contents, &current_contents, "\n");
            for diff_item in diff.diffs {
                match diff_item {
                    difference::Difference::Same(s) => println!("{}", s),
                    difference::Difference::Add(s) => println!(
                        "{}+ {}{}",
                        color::Fg(color::Green),
                        s,
                        color::Fg(color::Reset)
                    ),
                    difference::Difference::Rem(s) => println!(
                        "{}- {}{}",
                        color::Fg(color::Red),
                        s,
                        color::Fg(color::Reset)
                    ),
                }
            }

            // Update previous contents with current contents
            previous_contents = current_contents;
        }

        // Sleep for 10 minutes
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let file_path = "./file.txt"; // Replace with the actual file path
    monitor_file_changes(file_path);
}
