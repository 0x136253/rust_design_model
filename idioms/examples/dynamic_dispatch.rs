use std::io;
use std::fs;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = "-";

    // 它们必须活的比 `readable`长, 因此先声明:
    let (mut stdin_read, mut file_read);

    // We need to ascribe the type to get dynamic dispatch.
    let readable: &mut dyn io::Read = if arg == "-" {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(arg)?;
        &mut file_read
    };

    // Read from `readable` here.
    Ok(())
}
