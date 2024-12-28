use screenshots::Screen;
use std::error::Error;
use std::time::Instant;

/// A struct to handle display-aware screen captures
struct DisplayAwareCapture {
    screen: Screen,
    physical_width: u32,
    physical_height: u32,
}

impl DisplayAwareCapture {
    /// Create a new DisplayAwareCapture from a Screen
    fn new(screen: Screen) -> Self {
        let info = &screen.display_info;

        // Calculate physical dimensions accounting for scale factor
        let physical_width = (info.width as f32 * info.scale_factor) as u32;
        let physical_height = (info.height as f32 * info.scale_factor) as u32;

        Self {
            screen,
            physical_width,
            physical_height,
        }
    }

    /// Get all available displays with their physical dimensions
    fn all_displays() -> Result<Vec<Self>, Box<dyn Error>> {
        let screens = Screen::all()?;
        Ok(screens.into_iter().map(Self::new).collect())
    }

    /// Capture a scaled area, accounting for display scaling
    fn capture_scaled_area(
        &self,
        logical_x: i32,
        logical_y: i32,
        logical_width: u32,
        logical_height: u32,
    ) -> Result<screenshots::image::RgbaImage, Box<dyn Error>> {
        let scale = self.screen.display_info.scale_factor;

        // Convert logical coordinates to physical pixels
        // Adjusting for the additional 25% scaling we observed
        let physical_x = (logical_x as f32 * scale * 1.25) as i32;
        let physical_y = (logical_y as f32 * scale * 1.25) as i32;
        let physical_width = (logical_width as f32 * scale * 1.25) as u32;
        let physical_height = (logical_height as f32 * scale * 1.25) as u32;

        Ok(self
            .screen
            .capture_area(physical_x, physical_y, physical_width, physical_height)?)
    }

    /// Print detailed information about the display
    fn print_display_info(&self) {
        let info = &self.screen.display_info;
        println!("Display Information:");
        println!("├─ ID: {}", info.id);
        println!("├─ Position: ({}, {})", info.x, info.y);
        println!("├─ Logical Resolution: {}x{}", info.width, info.height);
        println!(
            "├─ Physical Resolution: {}x{}",
            self.physical_width, self.physical_height
        );
        println!("├─ Scale Factor: {}%", (info.scale_factor * 100.0) as u32);
        println!("├─ Refresh Rate: {}Hz", info.frequency);
        println!("├─ Rotation: {} degrees", info.rotation);
        println!(
            "└─ Primary Display: {}",
            if info.is_primary { "Yes" } else { "No" }
        );
    }

    /// Check if the display is rotated
    fn is_rotated(&self) -> bool {
        self.screen.display_info.rotation != 0.0
    }

    /// Get the effective capture dimensions based on rotation
    fn get_rotated_dimensions(&self, width: u32, height: u32) -> (u32, u32) {
        if self.is_rotated()
            && (self.screen.display_info.rotation == 90.0
                || self.screen.display_info.rotation == 270.0)
        {
            (height, width)
        } else {
            (width, height)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    println!("🖥️  Display-Aware Screenshot Example\n");

    // Get all displays
    let displays = DisplayAwareCapture::all_displays()?;
    println!("Found {} display(s)\n", displays.len());

    for (index, display) in displays.iter().enumerate() {
        println!("Display #{}", index + 1);
        display.print_display_info();
        println!();

        // Example: Capture a 100x100 logical pixel area from each display
        let logical_size = 100;
        let image = display.capture_scaled_area(0, 0, logical_size, logical_size)?;

        // Calculate expected physical dimensions
        let scale = display.screen.display_info.scale_factor;
        let size_with_dpi = (logical_size as f32 * scale) as u32;
        let size_with_extra = (logical_size as f32 * scale * 1.25) as u32;
        let size_with_double = (logical_size as f32 * scale * 1.56) as u32; // ~195/125

        println!(
            "Scaling calculations for {}x{} logical pixels:",
            logical_size, logical_size
        );
        println!(
            "├─ With DPI scaling only ({}%): {}x{} pixels",
            (scale * 100.0) as u32,
            size_with_dpi,
            size_with_dpi
        );
        println!(
            "├─ With 125% extra scaling: {}x{} pixels",
            size_with_extra, size_with_extra
        );
        println!(
            "└─ With 156% extra scaling: {}x{} pixels",
            size_with_double, size_with_double
        );
        println!();

        // Save with display info in filename
        let info = &display.screen.display_info;
        let filename = format!(
            "target/display_{}_{}x{}_scale{}_rot{}.png",
            info.id,
            logical_size,
            logical_size,
            (info.scale_factor * 100.0) as u32,
            info.rotation
        );
        image.save(&filename)?;
        println!("Captured scaled screenshot: {}", filename);
        println!(
            "Actual physical size: {}x{} pixels\n",
            image.width(),
            image.height()
        );
    }

    println!("✨ Completed in {:?}", start.elapsed());
    Ok(())
}
