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
pub struct Text {
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
pub struct Texts {
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
pub struct Parameters {
    pub category: Option<Category>,
    pub sort: Option<Sort>,
}

impl Responder for Texts {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _request: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).expect("Failed to serialize texts!");

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

impl Texts {
    pub async fn find_all(pool: &SqlitePool, sort: &Sort) -> Result<Vec<Entry>> {
        let mut texts = vec![];

        let records = sqlx::query!(
            r#"
                SELECT id, category, title, description
                FROM texts
                ORDER BY id;
            "#
        )
        .fetch_all(pool)
        .await?;

        for record in records {
            texts.push(Entry {
                id: record.id,
                category: record.category,
                title: record.title,
                description: record.description,
            });
        }

        match sort {
            Sort::Ascending => (),
            Sort::Descending => texts.reverse(),
        }

        Ok(texts)
    }

    pub async fn find_all_by_category(
        pool: &SqlitePool,
        category: &Category,
        sort: &Sort,
    ) -> Result<Vec<Entry>> {
        let mut texts = vec![];

        let records = sqlx::query!(
            r#"
                SELECT id, category, title, description
                FROM texts
                WHERE category = ?
                ORDER BY id;
            "#,
            format!("{}", &category)
        )
        .fetch_all(pool)
        .await?;

        for record in records {
            texts.push(Entry {
                id: record.id,
                category: record.category,
                title: record.title,
                description: record.description,
            });
        }

        match sort {
            Sort::Ascending => (),
            Sort::Descending => texts.reverse(),
        }

        Ok(texts)
    }

    pub async fn find_by_id(id: i32, pool: &SqlitePool) -> Result<Text> {
        let record = sqlx::query!(
            r#"
                SELECT id, category, title, content
                FROM texts WHERE id = $1;
            "#,
            id
        )
        .fetch_one(&*pool)
        .await?;

        Ok(Text {
            id: record.id,
            category: record.category,
            title: record.title,
            content: record.content,
        })
    }
}
