use screenshots::{Compression, Screen};
use std::fs;

// Struct for pixels on the screen.
pub struct Point {
    pub x: i32,
    pub y: i32,
}


fn screenshot(xy1: Option<Point>, xy2: Option<Point>, screen: Option<i32>) -> screenshots::Image {
    let display_info = screenshots::DisplayInfo::all().unwrap();
    
    if (screen.is_none()) {
    }
    let capture = screenshots::Screen::new(&display_info);
    let xy1 = xy1.unwrap();
    let xy2 = xy2.unwrap();

    
    let width: u32 = (xy2.x - xy1.x).abs() as u32;
    let height: u32 = (xy2.y - xy1.y).abs() as u32;
    let corner = Point {
        x: if xy1.x < xy2.x {xy1.x} else {xy2.x},
        y: if xy1.y < xy2.y {xy1.y} else {xy2.y},
    };
    capture.capture_area(corner.x, corner.y, width, height).unwrap()
}

fn read() -> i32 {
    1i32
}

pub fn run(compression: Option<String>, 
           screen: Option<i32>,
           bounds: (Option<Point>, Option<Point>)) -> Result<Vec<u8, Global>, EncodingError> {

    let image = screenshot(bounds.0, bounds.1, screen);
    let mut compressed_buffer: Result<Vec<u8, Global>, EncodingError>;
    match &**compression.unwrap_or(*"").to_lowercase() {
        "Best"  => {
            let compressed_buffer = image.to_png(Compression::Best).unwrap();
        },
        "Rle"  => {
            let compressed_buffer = image.to_png(Compression::Rle).unwrap();
        },
        "Huffman"  => {
            let compressed_buffer = image.to_png(Compression::Huffman).unwrap();
        },
        "Fast"  => {
            let compressed_buffer = image.to_png(Compression::Fast).unwrap();
        },

        _   => {
            let compressed_buffer = image.to_png(Compression::Default).unwrap();
        }
    }
    compressed_buffer
}
