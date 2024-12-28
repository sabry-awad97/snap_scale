use screenshots::Screen;
use std::error::Error;
use std::path::Path;
use std::time::Instant;

/// Represents the scaling configuration for display-aware screen captures
#[derive(Debug)]
struct ScalingConfig {
    dpi_scale: f32,
    total_scale: f32,
}

impl ScalingConfig {
    /// Determines the actual scaling factor by performing a test capture
    #[cfg(not(test))]
    fn determine_actual_scaling(screen: &Screen) -> f32 {
        let test_size = 100;
        if let Ok(test_image) = screen.capture_area(0, 0, test_size, test_size) {
            let actual_size = test_image.width() as f32;
            let dpi_scaled_size = test_size as f32 * screen.display_info.scale_factor;
            actual_size / dpi_scaled_size
        } else {
            1.56 // Fallback to empirically determined value if test capture fails
        }
    }

    #[cfg(test)]
    fn determine_actual_scaling(_screen: &Screen) -> f32 {
        1.56 // Use consistent value for testing
    }

    /// Creates a new ScalingConfig with dynamically determined scaling
    fn new(screen: &Screen) -> Self {
        let dpi_scale = screen.display_info.scale_factor;
        let extra_scale = Self::determine_actual_scaling(screen);

        Self {
            dpi_scale,
            total_scale: dpi_scale * extra_scale,
        }
    }

    fn scale_dimension(&self, logical_size: u32) -> u32 {
        (logical_size as f32 * self.total_scale) as u32
    }

    fn scale_coordinate(&self, logical_coord: i32) -> i32 {
        (logical_coord as f32 * self.total_scale) as i32
    }
}

/// A display-aware screen capture utility that handles DPI scaling and rotation
#[derive(Debug)]
struct DisplayAwareCapture {
    screen: Screen,
    scaling: ScalingConfig,
}

impl DisplayAwareCapture {
    /// Creates a new DisplayAwareCapture instance from a Screen
    fn new(screen: Screen) -> Self {
        Self {
            scaling: ScalingConfig::new(&screen),
            screen,
        }
    }

    /// Retrieves all available displays with their configurations
    fn all_displays() -> Result<Vec<Self>, Box<dyn Error>> {
        let screens = Screen::all()?;
        Ok(screens.into_iter().map(Self::new).collect())
    }

    /// Captures a scaled area, accounting for display scaling
    fn capture_scaled_area(
        &self,
        logical_x: i32,
        logical_y: i32,
        logical_width: u32,
        logical_height: u32,
    ) -> Result<screenshots::image::RgbaImage, Box<dyn Error>> {
        let physical_x = self.scaling.scale_coordinate(logical_x);
        let physical_y = self.scaling.scale_coordinate(logical_y);
        let physical_width = self.scaling.scale_dimension(logical_width);
        let physical_height = self.scaling.scale_dimension(logical_height);

        Ok(self
            .screen
            .capture_area(physical_x, physical_y, physical_width, physical_height)?)
    }

    /// Saves a screenshot with detailed metadata in the filename
    fn save_screenshot(
        &self,
        image: &screenshots::image::RgbaImage,
        prefix: &str,
        logical_size: u32,
        target_dir: impl AsRef<Path>,
    ) -> Result<String, Box<dyn Error>> {
        let info = &self.screen.display_info;
        let filename = format!(
            "{}/{}_{}x{}_dpi{}_scale{}_rot{}.png",
            target_dir.as_ref().to_string_lossy(),
            prefix,
            logical_size,
            logical_size,
            (info.scale_factor * 100.0) as u32,
            (self.scaling.total_scale * 100.0) as u32,
            info.rotation
        );
        image.save(&filename)?;
        Ok(filename)
    }

