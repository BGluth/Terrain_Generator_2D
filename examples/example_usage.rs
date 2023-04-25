use chrono::prelude::*;
use chrono::Local;

const OUT_DIR: &str = "examples/generated_terrain";
const WIDTH: usize = 10000;
const HEIGHT: usize = 10000;

fn main() {
    let curr_time_date = Local::now();
    let image_path = format!(
        "{}/{}-{}-{}-{}-{}-{}{}",
        OUT_DIR,
        curr_time_date.year(),
        curr_time_date.month(),
        curr_time_date.day(),
        curr_time_date.hour(),
        curr_time_date.minute(),
        curr_time_date.second(),
        ".png"
    );

    let terrain_image = terrain_generator_2d::generate_terrain(WIDTH, HEIGHT);
    terrain_image.save(image_path).unwrap();
}
