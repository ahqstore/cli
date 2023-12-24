use chalk_rs::Chalk;

pub fn main_help() -> String {
  let mut chalk = Chalk::new();

  let cli = chalk.blue().bold().string(&"AHQ Store CLI");

  let usage = chalk.green().bold().string(&"Usage:");

  let cmds = chalk.green().bold().string(&"Commands:");
  let help = chalk.cyan().bold().string(&"help");
  let create = chalk.cyan().bold().string(&"create");
  let build = chalk.cyan().bold().string(&"build");

  let opt = chalk.yellow().bold().string(&"Options:");
  let force = chalk.cyan().bold().string(&"--force, -f");

  let env = chalk.yellow().bold().string(&"Required ENV:");
  let gh_token = chalk.cyan().bold().string(&"GH_TOKEN");
  let gh_repo = chalk.cyan().bold().string(&"GITHUB_REPOSITORY");
  let r_id = chalk.cyan().bold().string(&"RELEASE_ID");

  format!(
    r#"{cli}
Ship your apps to the ahq store quickly and efficiently

{usage}
  ahqstore (command) [options]
{cmds}
  {help}
    Shows the help menu
  {create}
    Generates AHQ Store config files required to ship your apps
    {opt}
      {force} Override Existing contents if .ahqstore dir isn't empty
  {build}
    Build the ahqstore config file
    {env}
      {r_id} GitHub Actions Release Id
      {gh_token} GitHub Token
      {gh_repo} GitHub owner & repo name, eg ahqstore/app
                        (Equivalent to GITHUB_REPOSITORY in GitHub actions)"#
  )
}

pub fn not_found(name: &str) -> String {
  let mut chalk = Chalk::new();

  let cmd = chalk.red().bold().string(&"Command not found:");
  let tip = chalk.green().bold().string(&"Tip:");
  let astore = chalk.cyan().bold().string(&"ahqstore");

  format!(
    r#"{cmd} {name}

{tip}
  Write {astore} to view the help menu"#
  )
}
