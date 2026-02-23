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

  let keygen = style("keygen").cyan().bold();

  let env = style("Environment Variables:").yellow().bold();

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

  {keygen}
    Generates an ED25519 key-value pair

  {build}
    Build the ahqstore assets into an ahqstore tarball

    • `./.ahqstore/artifacts` should contain the assets.
    • Please do create an `assetmap.json` file mapping the asset names with the ID the CLI should give them.
    • `./.ahqstore/bundle` folder should contain all the screenshots in the following format `<index>.png`. ONLY PNG is supported.
    • `./.ahqstore/bundle` must contain `0.png`
    • OIDC Verification Keys are mandatory

    {env}
      GitHub Release Id"#
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
