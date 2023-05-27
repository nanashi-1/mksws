use std::path::Path;

use clap::Parser;
use rocket::{
    catch, catchers,
    fs::{FileServer, Options},
    launch,
    response::content::RawHtml,
    Config, Request,
};

mod arguments;
mod template;

#[catch(404)]
fn not_found(req: &Request) -> RawHtml<String> {
    let uri_string = req.uri().to_string();

    if Path::new(&format!(".{uri_string}")).is_dir() {
        return RawHtml(
            template::FileList::try_from(req.uri().to_string())
                .unwrap()
                .to_string(),
        );
    }

    RawHtml(template::FileNotFound::new(req.uri().to_string()).to_string())
}

#[launch]
fn rocket() -> _ {
    let args = arguments::Arguments::parse();
    let config = Config {
        address: args.ip_address.parse().unwrap(),
        port: args.port,
        ..Default::default()
    };

    rocket::custom(config)
        .mount(
            "/",
            FileServer::new(
                ".",
                Options::DotFiles | Options::Index | Options::NormalizeDirs,
            ),
        )
        .register("/", catchers![not_found])
}
