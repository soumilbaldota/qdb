use actix_web::{post, get, web, App, HttpResponse, HttpServer, Responder};
use crossbeam::queue::ArrayQueue;
use tokio::sync::{oneshot};
use crate::types::{QueryRequest};
use std::sync::Arc;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from QDB!")
}

#[post("/query")]
async fn query_handler(
    body: String,
    parser_queue: web::Data<Arc<ArrayQueue<QueryRequest>>>,
) -> impl Responder {
    let (resp_tx, resp_rx) = oneshot::channel();
    let request = QueryRequest {
        raw_sql: body,
        respond_to: resp_tx,
    };

    let mut job = Some(request);
    while let Some(req) = job {
        match parser_queue.push(req) {
            Ok(_) => break,
            Err(e) => {
                job = Some(e);
                tokio::task::yield_now().await;
            }
        }
    }

    match resp_rx.await {
        Ok(result) => HttpResponse::Ok().body(result.output),
        Err(_) => HttpResponse::InternalServerError().body("Execution failed"),
    }
}

pub async fn start(parser_queue: Arc<ArrayQueue<QueryRequest>>) -> std::io::Result<()> {


    println!("Starting web server");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(parser_queue.clone()))
            .service(hello)
            .service(query_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
