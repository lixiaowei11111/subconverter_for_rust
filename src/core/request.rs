use actix_web::{HttpResponse, Responder, get, web};
use awc::{Client, body};
use serde::Deserialize;

#[derive(Deserialize)]
struct SubParams {
    target: String,
    url: String,
}

#[get("/sub")]
pub async fn sub(params: web::Query<SubParams>) -> impl Responder {
    let target = &params.target;
    let url = &params.url;
    HttpResponse::Ok().body(format!("Target: {}\nURL: {}\n", target, url))
}

async fn request(url: String) -> Result<(), awc::error::SendRequestError> {
    let client = Client::default();

    let mut response = client.get(url).send().await?;

    if response.status().is_success() {
        let body = response.body().await?;
    } else {
    }

    Ok(())
}
