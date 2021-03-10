use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.4.0", author = "Heyin <heyin.cjy@alibaba-inc.com>")]
pub struct CommandOpts {
    #[clap(short, long)]
    pub host: Option<String>,
    #[clap(short, long)]
    pub db: Option<String>,
    #[clap(short, long)]
    pub username: Option<String>,
    #[clap(short)]
    pub commands: Vec<String>,
    #[clap(long)]
    pub count: Option<u64>,
    #[clap(short, long)]
    pub password: Option<String>,
    #[clap(short, long, default_value = "16")]
    pub threads: usize,
    #[clap(short, long, default_value = "3600")]
    pub secs: u64,
    #[clap(long, default_value = "3306")]
    pub port: u16,
}


