use std::{fs, path::PathBuf, str::FromStr};

use anyhow::anyhow;
use arg_parsing::ParsedArgs;
use chrono::{Datelike, Local, Timelike};
use clap::Parser;
use log::info;

mod arg_parsing;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let p_args = ParsedArgs::parse();
    let curr_time_date = Local::now();

    let image_path = format!(
        "{}-{}-{}-{}-{}-{}.{}",
        curr_time_date.year(),
        curr_time_date.month(),
        curr_time_date.day(),
        curr_time_date.hour(),
        curr_time_date.minute(),
        curr_time_date.second(),
        "png"
    );

    let out_dir_path = PathBuf::from_str(&p_args.out_dir)?;
    let out_path = out_dir_path.join(image_path);

    info!("Generating {:?}...", out_path);

    let terrain_image =
        terrain_generator_2d::generate_terrain(p_args.dimensions.x, p_args.dimensions.y);

    fs::create_dir_all(&out_dir_path)?;
    terrain_image.save(&out_path)?;

    if p_args.output_file_name {
        println!(
            "{}",
            out_path.to_str().ok_or(anyhow!(
                "Converting the generated image path to a string to send to stdout"
            ))?
        );
    }

    Ok(())
}
