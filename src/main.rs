use arg_parsing::ParsedArgs;
use chrono::{Datelike, Local, Timelike};
use clap::Parser;

mod arg_parsing;

fn main() -> anyhow::Result<()> {
    let p_args = ParsedArgs::parse();

    let curr_time_date = Local::now();
    let image_path = format!(
        "{:?}/{}-{}-{}-{}-{}-{}{}",
        p_args.out_dir,
        curr_time_date.year(),
        curr_time_date.month(),
        curr_time_date.day(),
        curr_time_date.hour(),
        curr_time_date.minute(),
        curr_time_date.second(),
        ".png"
    );

    let terrain_image =
        terrain_generator_2d::generate_terrain(p_args.dimensions.x, p_args.dimensions.y);
    terrain_image.save(&image_path).unwrap();

    if p_args.output_file_name {
        println!("{}", &image_path);
    }

    Ok(())
}
