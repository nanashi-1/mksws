use clap::Parser;

#[derive(Parser)]
#[command(
    author = "nanashi-1",
    version = "1.0",
    about = "Make a static web server."
)]
pub struct Arguments {
    #[arg(
        short = 'i',
        long = "ip_address",
        default_value_t = String::from("0.0.0.0"),
        help = "ip address to listen to"
    )]
    pub ip_address: String,

    #[arg(
        short = 'p',
        long = "port",
        default_value_t = 8181,
        help = "port to listen to"
    )]
    pub port: u16,
}
