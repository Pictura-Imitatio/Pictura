use screenshots::{Compression, Screen};

// Struct for pixels on the screen.
pub struct Point {
    pub x: i32,
    pub y: i32,
}

fn global_to_local(point: &Point, screen: Screen) -> Point {
    Point {
        x: point.x - screen.display_info.x,
        y: point.y - screen.display_info.y,
    }
}

/* TODO: 
 *  DONE: - grab screens from point
 *  TODO: - attempt to allow a square of 2 monitors
 *  DONE: - make point order and position ambigious (i.e. (bl,tr), or (tl,br), etc)
 */
fn screenshot(global_coordinates: (Option<Point>, Option<Point>)) -> Vec<screenshots::Image> {
    if global_coordinates.0.is_none() && global_coordinates.1.is_none() {
    //if screen.is_none() {
        let screens = Screen::all().unwrap();
        let mut images = Vec::new();
        for capture in screens {
            let cap = capture.capture().unwrap();
            images.push(cap); 
            println!("{capture:?}");
        }
        images
    }

    /* TODO:
     *  TODO: - what about the middle monitor
     *      -- think about comparing the global coordinates of all screens to the range of tlbr
     * 
     *  */
    else {
        let global_coordinates = (global_coordinates.0.unwrap(), global_coordinates.1.unwrap());
        let global_tl = Point { // The top left of the rectangle created by the global coordinates
            x: if global_coordinates.0.x < global_coordinates.1.x { global_coordinates.0.x } else { global_coordinates.1.x },
            y: if global_coordinates.0.y < global_coordinates.1.y { global_coordinates.0.y } else { global_coordinates.1.y },
        };

        let global_br = Point {  // The bottom right of the rectangle created by the global coordinates
            x: if global_coordinates.0.x < global_coordinates.1.x { global_coordinates.1.x } else { global_coordinates.0.x },
            y: if global_coordinates.0.y < global_coordinates.1.y { global_coordinates.1.y } else { global_coordinates.0.y }
        };


        let screen_tl = Screen::from_point(global_tl.x, global_tl.y).unwrap();   // Screen that contains the top left coodinate
        let screen_br = Screen::from_point(global_br.x, global_br.y).unwrap();   // Screen that contains the bottom right coordinate
        let local_tl  = global_to_local(&global_tl, screen_tl);                  // Top left in local coordinates
        let local_br  = global_to_local(&global_br, screen_br);                  // Bottom right in local coordinates

        if screen_tl.display_info.id != screen_br.display_info.id {
        /* TODO:
         *  - top to bottom
         *  - test left to right
         *  - test right to left
         *  - cover middle screen
         */
            let local_tl_width  = screen_tl.display_info.width - local_tl.x as u32;
            let local_tl_height = (local_tl.y - global_to_local(&global_br, screen_tl).y) as u32;

            let local_br_tl     = Point { // the local top left corner of the rectangle on the br screen
                x: 0,
                y: global_to_local(&global_tl, screen_br).y,
            };
            let local_br_width  = local_br.x as u32;
            let local_br_height = (local_br.y - local_br_tl.y) as u32;

            vec![screen_tl.capture_area(local_tl.x,    local_tl.y,     local_tl_width, local_tl_height).unwrap(),
                 screen_br.capture_area(local_br_tl.x, local_br_tl.y,  local_br_width, local_br_height).unwrap()]
        }

        else {
            /* DONE:
             *  DONE: - convert global to local
             */
            let width:u32  = (local_br.x - local_tl.x) as u32;
            let height:u32 = (local_br.y - local_tl.y) as u32;
            println!("{} {}/{} {}\n{width}/{height}\n{screen_tl:?}", local_tl.x, local_tl.y, local_br.x, local_br.y);
            vec![screen_tl.capture_area(local_tl.x, local_tl.y, width, height).unwrap()]
        }
        
    }

}

pub fn run(compression: Option<String>,
           bounds: (Option<Point>, Option<Point>)) 
            -> Vec<Vec<u8>> {

    let images = screenshot((bounds.0, bounds.1));
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
