use screenshots::{Compression, Screen};

// Struct for pixels on the screen.
pub struct Point {
    pub x: i32,
    pub y: i32,
}

fn global_to_local(point: &Point, screen: Screen) -> Point {
    let x = point.x - screen.display_info.x;
    let y = point.y - screen.display_info.y;
    Point {
        x: if x > 0 { x } else { 0 },
        y: if y > 0 { y } else { 0 },
    }
}

/* TODO: 
 *  DONE: - grab screens from point
 *  TODO: - attempt to allow a square of 2 monitors
 *  TODO: - attempt to allow a square of 3+ monitors
 *  DONE: - make point order and position ambigious (i.e. (bl,tr), (tl,br), etc)
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

    else {
        /* TODO:
         *  TODO: - what about the middle monitor
         *      -- think about comparing the global coordinates of all screens to the range of tlbr
         */
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
             *  TODO: - top to bottom
             *  - test left to right
             *  - test right to left
             *  TODO: - cover middle screen
             */



            let mut local_br_tl   = Point { x: 0, y: 0 };
                if global_tl.x < screen_br.display_info.x {
                    local_br_tl.x = 0;
                } 
                else {
                    local_br_tl.x = global_tl.x - screen_br.display_info.x;
                }
                if global_tl.y < screen_br.display_info.y {
                    local_br_tl.y = screen_br.display_info.y;
                } 
                else {
                    local_br_tl.y = global_tl.y - screen_br.display_info.y;
                }

            let mut local_tl_br   = Point { x: 0, y: 0 };
                if global_br.x > screen_tl.display_info.x + screen_tl.display_info.width as i32 {
                    local_tl_br.x = screen_tl.display_info.x + screen_tl.display_info.width as i32;
                } 
                else {
                    local_tl_br.x = global_br.x - screen_tl.display_info.x;
                }
                if global_tl.y > screen_tl.display_info.y + screen_tl.display_info.height as i32 {
                    local_tl_br.y = screen_tl.display_info.y + screen_tl.display_info.height as i32;
                } 
                else {
                    local_tl_br.y = global_br.y - screen_tl.display_info.y;
                }
            


            vec![screen_tl.capture_area(local_tl.x,    local_tl.y,     (local_tl_br.y - local_tl_br.y) as u32, (local_tl_br.y - local_tl_br.y) as u32).unwrap(),
                 screen_br.capture_area(local_br_tl.x, local_br_tl.y,  (local_br.x - local_br_tl.x) as u32,    (local_br.y - local_br_tl.y) as u32).unwrap()]
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
