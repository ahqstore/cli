use console::style;
use strsim::levenshtein;

pub fn main_help() -> String {
  let cli = style("AHQ Store CLI").blue().bold();
  let ver = style(format!("v{}", env!("CARGO_PKG_VERSION"))).dim();

  let usage = style("Usage:").green().bold();

  let cmds = style(&"Commands:").green().bold();
  let help = style("help").cyan().bold();

  let new = style("new").cyan().bold();
  let interactive = style("[interactive]").magenta();

  let build = style("build").cyan().bold();

  let opt = style("Options:").yellow().bold();
  let force = style("--force, -f").cyan().bold();

  let env = style("Variables:").yellow().bold();

  let app_id = style("APP_ID").cyan().bold();
  let gh_token = style("GH_TOKEN").cyan().bold();
  let r_id = style("RELEASE_ID").cyan().bold();

  let optional = style("(Optional)").yellow().bold();

  format!(
    r#"{cli} {ver}
Community Repository Application Management handled with ease!

{usage}
  ahqstore (command) [options]
{cmds}
  {help}
    Shows the help menu

  {new}
    {interactive} Generates a new AHQ Store Application Project!
    {opt}
      {force} Override Existing contents if .ahqstore dir isn't empty

  {build}
    Build the ahqstore config file
    {env}
      {r_id} GitHub Release Id
      {gh_token} GitHub Token (generally passed by CI)

      {app_id} {optional} Application Id (required if your config has more than 1 appIds)"#
  )
}

pub fn not_found(name: &str) -> String {
  let commands = vec!["help", "new", "build"];

  // Find the closest match
  let best_match = commands
    .iter()
    .map(|cmd| (cmd, levenshtein(name, cmd)))
    .min_by_key(|&(_, distance)| distance);

  let cmd_err = style("Command not found:").red().bold();
  let mut output = format!("{cmd_err} {name}\n");

  if let Some((suggestion, dist)) = best_match {
    if dist <= 2 {
      let tip = style("Did you mean?").yellow().bold();
      let sug = style(suggestion).cyan().bold();
      output.push_str(&format!("\n{tip} {sug}\n"));
    }
  }

  let help_tip = style("Tip:").green().bold();
  let astore = style("ahqstore help").cyan().bold();
  output.push_str(&format!(
    "\n{help_tip} Write {astore} to view the help menu"
  ));

  output
}
