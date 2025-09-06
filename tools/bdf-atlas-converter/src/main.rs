mod args;
mod font_convert;
mod font_info;
mod layout;
mod utils;

use crate::args::Args;
use crate::font_convert::cmd_convert_font;
use crate::font_info::cmd_font_info;
use clap::Parser;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    if args.info {
        cmd_font_info(args)
    } else {
        cmd_convert_font(args)
    }
}
