#[derive(clap::Parser)]
pub struct Config {
    #[clap(long)]
    pub database_url: String,
}

