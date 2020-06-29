use crate::texts::{Parameters, Sort, Texts};
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::SqlitePool;

#[get("/texts")]
async fn find_all(
    db_pool: web::Data<SqlitePool>,
    parameters: web::Query<Parameters>,
) -> impl Responder {
    let result = match &parameters.category {
        Some(category) => match &parameters.sort {
            Some(sort) => Texts::find_all_by_category(db_pool.get_ref(), category, sort).await,
            None => {
                Texts::find_all_by_category(db_pool.get_ref(), category, &Sort::Ascending).await
            }
        },
        None => match &parameters.sort {
            Some(sort) => Texts::find_all(db_pool.get_ref(), sort).await,
            None => Texts::find_all(db_pool.get_ref(), &Sort::Ascending).await,
        },
    };

    match result {
        Ok(texts) => HttpResponse::Ok().json(texts),
        _ => HttpResponse::BadRequest().body("Error trying to read all entries from the database"),
    }
}

#[get("/texts/{id}")]
async fn find_by_id(id: web::Path<i32>, db_pool: web::Data<SqlitePool>) -> impl Responder {
    let result = Texts::find_by_id(id.into_inner(), db_pool.get_ref()).await;

    match result {
        Ok(text) => HttpResponse::Ok().json(text),
        _ => HttpResponse::BadRequest().body("Text not found"),
    }
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api").service(web::scope("/v1").service(find_all).service(find_by_id)),
    );
}
