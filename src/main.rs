extern crate clap;

use clap::{Arg, App, SubCommand, ArgMatches};

fn main() {
    let matches = App::new("Staple Static Blog Generator")
                        .version("0.0.1")
                        .author("Kilerd Chan <blove694@gmail.com>")
                        .about("generating blog for you")
                        .subcommand(SubCommand::with_name("init")
                            .about("init staple project")
                            .version("0.0.1")
                            .arg(Arg::with_name("NAME")
                                .required(true)
                                .index(1)
                                .help("the folder for staple")))
                        .subcommand(SubCommand::with_name("new")
                            .about("new article")
                            .version("0.0.1")
                            .arg(Arg::with_name("title")
                                .short("t")
                                .help("title for new article")))
                        .subcommand(SubCommand::with_name("build")
                            .about("generate html")
                            .version("0.0.1"))

                        .get_matches();

    dispatch(matches);
}

fn dispatch(matches: ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("init", Some(m)) => init_project(m),
        ("new", Some(m)) => new_project(m),
        ("build", Some(m)) => build_project(m),
        _ => Ok(())
    }
}

fn init_project(matches: &ArgMatches) -> Result<(), String> {
    Ok(())
}

fn new_project(matches: &ArgMatches) -> Result<(), String> {
    Ok(())
}

fn build_project(matches: &ArgMatches) -> Result<(), String> {
    Ok(())
}