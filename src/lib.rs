pub mod configuration;
pub mod routes;
pub mod startup;

// #[get("/health_check")]
// async fn health_check() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// #[derive(Deserialize)]
// struct FormData {
//     email: String,
//     name: String
// }

// #[post("/subscriptions")]
// async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
//         .listen(listener)?
//         .run();

//     Ok(server)
// }
