use screenshots::Compression;
use std::fs;

// Struct for pixels on the screen.
pub struct Point {
    pub x: i32,
    pub y: i32,
}

fn screenshot(xy1: Point, xy2: Point) -> screenshots::Image {
    let display_info = screenshots::DisplayInfo::all().unwrap()[0];
    let capture = screenshots::Screen::new(&display_info);
    let width: u32 = (xy2.x - xy1.x).abs() as u32;
    let height: u32 = (xy2.y - xy1.y).abs() as u32;
    let corner = Point {
        x: if xy1.x < xy2.x {xy1.x} else {xy2.x},
        y: if xy1.y < xy2.y {xy1.y} else {xy2.y},
    };
    capture.capture_area(corner.x, corner.y, width, height).unwrap()
}

pub fn run() {
    let xy1 = Point {
        x: 0,
        y: 0,
    };
    let xy2 = Point {
        x: 1920,
        y: 1080,
    };

    let image = screenshot(xy1, xy2);
    let buffer = image.to_png(Compression::Fast).unwrap();
    let compressed_buffer = image.to_png(Compression::Best).unwrap();
    
    fs::write(format!("target-2.png"), buffer).unwrap();

    fs::write(
        format!("target-2-compressed.png"), 
        compressed_buffer
    ).unwrap();
}
