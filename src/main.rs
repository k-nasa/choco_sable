mod remove;
mod switch;

use clap::*;

fn main() {
    let matches = app().get_matches();

    match matches.subcommand() {
        ("switch", Some(matches)) => switch::exec(matches),
        ("remove", Some(matches)) => remove::exec(matches),
        _ => app().print_help().unwrap(),
    }
}

fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(SubCommand::with_name("switch").arg(Arg::with_name("branch").required(true)))
        .subcommand(SubCommand::with_name("remove").arg(Arg::with_name("branches").multiple(true)))
}