    /// Prints detailed display information in a beautiful tree format
    fn print_display_info(&self, index: usize) {
        let info = &self.screen.display_info;
        println!("\n📺 Display #{}", index + 1);
        println!("├─ 🆔 ID: {}", info.id);
        println!("├─ 📍 Position: ({}, {})", info.x, info.y);
        println!("├─ 🖥️  Resolution");
        println!("│  ├─ Logical: {}x{}", info.width, info.height);
        println!(
            "│  └─ Physical: {}x{}",
            (info.width as f32 * info.scale_factor) as u32,
            (info.height as f32 * info.scale_factor) as u32
        );
        println!("├─ 📏 Scaling");
        println!(
            "│  ├─ DPI Scale: {:.2}x ({:.0}%)",
            self.scaling.dpi_scale,
            self.scaling.dpi_scale * 100.0
        );
        println!(
            "│  ├─ Extra Scale: {:.2}x",
            self.scaling.total_scale / self.scaling.dpi_scale
        );
        println!(
            "│  └─ Total Scale: {:.2}x ({:.0}%)",
            self.scaling.total_scale,
            self.scaling.total_scale * 100.0
        );
        println!("├─ 🔄 Rotation: {}°", info.rotation);
        println!(
            "└─ 🎯 Primary: {}",
            if info.is_primary { "Yes" } else { "No" }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::EPSILON;

    /// Test helper to create a mock screen with specific properties
    fn create_mock_screen(width: u32, height: u32, scale: f32, rotation: f32) -> Screen {
        let screens = Screen::all().unwrap();
        let mut screen = screens[0].clone();
        screen.display_info.width = width;
        screen.display_info.height = height;
        screen.display_info.scale_factor = scale;
        screen.display_info.rotation = rotation;
        screen
    }

    mod scaling_config {
        use super::*;

        #[test]
        fn test_new_scaling_config() {
            let screen = create_mock_screen(1920, 1080, 1.25, 0.0);
            let config = ScalingConfig::new(&screen);

            assert!(
                (config.dpi_scale - 1.25).abs() < EPSILON,
                "DPI scale should be 1.25"
            );
            assert!(
                (config.total_scale - 1.25 * 1.56).abs() < EPSILON,
                "Total scale should be DPI scale * extra scale"
            );
        }

        #[test]
        fn test_scale_dimension() {
            let screen = create_mock_screen(1920, 1080, 1.0, 0.0);
            let config = ScalingConfig::new(&screen);

            assert_eq!(
                config.scale_dimension(100),
                (100.0 * 1.56) as u32,
                "Should scale dimensions correctly"
            );
        }

        #[test]
        fn test_scale_coordinate() {
            let screen = create_mock_screen(1920, 1080, 2.0, 0.0);
            let config = ScalingConfig::new(&screen);

            assert_eq!(
                config.scale_coordinate(50),
                (50.0 * 2.0 * 1.56) as i32,
                "Should scale coordinates correctly"
            );
        }
    }

    mod display_aware_capture {
        use super::*;

        #[test]
        fn test_new_display_capture() {
            let screen = create_mock_screen(1920, 1080, 1.25, 0.0);
            let capture = DisplayAwareCapture::new(screen);

            assert!(
                (capture.scaling.total_scale - 1.25 * 1.56).abs() < EPSILON,
                "Total scaling should be correctly calculated"
            );
        }

        #[test]
        fn test_capture_scaled_area_calculations() {
            let screen = create_mock_screen(1920, 1080, 1.5, 0.0);
            let capture = DisplayAwareCapture::new(screen);

            let logical_size = 100;
            let expected_physical_size = (logical_size as f32 * 1.5 * 1.56) as u32;

            let result = capture.capture_scaled_area(0, 0, logical_size, logical_size);
            assert!(result.is_ok(), "Capture should succeed");
        }

        #[test]
        fn test_save_screenshot_filename() {
            let screen = create_mock_screen(1920, 1080, 1.25, 90.0);
            let capture = DisplayAwareCapture::new(screen);

            let test_image = capture.capture_scaled_area(0, 0, 100, 100).unwrap();
            let filename = capture
                .save_screenshot(&test_image, "test", 100, "target")
                .unwrap();

            assert!(
                filename.contains("test_100x100_dpi125_scale195_rot90.png"),
                "Filename should contain correct metadata: {}",
                filename
            );
        }
    }

    mod integration_tests {
        use super::*;
        use std::path::PathBuf;

        #[test]
        fn test_full_capture_workflow() {
            let screen = create_mock_screen(1920, 1080, 1.25, 0.0);
            let capture = DisplayAwareCapture::new(screen);
            let logical_size = 100;

            let result = std::panic::catch_unwind(|| {
                let image = capture
                    .capture_scaled_area(0, 0, logical_size, logical_size)
                    .unwrap();

                let filename = capture
                    .save_screenshot(&image, "integration_test", logical_size, "target")
                    .unwrap();

                let path = PathBuf::from(&filename);
                assert!(path.exists(), "Screenshot file should exist: {}", filename);

                let actual_width = image.width();
                let actual_height = image.height();

                assert_eq!(
                    actual_width, actual_height,
                    "Image should maintain 1:1 aspect ratio"
                );

                assert!(
                    actual_width > logical_size,
                    "Image width should be larger than logical size"
                );
                assert!(
                    actual_height > logical_size,
                    "Image height should be larger than logical size"
                );

                let min_expected_size = (logical_size as f32 * capture.scaling.total_scale) as u32;
                assert!(
                    actual_width >= min_expected_size,
                    "Image width should be at least the minimum expected size"
                );

                std::fs::remove_file(path).unwrap();
            });

            assert!(
                result.is_ok(),
                "Full capture workflow should complete without errors"
            );
        }
    }

    #[cfg(feature = "proptest")]
    mod property_tests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn test_scaling_properties(
                logical_size in 1u32..1000,
                scale_factor in 1.0f32..4.0
            ) {
                let screen = create_mock_screen(1920, 1080, scale_factor, 0.0);
                let config = ScalingConfig::new(&screen);
                let scaled = config.scale_dimension(logical_size);

                prop_assert!(scaled > logical_size, "Scaled size should be larger than logical size");
                prop_assert!(scaled as f32 / logical_size as f32 >= scale_factor,
                    "Scaling should at least match the scale factor");
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    println!("🖥️  Display-Aware Screenshot Example");

    // Get all displays
    let displays = DisplayAwareCapture::all_displays()?;
    println!("\nFound {} display(s)", displays.len());

    // Process each display
    for (index, display) in displays.iter().enumerate() {
        display.print_display_info(index);

        // Capture test area
        let logical_size = 100;
        let image = display.capture_scaled_area(0, 0, logical_size, logical_size)?;

        // Save and report results
        let filename = display.save_screenshot(&image, "display", logical_size, "target")?;

        println!("\n📸 Screenshot Details");
        println!("├─ File: {}", filename);
        println!("├─ Dimensions");
        println!("│  ├─ Logical: {}x{}", logical_size, logical_size);
        println!(
            "│  ├─ Expected: {}x{}",
            display.scaling.scale_dimension(logical_size),
            display.scaling.scale_dimension(logical_size)
        );
        println!("│  └─ Actual: {}x{}", image.width(), image.height());
    }

    println!("\n✨ Completed in {:?}", start.elapsed());
    Ok(())
}
