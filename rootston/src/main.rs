extern crate clap;
extern crate ini;
#[macro_use]
extern crate wlroots;

mod config;
mod desktop;
mod input;
mod server;
mod output;
mod view;

const ROOSTON_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const ROOSTON_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const ROOSTON_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

use clap::{App, Arg};

use wlroots::CompositorBuilder;

fn main() {
    let app = App::new("rooston").version(ROOSTON_VERSION)
                                 .author(ROOSTON_AUTHORS)
                                 .about(ROOSTON_DESCRIPTION)
                                 .arg(Arg::with_name("config").short("C")
                                                              .value_name("FILE")
                                                              .help("Path to the configuration \
                                                                     file (default: rooston.ini). \
                                                                     See `rooston.ini.example` \
                                                                     for config file \
                                                                     documentation.")
                                                              .takes_value(true))
                                 .arg(Arg::with_name("command").short("E")
                                                               .value_name("COMMAND")
                                                               .help("Command that will be ran \
                                                                      at startup."));
    let config = config::roots_config_from_args(app);
    let compositor = CompositorBuilder::new().build_auto(server::RootsServer::default(),
                                                         Box::new(input::RootsInput::new()),
                                                         Box::new(desktop::RootsDesktop::new()));
    compositor.run()
}
