extern crate time;
extern crate Terrain_Generator_2D;
extern crate image;

use std::fs::File;
use std::io::Write;

const OUT_DIR: &'static str = "examples/generated_terrain";
const WIDTH: u32 = 1028;
const HEIGHT: u32 = 1920;

fn main()
{
	let curr_time_date = time::now();
	let image_path = format!("{}/{}-{}-{}-{}-{}-{}{}", OUT_DIR, curr_time_date.tm_year, curr_time_date.tm_mon,
		curr_time_date.tm_mday, curr_time_date.tm_hour, curr_time_date.tm_min, curr_time_date.tm_sec, ".png");
	
	let mut image_file = match File::create(&image_path)
	{
		Ok(file) => file,
		Err(err_mesg) =>
		{
			println!("The following error has occured when trying to open the file {}: {}", image_path, err_mesg);
			return
		}
	};
	
	let terrain_image = Terrain_Generator_2D::generate_terrain(WIDTH, HEIGHT);
	terrain_image.save(&mut image_file, image::PNG);
}