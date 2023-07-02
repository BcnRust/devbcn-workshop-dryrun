use actix_web::web::ServiceConfig;
use api_lib::health::{hello_world, version};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let pool = actix_web::web::Data::new(pool);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool).service(hello_world).service(version);
    };

    Ok(config.into())
}
