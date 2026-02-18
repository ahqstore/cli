use console::style;

pub fn main_help() -> String {
  let cli = style("AHQ Store CLI").blue().bold();

  let usage = style("Usage:").green().bold();

  let cmds = style(&"Commands:").green().bold();
  let help = style("help").cyan().bold();
  let create = style("create").cyan().bold();
  let build = style("build").cyan().bold();
  let upload = style("upload").cyan().bold();

  let opt = style("Options:").yellow().bold();
  let force = style("--force, -f").cyan().bold();

  let env = style("Required ENV:").yellow().bold();
  let app_id = style("APP_ID").cyan().bold();
  let gh_token = style("GH_TOKEN").cyan().bold();
  let r_id = style("RELEASE_ID").cyan().bold();

  let optional = style("(Optional)").yellow().bold();

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

  {upload}
    Builds & Upload ahqstore config file
    {env}
      <-- identical to build -->
    
  {build}
    Build the ahqstore config file
    {env}
      {r_id} GitHub Release Id
      {gh_token} GitHub Token (generally passed by CI)

      {app_id} {optional} Application Id (required if your config has more than 1 appIds)"#
  )
}

pub fn not_found(name: &str) -> String {
  let cmd = style("Command not found:").red().bold();
  let tip = style(&"Tip:").green().bold();
  let astore = style("ahqstore").cyan().bold();

  format!(
    r#"{cmd} {name}

{tip}
  Write {astore} to view the help menu"#
  )
}
