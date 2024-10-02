use clap::{Arg, App, AppSettings, SubCommand};
use clap::{_clap_count_exprs, arg_enum};

arg_enum! {
    #[derive(Debug)]
    enum Format {
        Csv,
        Json,
    }
}

fn main() {
    let opts = App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("SERVER")
                .short("s")
                .long("server")
                .value_name("URL")
                .help("server url")
                .takes_value(true,)
        )
        .subcommand(SubCommand::with_name("post").about("post logs, taking input from stdin"))
        .subcommand(
                SubCommand::with_name("get").about("get logs").arg(
                    Arg::with_name("FORMAT")
                    .help("log format")
                    .short("f")
                    .long("format")
                    .takes_value(true)
                    // csv json のみ受け付ける
                    .possible_values(&Format::variants())
                    .case_insensitive(true),
                ),
        );

        let matches = opts.get_matches();

        let server = matches.value_of("SERVER").unwrap_or("localhost:3000");
        match matches.subcommand() {
            ("get", sub_match) => println!("get: {:?}", sub_match),
            ("post", sub_match) => println!("post: {:?}", sub_match),
            _ => unreachable!(),
        }

        match matches.subcommand() {
            ("get", sub_match) => {
                let format = sub_match
                    .and_then(|m| m.value_of("FORMAT"))
                    .map(|m| m.parse().unwrap())
                    .unwrap();
                match format {
                    Format::Csv => unimplemented!(),
                    Format::Json => unimplemented!(),
                }
            },
        }
}
