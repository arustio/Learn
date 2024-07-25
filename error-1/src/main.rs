use std::fs::File;
use std::io;

fn open_file() -> File {
    // let f: File = File::open("hello.txt").unwrap_or_else(|error: io::Error| {
    //     if(error.kind() == io::ErrorKind::NotFound){
    //         File::create("hello.txt").unwrap_or_else(|error: io::Error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
    // f

    File::open("hello.txt").expect("Failed to open hello.txt")
}

fn open_file_result() -> Result<File, io::Error> {
    let f = File::open("hello.txt")?;
    Ok(f)
}

fn main() {
    let result = open_file();
}
