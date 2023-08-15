use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Argumemts {
    #[clap(subcommand)]
    /// Mode
    pub mode: Mode,
}

#[derive(Debug, Subcommand)]
pub enum Mode{
    /// Launch app in gui mode
    Gui,
    /// Get image
    Image(ImageCommand),
    /// Get text form image
    Text(TextCommand)
}

#[derive(Debug, Args)]
pub struct ImageCommand {
    #[clap(subcommand)]
    pub command: ImageSubcommand
}

#[derive(Debug, Subcommand)]
pub enum ImageSubcommand {
    Output(OutputToPath),
    CopyToClipboard,
    Time(Seconds)
}

#[derive(Debug, Args)]
pub struct TextCommand {
    #[clap(subcommand)]
    pub command: TextSubcommand
}

#[derive(Debug, Subcommand)]
pub enum TextSubcommand {
    Output(OutputToPath),
    Verbose,
    CopyToClipboard,
    Time(Seconds)
}

#[derive(Debug, Args)]
pub struct OutputToPath{
    path: String
}

#[derive(Debug, Args)]
pub struct Seconds{
    seconds: u8
}
