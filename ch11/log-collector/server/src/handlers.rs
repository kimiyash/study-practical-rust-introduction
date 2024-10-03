use crate::db;
use crate::Server;
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use diesel::pg::PgConnection;
use futures::{StreamExt, TryStreamExt};
use itertools::Itertools;
use log::debug;
use std::io::{BufReader, Read};
use std::sync::Arc;

fn load_file(
    conn: &mut PgConnection,
    file: impl Read,
) -> Result<usize, Box<dyn std::error::Error>> {
    use crate::model::NewLog;

    let mut ret = 0;

    // CSVファイルが渡される`csv::Reader`を用いてapi::Logにデコードしていく
    let in_csv = BufReader::new(file);
    let in_log = csv::Reader::from_reader(in_csv).into_deserialize::<::api::Log>();

    // Itertoolsのchunksを用いて1000件ずつ処理する
    for logs in &in_log.chunks(1000) {
        let logs = logs
            .filter_map(Result::ok)
            .map(|log| NewLog {
                user_agent: log.user_agent,
                response_time: log.response_time,
                timestamp: log.timestamp.naive_utc(),
            })
            .collect_vec();

        let inserted = db::insert_logs(conn, &logs)?;
        ret += inserted.len();
    }

    Ok(ret)
}

/// `POST /csv`のハンドラ
pub async fn handle_post_csv(server: web::Data<Server>, mut payload: Multipart) -> impl Responder {
    let mut conn = server.pool.get().expect("DB connection failed");

    let mut total_sum = 0;

    while let Ok(Some(field)) = payload.try_next().await {
        if field.content_disposition().get_filename().is_some() {
            // Use `try_fold` to accumulate chunks of data into a single `Vec<u8>`
            let file_data = field
                .try_fold(Vec::new(), |mut acc, chunk| async move {
                    acc.extend_from_slice(&chunk);
                    Ok(acc)
                })
                .await;

            match file_data {
                Ok(data) => {
                    let file_result = load_file(&mut conn, &data[..]);
                    match file_result {
                        Ok(sum) => total_sum += sum,
                        Err(_) => return HttpResponse::InternalServerError().finish(),
                    }
                }
                Err(_) => return HttpResponse::BadRequest().finish(),
            }
        }
    }

    HttpResponse::Ok().json(api::csv::post::Response(total_sum))
}

/// `POST /logs`のハンドラ
pub async fn handle_post_logs(
    server: web::Data<Server>,
    log: web::Json<api::logs::post::Request>,
) -> impl Responder {
    use crate::model::NewLog;
    use chrono::Utc;

    let log = NewLog {
        user_agent: log.user_agent.clone(),
        response_time: log.response_time,
        timestamp: log.timestamp.unwrap_or_else(|| Utc::now()).naive_utc(),
    };

    let mut conn = server.pool.get().expect("Failed to get DB connection");

    db::insert_log(&mut conn, &log).expect("Failed to insert log");

    debug!("received log: {:?}", log);

    HttpResponse::Accepted().finish()
}

/// `GET /logs`のハンドラ
pub async fn handle_get_logs(
    server: web::Data<Server>,
    range: web::Query<api::logs::get::Query>,
) -> impl Responder {
    use chrono::{DateTime, Utc};

    let mut conn = server.pool.get().expect("Failed to get DB connection");
    let logs = db::logs(&mut conn, range.from, range.until).expect("DB query failed");
    let logs = logs
        .into_iter()
        .map(|log| api::Log {
            user_agent: log.user_agent,
            response_time: log.response_time,
            timestamp: DateTime::from_utc(log.timestamp, Utc),
        })
        .collect::<Vec<_>>();

    HttpResponse::Ok().json(api::logs::get::Response(logs))
}

/// `GET /csv`のハンドラ
pub async fn handle_get_csv(
    server: web::Data<Server>,
    range: web::Query<api::csv::get::Query>,
) -> impl Responder {
    use chrono::{DateTime, Utc};

    let mut conn = server.pool.get().expect("Failed to get DB connection");
    let logs = db::logs(&mut conn, range.from, range.until).expect("DB query failed");
    let v = Vec::new();
    let mut w = csv::Writer::from_writer(v);

    for log in logs.into_iter().map(|log| api::Log {
        user_agent: log.user_agent,
        response_time: log.response_time,
        timestamp: DateTime::from_utc(log.timestamp, Utc),
    }) {
        w.serialize(log).expect("Failed to serialize log");
    }

    let csv = w.into_inner().expect("Failed to finalize CSV writer");
    HttpResponse::Ok().content_type("text/csv").body(csv)
}
