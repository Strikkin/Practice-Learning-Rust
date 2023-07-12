use std::fs::File;

pub fn get_file(f: String) -> Result<File, std::io::Error>{
    return File::options()
    .read(true)
    .append(true)
    .create(true)
    .open(&f);
}
