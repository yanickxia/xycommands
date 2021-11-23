extern crate clap;

use clap::{App, Arg, SubCommand};


fn main() {
    let matches = App::new("xycmd")
        .version("1.0")
        .author("Yanick Xia. <me.yan.xia@qq.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .help("Sets an optional output file")
            .index(1))
        .arg(Arg::with_name("debug")
            .short("d")
            .multiple(true)
            .help("Turn debugging information on"))
        .subcommand(SubCommand::with_name("test")
            .about("does testing things")
            .arg(Arg::with_name("list")
                .short("l")
                .help("lists test values")))
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(o) = matches.value_of("output") {
        println!("Value for output: {}", o);
    }

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("d") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.is_present("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }


    // Continued program logic goes here...
}