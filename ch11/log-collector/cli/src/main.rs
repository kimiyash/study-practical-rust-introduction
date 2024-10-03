use clap::arg_enum;
use clap::{App, AppSettings, Arg, SubCommand};
use futures_util::StreamExt;
use reqwest::Client;
use std::io;

struct ApiClient {
    server: String,
    client: Client,
}

impl ApiClient {
    async fn post_logs(&self, req: &api::logs::post::Request) -> reqwest::Result<()> {
        self.client
            .post(format!("http://{}/logs", &self.server))
            .json(req)
            .send()
            .await
            .map(|_| ())
    }

    async fn get_logs(&self) -> reqwest::Result<api::logs::get::Response> {
        self.client
            .get(format!("http://{}/logs", &self.server))
            .send()
            .await?
            .json()
            .await
    }

    // async fn get_csv<W: io::Write>(&self, w: &mut W) -> reqwest::Result<u64> {
    //     self.client
    //         .get(&format!("http://{}/csv", &self.server))
    //         .send()
    //         .await?
    //         .copy_to(w)
    // }

    async fn get_csv<W: io::Write>(&self, w: &mut W) -> Result<u64, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(format!("http://{}/csv", &self.server))
            .send()
            .await?;

        let mut total_bytes_written = 0;

        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            w.write_all(&chunk)?; // 書き込みエラーをそのまま伝播
            total_bytes_written += chunk.len() as u64;
        }

        Ok(total_bytes_written) // 書き込んだバイト数を返す
    }
}

arg_enum! {
    #[derive(Debug)]
    enum Format {
        Csv,
        Json,
    }
}

async fn do_post_csv(api_client: &ApiClient) {
    let reader = csv::Reader::from_reader(io::stdin());
    for log in reader.into_deserialize::<api::logs::post::Request>() {
        let log = match log {
            Ok(log) => log,
            Err(e) => {
                eprintln!("[WARN failed to parse a line, skipping: {}", e);
                continue;
            }
        };
        api_client
            .post_logs(&log)
            .await
            .expect("api request failed");
    }
}

async fn do_get_json(api_client: &ApiClient) {
    let res = api_client.get_logs().await.expect("api request failed");
    let json_str = serde_json::to_string(&res).unwrap();
    println!("{}", json_str);
}

async fn do_get_csv(api_client: &ApiClient) {
    let out = io::stdout();
    let mut out = out.lock();
    api_client
        .get_csv(&mut out)
        .await
        .expect("api request failed");
}

#[tokio::main]
async fn main() {
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
                .takes_value(true),
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

    let server = matches
        .value_of("SERVER")
        .unwrap_or("localhost:3000")
        .into();
    let client = Client::new();
    let api_client = ApiClient { server, client };

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
                Format::Csv => do_get_csv(&api_client).await,
                Format::Json => do_get_json(&api_client).await,
            }
        }
        ("post", _) => do_post_csv(&api_client).await,
        _ => unreachable!(),
    }
}
