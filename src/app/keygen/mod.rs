use std::time::Duration;

use aws_lc_rs::signature::{Ed25519KeyPair, KeyPair};

use base64::{prelude::BASE64_STANDARD, Engine};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm};
use indicatif::ProgressBar;

use crate::app::TERMINAL;

pub fn keygen() {
  let theme = ColorfulTheme::default();

  let bar = ProgressBar::new_spinner();
  bar.enable_steady_tick(Duration::from_millis(30));

  let keypair = Ed25519KeyPair::generate().expect("Ed25519KeyPair could not be generated");

  let pkcs8 = keypair.to_pkcs8().expect("Unable to export to PKCS#8 v2");
  let pubkey = keypair.public_key();

  bar.disable_steady_tick();
  bar.finish_and_clear();

  if !TERMINAL.is_term() {
    panic!("Non Terminal Connections cannot interact with keygen!");
  }

  if Confirm::with_theme(&theme)
    .with_prompt("Ready to view your Private Key? (Ensure no one is watching your screen)")
    .default(false)
    .interact()
    .expect("Response is required")
  {
    TERMINAL
      .clear_last_lines(1000)
      .expect("Unable to clear lines");
    TERMINAL
      .clear_screen()
      .expect("SECURITY ERROR : The Terminal Screen does not support clearing");

    // Print the key with a distinct color to separate it from logs
    TERMINAL
      .write_line(&format!(
        "Keyset has been generated\n\nPKCS#8 Private Key:\n{}\n\nTHIS STRING IS CONFIDENTIAL. TREAT WITH CAUTION\n",
        style(BASE64_STANDARD.encode(pkcs8.as_ref()))
          .yellow()
          .bold()
      ))
      .expect("Could not write private key correctly");

    let _ = Confirm::with_theme(&theme)
      .with_prompt("Press `y` or `n` to clear the screen")
      .interact();

    TERMINAL
      .clear_last_lines(1000)
      .expect("Unable to clear lines");
    TERMINAL
      .clear_screen()
      .expect("SECURITY ERROR : The Terminal Screen does not support clearing");
  }

  println!("{}", style("AHQ Store CLI").underlined());
  println!(
    "{}",
    style("Your signing keys has been successfully generated").green()
  );

  TERMINAL
    .write_line(&format!(
      "\nPublic Key:\n{}",
      style(BASE64_STANDARD.encode(pubkey.as_ref()))
        .yellow()
        .bold()
    ))
    .expect("Could not print Public Key");
}
