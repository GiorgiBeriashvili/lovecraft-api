use crate::manuscript::{Category, Manuscripts, Parameters, Sort};
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::SqlitePool;

#[get("/manuscripts")]
async fn find_all(
    db_pool: web::Data<SqlitePool>,
    parameters: web::Query<Parameters>,
) -> impl Responder {
    let result = match &parameters.category {
        Some(category) => match category {
            Category::All => match &parameters.sort {
                Some(sort) => Manuscripts::find_all(db_pool.get_ref(), sort).await,
                None => Manuscripts::find_all(db_pool.get_ref(), &Sort::Ascending).await,
            },
            _ => match &parameters.sort {
                Some(sort) => {
                    Manuscripts::find_all_by_category(db_pool.get_ref(), category, sort).await
                }
                None => {
                    Manuscripts::find_all_by_category(db_pool.get_ref(), category, &Sort::Ascending)
                        .await
                }
            },
        },
        None => match &parameters.sort {
            Some(sort) => Manuscripts::find_all(db_pool.get_ref(), sort).await,
            None => Manuscripts::find_all(db_pool.get_ref(), &Sort::Ascending).await,
        },
    };

    match result {
        Ok(manuscripts) => HttpResponse::Ok().json(manuscripts),
        _ => HttpResponse::BadRequest().body("Error trying to read all entries from the database"),
    }
}

#[get("/manuscripts/{id}")]
async fn find_by_id(id: web::Path<i32>, db_pool: web::Data<SqlitePool>) -> impl Responder {
    let result = Manuscripts::find_by_id(id.into_inner(), db_pool.get_ref()).await;

    match result {
        Ok(manuscript) => HttpResponse::Ok().json(manuscript),
        _ => HttpResponse::BadRequest().body("Manuscript not found"),
    }
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api").service(web::scope("/v1").service(find_all).service(find_by_id)),
    );
}
