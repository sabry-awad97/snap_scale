use screenshots::{display_info::DisplayInfo, image::RgbaImage, Screen};
use std::time::Instant;

struct ScreenCapture {
    screen: Screen,
}

impl ScreenCapture {
    fn from_screen(screen: Screen) -> Self {
        Self { screen }
    }

    fn from_point(x: i32, y: i32) -> Option<Self> {
        Screen::from_point(x, y).ok().map(|screen| Self { screen })
    }

    fn capture(&self) -> Option<RgbaImage> {
        self.screen.capture().ok()
    }

    fn capture_area(&self, x: i32, y: i32, width: u32, height: u32) -> Option<RgbaImage> {
        self.screen.capture_area(x, y, width, height).ok()
    }

    fn display_info(&self) -> &DisplayInfo {
        &self.screen.display_info
    }
}

fn main() {
    let start = Instant::now();
    let screens = Screen::all().unwrap();

    for screen in screens {
        println!("capturer {screen:?}");
        let capturer = ScreenCapture::from_screen(screen);

        let mut image = capturer.capture().unwrap();
        image
            .save(format!("target/{}.png", capturer.display_info().id))
            .unwrap();

        image = capturer.capture_area(300, 300, 300, 300).unwrap();
        image
            .save(format!("target/{}-2.png", capturer.display_info().id))
            .unwrap();
    }

    let capturer = ScreenCapture::from_point(100, 100).unwrap();
    println!("capturer {:?}", capturer.screen);

    let image = capturer.capture_area(300, 300, 300, 300).unwrap();
    image.save("target/capture_display_with_point.png").unwrap();
    println!("Time elapsed: {:?}", start.elapsed());
}
