use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthorizeQuery {
    redirect_uri: String,
    state: String,
}

#[get("/oauth/authorize")]
pub async fn authorize(query: web::Query<AuthorizeQuery>) -> impl Responder {
    let redirect_uri = format!(
        "{}?code=fake-auth-code&state={}",
        query.redirect_uri, query.state
    );
    HttpResponse::Found()
        .append_header(("Location", redirect_uri))
        .finish()
}
