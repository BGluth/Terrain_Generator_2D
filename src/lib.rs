extern crate noise;
extern crate image;
extern crate rand;
extern crate palette;

use rand::Rng;
use noise::{Seedable, MultiFractal, NoiseModule};
use palette::{Hsv, Gradient};

use std::path::Path;

#[derive(Copy, Clone, Debug)]
struct Colour
{
	r: u8,
	g: u8,
	b: u8
}

impl Colour
{
	fn new(r: u8, g: u8, b: u8) -> Colour
	{
		Colour
		{
			r: r,
			g: g,
			b: b
		}
	}
}

impl Into<image::Rgb<u8>> for Colour
{
	fn into(self) -> image::Rgb<u8>
	{
		image::Rgb{ data: [self.r, self.g, self.b] }
	}
}

impl Into<palette::Rgb<f32>> for Colour
{
	fn into(self) -> palette::Rgb<f32>
	{
		palette::Rgb::new_u8(self.r, self.g, self.b)
	}
}

impl From<palette::Rgb<f32>> for Colour
{
	fn from(pal_col: palette::Rgb<f32>) -> Self
	{
		// WARNING: Potential data loss from the conversions
		Colour
		{
			r: (pal_col.red * 255 as f32) as u8,
			g: (pal_col.green * 255 as f32) as u8,
			b: (pal_col.blue * 255 as f32) as u8
		}
	}
}

pub fn generate_terrain(width: u32, height: u32) -> image::DynamicImage
{
	fn gen_colour_table() -> [Colour; 256]
	{
		fn lerp_between_colours_in_range(a: Colour, b: Colour, start_idx: usize, end_idx: usize, colour_lookup: &mut [Colour])
		{
			let (pal_col_a, pal_col_b): (palette::Rgb, palette::Rgb) = (a.into(), b.into());
			
			let col_gradient = Gradient::new(vec![pal_col_a, pal_col_b]);
			let num_steps = (end_idx - start_idx) + 1;
			
			for (table_idx_offset, col) in col_gradient.take(num_steps).enumerate()
			{
				//println!("Gradient Colour: {:#?}", &col);
				colour_lookup[start_idx + table_idx_offset] = Colour::from(col);
			}
		}
		
		let mut colour_lookup = [Colour::new(0, 0, 0); 256];
		
		let dark_blue = Colour::new(5, 35, 84);
		let light_blue = Colour::new(24, 108, 244);
		let sand = Colour::new(244, 252, 138);
		let lighter_sand = Colour::new(251, 255, 186);
		let beach_rocks = Colour::new(165, 165, 165);
		let grass = Colour::new(74, 224, 106);
		let trees = Colour::new(7, 145, 37);
		let light_trees = Colour::new(104, 154, 113);
		let mountain = Colour::new(191, 191, 191);
		let snow = Colour::new(242, 242, 242);
		
		lerp_between_colours_in_range(dark_blue, light_blue, 0, 64, &mut colour_lookup); // Ocean
		lerp_between_colours_in_range(light_blue, sand, 65, 66, &mut colour_lookup); // Initial sand
		lerp_between_colours_in_range(sand, lighter_sand, 67, 68, &mut colour_lookup); // Later sand
		lerp_between_colours_in_range(lighter_sand, beach_rocks, 69, 71, &mut colour_lookup); // Beach Rocks
		lerp_between_colours_in_range(beach_rocks, grass, 72, 74, &mut colour_lookup); // Grass
		lerp_between_colours_in_range(beach_rocks, grass, 75, 79, &mut colour_lookup); // More Grass
		lerp_between_colours_in_range(grass, trees, 80, 84, &mut colour_lookup); // Trees
		lerp_between_colours_in_range(grass, trees, 85, 110, &mut colour_lookup); // More Trees
		lerp_between_colours_in_range(trees, light_trees, 111, 126, &mut colour_lookup); // Light Trees
		lerp_between_colours_in_range(mountain, mountain, 127, 152, &mut colour_lookup); // Bare Mountain
		lerp_between_colours_in_range(mountain, snow, 153, 157, &mut colour_lookup); // Snow
		lerp_between_colours_in_range(snow, snow, 157, 255, &mut colour_lookup); // More Snow
		
		colour_lookup
	}
	
	let mut image_buf: image::RgbImage = image::ImageBuffer::new(width, height);
	let colour_lookup = gen_colour_table();
	
	let mut rng = rand::thread_rng();
	let seed = rng.gen_range(0, usize::max_value());
	
	let noise_generator = noise::Fbm::new().set_seed(seed).set_frequency(0.01).set_octaves(6).set_lacunarity(2.0).set_persistence(0.5);
	
	for (x, y, pix) in image_buf.enumerate_pixels_mut()
	{
		let pixel_val = (noise_generator.get([x as f32, y as f32]) * 255.0).abs() as usize;
		let colour = colour_lookup[pixel_val];
		*pix = colour.into();
	}
	
	image::ImageRgb8(image_buf)
}