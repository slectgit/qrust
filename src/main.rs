extern crate image;
extern crate nsvg;

use qrcode_generator::QrCodeEcc;
use std::env;
use std::process;

/*
  Argument:
    "text"
    "size"
    "save png path"
    "foreground"
    "background"
*/

fn main() {
  // get argument
  let args: Vec<String> = env::args().collect();

  if args.iter().len() != 6 {
    println!("give five arguments");
    println!("`text` `size` `save location path` `foreground` `background`");
    process::exit(0);
  }

  // string to int
  let size = match args[2].parse::<i32>() {
    Ok(i) => i,
    Err(_e) => -1,
  };

  // check invalid size
  if size < 128 || 8192 < size{
    println!("plese give valid size");
    println!("size between 128 to 8192");
    process::exit(0);
  }

  // get qr code string
  let result: String =
    qrcode_generator::to_svg_to_string(args[1].to_string(), QrCodeEcc::Low, size.try_into().unwrap(), &args[4], &args[5], None::<&str>)
      .unwrap();

  let svg = nsvg::parse_str(&result, nsvg::Units::Pixel, 96.0).unwrap();

  let image = svg.rasterize(1.0).unwrap();

  let save_path = args[3].to_string();
  let (width, height) = image.dimensions();

  image::save_buffer(
    save_path,
    &image.into_raw(),
    width,
    height,
    image::ColorType::Rgba8,
  )
  .expect("Failed to save png.");
  
}
