use std::{collections::HashSet, panic, sync::LazyLock};

use console::{style, Term};
use strsim::levenshtein;
use tokio::runtime::Builder;

mod help;

pub mod structs;
mod update;

// Commands
mod keygen;
mod new;

pub static TERMINAL: LazyLock<Term> = LazyLock::new(Term::stdout);

#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub fn start(args: Vec<String>, _gh: bool) {
  rustls::crypto::aws_lc_rs::default_provider()
    .install_default()
    .expect("Unable to install aws-lc-rs");

  panic::set_hook(Box::new(|err| {
    let msg = if let Some(x) = err.payload_as_str() {
      x
    } else {
      "Error couldn't be serialized"
    };

    let err = style("An Error Occured!").red().bold();

    println!("-----------------");
    println!("{err}");
    println!();
    println!("{msg}");
    println!("-----------------");
  }));

  Builder::new_current_thread()
    .enable_all()
    .build()
    .expect("Unable to build runtime")
    .block_on(async move {
      update::check_updates().await;

      match args.get(0) {
        Some(cmd) => match cmd as &str {
          "keygen" => {
            let cmd = ArgParser { flags: [] };

            let Some(_) = cmd.parse(args.get(1..).unwrap_or_default()) else {
              return;
            };

            keygen::keygen();
          }
          "new" => {
            let cmd = ArgParser {
              flags: [Flag {
                aliases: &["--force", "-f"],
                id: 0,
              }],
            };

            let Some(parsed) = cmd.parse(args.get(1..).unwrap_or_default()) else {
              return;
            };

            let force = parsed.get(&0).is_some();

            new::generate(force).await;
          }
          "help" => println!("{}", help::main_help()),
          a => println!("{}", help::not_found(a)),
        },
        _ => println!("{}", help::main_help()),
      }
    });
}

struct ArgParser<const N: usize> {
  pub flags: [Flag; N],
}

impl<const N: usize> ArgParser<N> {
  pub fn parse(&self, args: &[String]) -> Option<HashSet<usize>> {
    let mut out: HashSet<usize> = HashSet::with_capacity(N);

    for arg in args {
      if let Some(x) = self
        .flags
        .iter()
        .find(|x| x.aliases.contains(&(&arg as &str)))
      {
        out.insert(x.id);
      } else {
        println!("{} {arg}", style("Unknown argument:").red().bold());

        let flags = self
          .flags
          .iter()
          .map(|x| x.aliases)
          .flatten()
          .map(|x| *x as &str)
          .collect::<Box<[&str]>>();

        if let Some((arg, dist)) = flags
          .iter()
          .filter(|x| x.len() >= 3)
          .map(|flag| (flag, levenshtein(arg, flag)))
          .min_by_key(|&(_, distance)| distance)
        {
          if dist <= 2 {
            let tip = style("Did you mean?").yellow().bold();
            let sug = style(arg).cyan().bold();

            println!("\n{tip} {sug}");
          } else {
            let poss = style("All Flags:").yellow().bold();

            println!("\n{poss} {}", flags.join(", "));
          }
        } else {
          println!("\nThis command takes no arguments!");
        }

        return None;
      }
    }

    Some(out)
  }
}

struct Flag {
  id: usize,
  aliases: &'static [&'static str],
}
