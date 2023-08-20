use screenshots::{Compression, Screen};

// Struct for pixels on the screen.
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/* TODO: 
 *  - grab screens from point
 *  - attempt to allow a square of 2 monitors
 */
fn screenshot(xy1: Option<Point>, xy2: Option<Point>, screen: Option<usize>) -> Vec<screenshots::Image> {
    
    if screen.is_none() {
        let screens = Screen::all().unwrap();
        let mut images = Vec::new();
        for capture in screens {
            images.push(capture.capture().unwrap());
        }
        images
    }
    else {
        let display_info = screenshots::DisplayInfo::all().unwrap()[screen.unwrap()];

        let capture = screenshots::Screen::new(&display_info);
        let xy1 = xy1.unwrap();
        let xy2 = xy2.unwrap();

        let width: u32 = (xy2.x - xy1.x).abs() as u32;
        let height: u32 = (xy2.y - xy1.y).abs() as u32;
        
        let corner = Point {
            x: if xy1.x < xy2.x {xy1.x} else {xy2.x},
            y: if xy1.y < xy2.y {xy1.y} else {xy2.y},
        };
        vec![capture.capture_area(corner.x, corner.y, width, height).unwrap()]
    }
}

fn read() -> i32 {
    1i32
}

pub fn run(compression: Option<String>, 
           screen: Option<usize>, 
           bounds: (Option<Point>, Option<Point>)) 
            -> Vec<Vec<u8>> {

    let images = screenshot(bounds.0, bounds.1, screen);
    let mut compressed_buffers = Vec::new();
    for image in images {
        match &*compression.clone().unwrap_or_default().to_lowercase() {
            "Best"  => {
                compressed_buffers.push(image.to_png(Compression::Best).unwrap());
            },
            "Fast"  => {
                compressed_buffers.push(image.to_png(Compression::Fast).unwrap());
            },

            _   => {
                compressed_buffers.push(image.to_png(Compression::Default).unwrap());
            }
        }
    }
    compressed_buffers
}
