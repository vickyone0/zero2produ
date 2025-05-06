use actix_web::{web, App, HttpResponse, HttpServer};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
//use sqlx::types::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}





    impl TryFrom<FormData> for NewSubscriber {
        type Error = String;
        fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
        }
        }

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    
    let new_subscriber = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
        };
    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
// pub async fn insert_subscriber(
//     pool: &PgPool,
//     new_subscriber: &NewSubscriber,
// ) -> Result<(), sqlx::Error> {
//     sqlx::query!(
//         r#"
//     INSERT INTO subscriptions (id, email, name, subscribed_at)
//     VALUES ($1, $2, $3, $4)
//     "#,
//     Uuid::new_v4(),
//     new_subscriber.email,
//     new_subscriber.name,
//     Utc::now()

//     )
//     .execute(pool)
//     //.instrument(query_span)
//     .await
//     .map_err(|e| {
//         tracing::error!("Failed to execute query: {:?}", e);
//         e
//     })?;
//     Ok(())
// }

// match sqlx::query!(
//     r#"
//     INSERT INTO subscriptions (id, email, name, subscribed_at)
//     VALUES ($1, $2, $3, $4)
//     "#,
//     request_id,
//     form.email,
//     form.name,
//     Utc::now()
//     )
//     // We use `get_ref` to get an immutable reference to the `PgConnection`
//     // wrapped by `web::Data`.
//     .execute(pool.as_ref())
//     .instrument(query_span)
//     .await
//     {
//         Ok(_) =>  {
//             HttpResponse::Ok().finish()
//         },
//         Err(e) => {
//             tracing::error!("Failed to execute query: {:?}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }

pub fn is_valid_name(s: &str) -> bool {
    let is_empty_or_whitespace = s.trim().is_empty();

    let is_too_long = s.graphemes(true).count() > 256;

    let forbidden_characters = [
        '/', '\\', '(', ')', '<', '>', '{', '}', '[', ']', '|', '`', '~', '!', '@', '#', '$', '%',
        '^', '&', '*', '-', '+', '=', ':', ';', '"', '\'', '?',
    ];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}
