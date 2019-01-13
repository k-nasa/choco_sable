mod remove;
mod switch;
mod util;

use clap::*;

fn main() {
    let matches = app().get_matches();

    match matches.subcommand() {
        ("switch", Some(_)) => switch::exec(),
        ("remove", Some(_)) => remove::exec(),
        _ => app().print_help().unwrap(),
    }
}

fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("switch"))
        .subcommand(SubCommand::with_name("remove"))
}
