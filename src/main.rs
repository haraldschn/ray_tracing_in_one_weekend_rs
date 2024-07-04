pub mod color;
pub mod vec3;

use color::{color, write_color};
use indicatif::ProgressBar;

fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let bar = ProgressBar::new(image_height);
    for j in 0..image_height {
        bar.inc(1);
        for i in 0..image_width {
            let pixel_color = color(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.,
            );
            write_color(&pixel_color);
        }
    }
}
