// Imports
use clap::{App as ClapApp, Arg, SubCommand};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use actix_web::{get, web, App, HttpServer, Responder};

// Struct definition
#[derive(Debug)]
struct Args {
    config_file: String,
    input_file: String,
    output_file: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    ip: String,
}


#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let args = parse_args();

    let config = load_configs(&args.config_file);
    println!("{:?}", config);

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn parse_args() -> Args {
    let matches = ClapApp::new("My Super Program")
        .version("1.0")
        .author("Damien D. <damien.dube@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Sets a custom config file")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Sets the output file to use")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.toml"
    let config_file = matches
        .value_of("config")
        .unwrap_or("config/default.toml")
        .to_string();
    println!("Value for config: {}", config_file);

    let input_file = matches.value_of("input").unwrap().to_string();
    println!("Using input file: {}", input_file);

    let output_file = matches.value_of("output").unwrap().to_string();
    println!("Using output file: {}", output_file);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    return Args {
        config_file: config_file,
        input_file: input_file,
        output_file: output_file,
    };
}

fn load_configs(config_file: &String) -> Config {
    let config_contents =
        fs::read_to_string(config_file).expect("Something went wrong reading the file");

    let config: Config = match toml::from_str(&config_contents) {
        Ok(config) => config,
        Err(e) => panic!("There was a problem opening the condif file: {:?}", e),
    };

    return config;
}
