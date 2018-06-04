extern crate clap;

use clap::{Arg, App, SubCommand};

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
    

    println!("Hello, world!");
}
