use std::fmt::Display;
use std::str::{FromStr, Split};

use anyhow::{bail, Context};
use clap::Parser;

const DEFAULT_DIM: Vector2 = Vector2 { x: 2000, y: 2000 };
const DEFAULT_WRITE_DIR: &str = "generated_images/";

#[derive(Clone, Debug, Default)]
pub(crate) struct Vector2 {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl FromStr for Vector2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_intern(s)
            .with_context(|| format!("Unable to parse a vector from the string \"{}\"", s))
            .map_err(|e| format!("{e:#}"))
    }
}

impl Vector2 {
    fn from_str_intern(s: &str) -> anyhow::Result<Self> {
        let mut v_iter = s.split('x');

        let x = Self::parse_split_iter(&mut v_iter)?;
        let y = Self::parse_split_iter(&mut v_iter)?;

        if v_iter.count() == 0 {
            bail!("Unexpected extra data used to create vector");
        }

        Ok(Vector2 { x, y })
    }

    fn parse_split_iter(iter: &mut Split<char>) -> anyhow::Result<usize> {
        iter.next()
            .with_context(|| "Failed to parse a string as a vector scalar")
            .and_then(|s| {
                s.parse()
                    .with_context(|| format!("Failed to parse \"{}\" as an unsigned integer", s))
            })
    }
}

#[derive(Debug, Parser)]
pub(crate) struct ParsedArgs {
    /// The dimensions of the image to generate.
    #[clap(short, long, default_value_t=DEFAULT_DIM)]
    pub(crate) dimensions: Vector2,

    /// The directory to write the generate image to.
    #[clap(short, long, default_value_t=String::from(DEFAULT_WRITE_DIR))]
    pub(crate) out_dir: String,

    /// Whether or not to output the path to the generated file.
    #[clap(short, long, default_value_t = false)]
    pub(crate) output_file_name: bool,
}
