use anyhow::*;
use gpui::Pixels;
use image::DynamicImage;
use xcap::Monitor;

pub struct ScreenPixel(Pixels);

impl From<Pixels> for ScreenPixel {
    fn from(pixels: Pixels) -> Self {
        let scale_factor = Monitor::from_point(0, 0).unwrap().scale_factor();

        ScreenPixel(pixels.ceil() * scale_factor)
    }
}

impl From<ScreenPixel> for u32 {
    fn from(pixel: ScreenPixel) -> u32 {
        pixel.0.into()
    }
}

pub fn screenshot(
    x: ScreenPixel,
    y: ScreenPixel,
    width: ScreenPixel,
    height: ScreenPixel,
) -> Result<DynamicImage, Error> {
    let monitor = Monitor::from_point(0, 0).unwrap();
    let image_buffer = monitor.capture_image().unwrap();
    let macos_topbar_height = 37 * monitor.scale_factor() as u32;

    let image = DynamicImage::from(image_buffer);
    let x = x.into();
    let y: u32 = y.into();
    let width = width.into();
    let height = height.into();

    if width == 0 || height == 0 {
        bail!("Invalid region size when taking screenshot");
    }

    let image = image.crop_imm(x, y + macos_topbar_height, width, height);

    // uncomment for debug
    // image.save("assets/test_screenshot.png").unwrap();

    Ok(image)
}
