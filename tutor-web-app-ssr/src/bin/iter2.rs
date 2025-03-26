use actix_web::{
    App, Error, HttpResponse, HttpServer, error,
    web::{self, Data},
};
use tera::Tera;

// Store Tera template in Application State
async fn index(tmpl: Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = tmpl
        .render("form.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

//----------------------------------------------------------------------

#[derive(serde::Deserialize)]
pub struct Tutor {
    name: String,
}

//----------------------------------------------------------------------
// curl -X POST localhost:8080/tutors -d "name=Terry"
async fn handle_post_tutor(
    tmpl: Data<tera::Tera>,
    params: web::Form<Tutor>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", &params.name);
    ctx.insert("text", "Welcome!");
    let s = tmpl
        .render("user.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// ══════════════════════════════════════════════════════════════════════
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on: 127.0.0.1:8085, open browser and visit have a try!");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter2/**/*")).unwrap();

        App::new().app_data(Data::new(tera)).configure(app_config)
    })
    .bind("127.0.0.1:8085")?
    .run()
    .await
}

//----------------------------------------------------------------------
fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/tutor").route(web::post().to(handle_post_tutor))),
    );
}
