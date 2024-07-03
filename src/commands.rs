use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn get_bytes_from_path(path: &str) -> Vec<u8> {
    let mut f = File::open(path).expect("Cannot open file");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)
        .expect("Unable to read into file buffer");
    buffer
}

pub fn print(path: &str) {
    let buffer = get_bytes_from_path(path);
    let png = Png::try_from(buffer.as_slice()).unwrap();

    let chunk_types: Vec<String> = png
        .chunks()
        .iter()
        .map(|c| c.chunk_type().to_string())
        .collect();
    println!("The following chunks can be decoded:");
    for chunk in chunk_types {
        println!("{}", chunk);
    }
}

pub fn encode(path: &str, chunk_type: &str, message: &str) {
    let buffer = get_bytes_from_path(path);
    let mut png = Png::try_from(buffer.as_slice()).unwrap();

    let end = png
        .remove_chunk("IEND")
        .expect("Unable to remove end chunk");

    png.append_chunk(Chunk::new(
        ChunkType::from_str(chunk_type).unwrap(),
        message.as_bytes().into(),
    ));
    png.append_chunk(end);

    let write_path = std::path::Path::new(path);
    std::fs::write(write_path, png.as_bytes()).expect("Unable to write to file");
    println!("Message Encoded!");
}

pub fn decode(path: &str, chunk_type: &str) {
    let buffer = get_bytes_from_path(path);
    let png = Png::try_from(buffer.as_slice()).unwrap();

    let target = png
        .chunk_by_type(chunk_type)
        .expect("Unable to locate chunk_type");

    println!("Hidden message is: {}", target.data_as_string().unwrap());
}

pub fn remove(path: &str, chunk_type: &str) {
    let buffer = get_bytes_from_path(path);
    let mut png = Png::try_from(buffer.as_slice()).unwrap();

    png.remove_chunk(chunk_type)
        .expect("Unable to remove chunk");
    let write_path = std::path::Path::new(path);
    std::fs::write(write_path, png.as_bytes()).expect("Unable to write to file");
    println!("Chunk removed!");
}
