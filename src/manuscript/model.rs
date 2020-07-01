use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Deserialize)]
pub enum Sort {
    Ascending,
    Descending,
}

#[derive(Debug, Deserialize)]
pub enum Category {
    All,
    Fiction,
    Poetry,
    Essay,
    Letter,
}

impl std::fmt::Display for Category {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, formatter)
    }
}

#[derive(Serialize, FromRow)]
pub struct Manuscript {
    pub id: i32,
    pub category: String,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, FromRow)]
pub struct Entry {
    pub id: i32,
    pub category: String,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, FromRow)]
pub struct Manuscripts {
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
pub struct Parameters {
    pub category: Option<Category>,
    pub sort: Option<Sort>,
}

impl Responder for Manuscripts {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _request: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).expect("Failed to serialize manuscripts!");

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

impl Manuscripts {
    pub async fn find_all(pool: &SqlitePool, sort: &Sort) -> Result<Vec<Entry>> {
        let mut manuscripts = vec![];

        let records = sqlx::query!(
            r#"
                SELECT id, category, title, description
                FROM manuscript
                ORDER BY id;
            "#
        )
        .fetch_all(pool)
        .await?;

        for record in records {
            manuscripts.push(Entry {
                id: record.id,
                category: record.category,
                title: record.title,
                description: record.description,
            });
        }

        match sort {
            Sort::Ascending => (),
            Sort::Descending => manuscripts.reverse(),
        }

        Ok(manuscripts)
    }

    pub async fn find_all_by_category(
        pool: &SqlitePool,
        category: &Category,
        sort: &Sort,
    ) -> Result<Vec<Entry>> {
        let mut manuscripts = vec![];

        let records = sqlx::query!(
            r#"
                SELECT id, category, title, description
                FROM manuscript
                WHERE category = ?
                ORDER BY id;
            "#,
            format!("{}", &category)
        )
        .fetch_all(pool)
        .await?;

        for record in records {
            manuscripts.push(Entry {
                id: record.id,
                category: record.category,
                title: record.title,
                description: record.description,
            });
        }

        match sort {
            Sort::Ascending => (),
            Sort::Descending => manuscripts.reverse(),
        }

        Ok(manuscripts)
    }

    pub async fn find_by_id(id: i32, pool: &SqlitePool) -> Result<Manuscript> {
        let record = sqlx::query!(
            r#"
                SELECT id, category, title, content
                FROM manuscript WHERE id = $1;
            "#,
            id
        )
        .fetch_one(&*pool)
        .await?;

        Ok(Manuscript {
            id: record.id,
            category: record.category,
            title: record.title,
            content: record.content,
        })
    }
}
