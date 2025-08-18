use std::fs::File; // Import File struct for handling files
use std::io::ErrorKind; // Import ErrorKind to check the type of I/O error

fn main() {
    // Try to open "hello.txt". This returns a Result<File, Error>.
    let f: Result<File, std::io::Error> = File::open("hello.txt");

    // Use pattern matching to handle the Result
    let _f: File = match f {
        // Case 1: File was successfully opened → just return the file
        Ok(file) => file,

        // Case 2: Failed to open file → handle the error
        Err(error) => match error.kind() {
            // Sub-case: The file was not found
            ErrorKind::NotFound => match File::create("hello.txt") {
                // Successfully created a new file
                Ok(fc) => fc,
                // Failed to create the file → panic and stop program
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },

            // Other types of errors → panic with the error info
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Another way: using unwrap_or_else with closures instead of match
    let _f: File = File::open("hello.txt").unwrap_or_else(|error| {
        // If file not found, try to create it
        if error.kind() == ErrorKind::NotFound {
            return File::create("hello.txt").unwrap_or_else(|error| {
                // If creation fails → panic
                panic!("Problem creating the file: {:?}", error);
            });
        } else {
            // If error is something else → panic
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // At this point, f is guaranteed to be a valid File handle
}
