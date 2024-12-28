use anyhow::Result;
use screenshots::Screen;
use std::path::Path;
use std::time::Instant;

struct ScreenCapture {
    screen: Screen,
}

impl ScreenCapture {
    fn from_screen(screen: Screen) -> Self {
        Self { screen }
    }

    fn from_point(x: i32, y: i32) -> Result<Self> {
        Ok(Self {
            screen: Screen::from_point(x, y)?,
        })
    }

    fn capture_full(&self) -> Result<Vec<u8>> {
        let image = self.screen.capture()?;
        Ok(image.into_raw())
    }

    fn capture_area(&self, x: i32, y: i32, width: u32, height: u32) -> Result<Vec<u8>> {
        let image = self.screen.capture_area(x, y, width, height)?;
        Ok(image.into_raw())
    }

    fn save_capture(&self, image: Vec<u8>, path: impl AsRef<Path>) -> Result<()> {
        std::fs::write(path, image)?;
        Ok(())
    }

    fn display_id(&self) -> u32 {
        self.screen.display_info.id
    }
}

fn main() {
    let start = Instant::now();

    if let Ok(screens) = Screen::all() {
        for screen in screens {
            println!("Capturing screen: {:?}", screen);
            let capturer = ScreenCapture::from_screen(screen);

            // Capture and save full screen
            if let Ok(image) = capturer.capture_full() {
                let _ =
                    capturer.save_capture(image, format!("target/{}.png", capturer.display_id()));
            }

            // Capture and save area
            if let Ok(image) = capturer.capture_area(300, 300, 300, 300) {
                let _ =
                    capturer.save_capture(image, format!("target/{}-2.png", capturer.display_id()));
            }
        }
    }

    // Capture from specific point
    if let Ok(capturer) = ScreenCapture::from_point(100, 100) {
        println!(
            "Capturing from point with display id: {}",
            capturer.display_id()
        );
        if let Ok(image) = capturer.capture_area(300, 300, 300, 300) {
            let _ = capturer.save_capture(image, "target/capture_display_with_point.png");
        }
    }

    println!("Time elapsed: {:?}", start.elapsed());
}
