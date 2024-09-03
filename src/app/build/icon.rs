use crate::app::ERR;
use image::{load_from_memory_with_format as load_img, ImageFormat};
use std::fs;
use std::process;

pub fn get_icon(uid: &str) -> Vec<u8> {
  let base_img = format!("./.ahqstore/images/{uid}/icon.png");

  let Ok(icon) = fs::read(&base_img) else {
    ERR.println(&"Unable to read icon file!");
    process::exit(1);
  };

  validate_png(&icon);

  icon
}

pub fn get_images(uid: &str) -> Vec<Vec<u8>> {
  let base_img = format!("./.ahqstore/images/{uid}");

  let Ok(image_dir) = fs::read_dir(&base_img) else {
    ERR.println(&"Unable to read image dir!");
    process::exit(1);
  };

  let mut entries = image_dir
    .map(|res| res.expect("Unable to unwrap dir entry").path())
    .filter(|f| !f.ends_with("icon.png"))
    .map(|res| fs::read(res).expect("Unable to read bytes"))
    .map(|img| {
      validate_png(&img);
      return img;
    })
    .collect::<Vec<_>>();

  entries.truncate(10);

  entries
}

pub fn validate_png(data: &Vec<u8>) {
  let Ok(_) = load_img(&data, ImageFormat::Png) else {
    ERR.println(&"Invalid PNG");
    process::exit(1);
  };
}
