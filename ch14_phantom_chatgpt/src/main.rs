use std::{marker::PhantomData, fmt::Write};

#[derive(Debug)]
struct File<Mode> {
    name: String,
    mode: PhantomData<Mode>,
}

#[derive(Debug)]
struct ReadOnly;
struct ReadWrite;

impl File<ReadOnly> {
    fn open_read_only(name: String) -> File<ReadOnly> {
        File {
            name,
            mode: PhantomData,
        }
    }

    // Methods specific to read-only files...
}

impl File<ReadWrite> {
    fn open_read_write(name: String) -> File<ReadWrite> {
        File {
            name,
            mode: PhantomData,
        }
    }

    // Methods specific to read-write files...
}

fn main() {
    let file_ro = File::<ReadOnly>::open_read_only("example.txt".to_string());
    let file_rw = File::<ReadWrite>::open_read_write("example.txt".to_string());
    println!("{:?}", file_ro);
    // Compile-time checks prevent misuse.
    // file_ro.write_data("some data"); // This line would not compile
}
