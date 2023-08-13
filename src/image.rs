use screenshots;
use std::num;

// Struct for pixels on the screen.
pub struct Point {
    pub x: u32,
    pub y: u32,
}

fn screenshot(xy1: Point, xy2: Point) -> screenshots::Image {
    let display_info = screenshots::DisplayInfo::all().unwrap()[0];
    let capture = screenshots::Screen::new(&display_info);
    let width: u32 = num::abs(xy2.x - xy1.x);
    let height: u32 = num::abs(xy2.y - xy1.y);
    let corner = Point {
        x: if xy1.x > xy2.x {xy1.x} else {xy2.x},
        y: if xy1.y > xy2.y {xy1.y} else {xy2.y},
    };
    capture.capture_area(corner.x, corner.y, width, height).unwrap()
}

fn main() {
    let xy1 = Point {
        x: 0,
        y: 0,
    };
    let xy2 = Point {
        x: 1080,
        y: 1920,
    };

    let image = screenshot(xy1, xy2);
    let buffer = image.to_png(Compression::Fast).unwrap();
    let compressed_buffer = image.to_png(Compression::Best).unwrap();

}
