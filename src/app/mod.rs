use tokio::runtime::Builder;

mod help;

mod update;

#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub fn start(args: Vec<String>, _gh: bool) {
  rustls::crypto::aws_lc_rs::default_provider()
    .install_default()
    .expect("Unable to install aws-lc-rs");

  Builder::new_current_thread()
    .enable_all()
    .build()
    .expect("Unable to build runtime")
    .block_on(async move {
      update::check_updates().await;

      if args.len() >= 1 {
        match args[0].as_str() {
          // "create" => create::create(args.len() > 1 && (&args[1] == "--force" || &args[1] == "-f")),
          // "build" => build::build_config(false, gh),
          // "upload" => build::build_config(true, gh),
          "help" => println!("{}", help::main_help()),
          a => println!("{}", help::not_found(a)),
        }
      } else {
        println!("{}", help::main_help());
      }
    });
}
