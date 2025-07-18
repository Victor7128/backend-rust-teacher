use actix_web::{web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

mod bd;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let pool = bd::get_pool()
        .await
        .expect("Error conectando a la base de datos Aiven");

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(actix_web::web::Data::new(pool.clone()));
    };

    Ok(config.into())
}