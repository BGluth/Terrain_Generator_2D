use palette::{Gradient, LinSrgb};
use rand::Rng;

#[derive(Copy, Clone, Debug)]
struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour { r, g, b }
    }
}

impl From<Colour> for image::Rgb<u8> {
    fn from(val: Colour) -> Self {
        image::Rgb([val.r, val.g, val.b])
    }
}

impl From<Colour> for palette::rgb::Rgb {
    fn from(val: Colour) -> Self {
        palette::rgb::Rgb::from_components((val.r.into(), val.g.into(), val.b.into()))
    }
}

impl From<palette::rgb::Rgb> for Colour {
    fn from(v: palette::rgb::Rgb) -> Self {
        Self::from_rgb_f32(v.red, v.green, v.blue)
    }
}

impl From<LinSrgb> for Colour {
    fn from(v: LinSrgb) -> Self {
        Self::from_rgb_f32(v.red, v.green, v.blue)
    }
}

impl Colour {
    /// WARNING: Potential data loss from the conversions
    fn from_rgb_f32(r: f32, g: f32, b: f32) -> Self {
        Colour::new(
            (r * 255_f32) as u8,
            (g * 255_f32) as u8,
            (b * 255_f32) as u8,
        )
    }
}

pub fn generate_terrain(width: usize, height: usize) -> image::DynamicImage {
    fn gen_colour_table() -> [Colour; 256] {
        fn lerp_between_colours_in_range(
            a: Colour,
            b: Colour,
            start_idx: usize,
            end_idx: usize,
            colour_lookup: &mut [Colour],
        ) {
            let (pal_col_a, pal_col_b): (palette::rgb::Rgb, palette::rgb::Rgb) =
                (a.into(), b.into());

            let col_gradient =
                Gradient::new(vec![LinSrgb::from(pal_col_a), LinSrgb::from(pal_col_b)]);
            let num_steps = (end_idx - start_idx) + 1;

            for (table_idx_offset, col) in col_gradient.take(num_steps).enumerate() {
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
        let darker_grass = Colour::new(60, 201, 91);
        let trees = Colour::new(7, 145, 37);
        let light_trees = Colour::new(104, 154, 113);
        let mountain = Colour::new(191, 191, 191);
        let snow = Colour::new(242, 242, 242);

        lerp_between_colours_in_range(dark_blue, dark_blue, 0, 68, &mut colour_lookup); // Ocean
        lerp_between_colours_in_range(dark_blue, light_blue, 69, 138, &mut colour_lookup); // Ocean
        lerp_between_colours_in_range(light_blue, sand, 139, 140, &mut colour_lookup); // Initial sand
        lerp_between_colours_in_range(sand, lighter_sand, 141, 144, &mut colour_lookup); // Later sand
        lerp_between_colours_in_range(lighter_sand, beach_rocks, 145, 147, &mut colour_lookup); // Beach Rocks
        lerp_between_colours_in_range(beach_rocks, grass, 148, 149, &mut colour_lookup); // Grass
        lerp_between_colours_in_range(grass, darker_grass, 150, 166, &mut colour_lookup); // More Grass
        lerp_between_colours_in_range(darker_grass, light_trees, 167, 179, &mut colour_lookup); // Trees
        lerp_between_colours_in_range(light_trees, trees, 180, 199, &mut colour_lookup); // More Trees
        lerp_between_colours_in_range(trees, light_trees, 200, 210, &mut colour_lookup); // Light Trees
        lerp_between_colours_in_range(light_trees, mountain, 211, 224, &mut colour_lookup); // Bare Mountain
        lerp_between_colours_in_range(mountain, snow, 225, 242, &mut colour_lookup); // Snow
        lerp_between_colours_in_range(snow, snow, 243, 255, &mut colour_lookup); // More Snow

        colour_lookup
    }

    let mut image_buf: image::RgbImage = image::ImageBuffer::new(width as u32, height as u32);
    let colour_lookup = gen_colour_table();

    let mut rng = rand::thread_rng();
    let seed = rng.gen();

    let noise = simdnoise::NoiseBuilder::fbm_2d(width, height)
        .with_seed(seed)
        .with_freq(0.0006)
        .with_lacunarity(2.2)
        .with_octaves(64)
        .with_gain(0.65)
        .generate_scaled(0.0, 255.0);

    for (x, y, pix) in image_buf.enumerate_pixels_mut() {
        let idx = (x + y * width as u32) as usize;
        let pixel_val = noise[idx] as usize;
        let colour = colour_lookup[pixel_val];
        *pix = colour.into();
    }

    image::DynamicImage::ImageRgb8(image_buf)
}
