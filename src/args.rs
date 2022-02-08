use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub user: String,

    #[clap(short, long)]
    pub key: String,

    #[clap(short, long, default_value_t = 14400)]
    pub refresh: i64
}
