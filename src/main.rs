use screenshots;

fn main() {
    let displayinfo = screenshots::DisplayInfo::all().unwrap()[0];
    let capture = screenshots::Screen::new(&displayinfo);
    capture.capture().unwrap();
}
