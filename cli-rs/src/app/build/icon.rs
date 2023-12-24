use crate::app::ERR;
use base64::engine::{general_purpose::STANDARD, Engine};
use image::{load_from_memory_with_format as load_img, ImageFormat};
use std::fs;
use std::process;

pub fn get_icon() -> String {
  let Ok(icon) = fs::read("./.ahqstore/icon.png") else {
    ERR.println(&"Unable to read icon file!");
    process::exit(1);
  };

  validate_png(&icon);

  STANDARD.encode(&icon)
}

pub fn get_images() -> Vec<String> {
  let Ok(image_dir) = fs::read_dir("./.ahqstore/images") else {
    ERR.println(&"Unable to read image dir!");
    process::exit(1);
  };

  let mut entries = image_dir
    .map(|res| fs::read(res.unwrap().path()).unwrap())
    .collect::<Vec<_>>();

  entries.truncate(10);

  let val = entries
    .iter()
    .map(|img| {
      validate_png(&img);

      return STANDARD.encode(&img);
    })
    .collect::<Vec<_>>();

  val
}

pub fn validate_png(data: &Vec<u8>) {
  let Ok(_) = load_img(&data, ImageFormat::Png) else {
    ERR.println(&"Invalid PNG");
    process::exit(1);
  };
}
