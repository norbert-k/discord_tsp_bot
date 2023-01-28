use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArguments {
    /// Discord BOT Token
    #[arg(short, long)]
    pub token: String,
    /// OAuth2 client id
    #[arg(short, long)]
    pub client_id: String,
    /// Debug mode
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}