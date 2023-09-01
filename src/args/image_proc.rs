use screenshots::{Compression, Screen};


// Struct for pixels on the screen.
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub struct Rectangle {
    pub tl: Point,
    pub br: Point,
}

impl Point {
    fn to_local(&self, screen: Screen) -> Point {
        let x = self.x - screen.display_info.x;
        let y = self.y - screen.display_info.y;
        Point {
            x: if x > screen.display_info.width as i32 { screen.display_info.width as i32 } else if x > 0 { x } else { 0 },
            y: if y > screen.display_info.height as i32 { screen.display_info.height as i32 } else if y > 0 { x } else { 0 },
        }
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
        let mut local_tl  = global_tl.to_local(screen_tl);                  // Top left in local coordinates
        let mut local_br  = global_br.to_local(screen_br);                  // Bottom right in local coordinates

        if screen_tl.display_info.id != screen_br.display_info.id {
            /* TODO:
             *  TODO: - top to bottom
             *  - test left to right
             *  - test right to left
             *  TODO: - cover middle screen
             */

            let mut images = Vec::<screenshots::Image>::new();
            for screen in Screen::all().unwrap() {
                if do_overlap(Point { x: screen.display_info.x, 
                                      y: screen.display_info.y }, &global_tl, 
                              Point { x: screen.display_info.x + screen.display_info.width as i32,
                                      y: screen.display_info.y + screen.display_info.height as i32}, &global_br) {
                    local_tl = global_tl.to_local(screen);
                    local_br = global_tl.to_local(screen);
                    println!("local_tl: {} {}\nlocal_br: {} {}", local_tl.x, local_tl.y, local_br.x, local_br.y);
                    let cap = screen.capture_area(local_tl.x, local_tl.y, 
                                                  (local_br.x - local_tl.x) as u32, 
                                                  (local_br.y - local_tl.y) as u32).unwrap();
                    images.push(cap);
                }
            }
            images


            /*let mut local_br_tl   = Point { x: 0, y: 0 };
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
                local_tl_br.x = screen_tl.display_info.width as i32;
            } 
            else {
                local_tl_br.x = global_br.x - screen_tl.display_info.x;
            }
            if global_tl.y > screen_tl.display_info.y + screen_tl.display_info.height as i32 {
                local_tl_br.y = screen_tl.display_info.height as i32;
            } 
            else {
                local_tl_br.y = screen_tl.display_info.height as i32;
            }




            println!("local_tl: {} {}\nlocal_br: {} {}", local_tl.x, local_tl.y, local_br.x, local_br.y);
            vec![screen_tl.capture_area(local_tl.x,    
                                        local_tl.y,     
                                        (local_tl_br.x - local_tl_br.x) as u32, 
                                        (local_tl_br.y - local_tl_br.y) as u32).unwrap(),
                                        screen_br.capture_area(local_br_tl.x, 
                                                               local_br_tl.y,  
                                                               (local_br.x - local_br_tl.x) as u32,    
                                                               (local_br.y - local_br_tl.y) as u32).unwrap()]
        */}

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

    fn do_overlap(l1: Point, r1: &Point, l2: Point, r2: &Point) -> bool {
        // if rectangle has area 0, no overlap
        if l1.x == r1.x || l1.y == r1.y || r2.x == l2.x || l2.y == r2.y {
            return false;
        }
        // If one rectangle is on left side of other
        if l1.x > r2.x || l2.x > r1.x {
            return false;
        }

        // If one rectangle is above other
        if r1.y > l2.y || r2.y > l1.y {
            return false;
        }

        return true;
    }
