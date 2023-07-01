use super::model::{self, GetClip, UpdateClip};
use crate::data::{model::NewClip, DataErr, DatabasePool};

type Result<T> = std::result::Result<T, DataErr>;

// NOTE M accepts any type that implements the Into trait for the GetClip struct
pub async fn get_clip<M: Into<model::GetClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model: GetClip = model.into();
    let shortcode = model.shortcode.to_string();

    Ok(sqlx::query_as!(
        model::Clip,
        "SELECT * FROM clips WHERE shortcode = ?",
        shortcode
    )
    .fetch_one(pool)
    .await?)
}

pub async fn new_clip<M: Into<model::NewClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model: NewClip = model.into();
    // NOTE The query! macro provides a type-safe way to configure SQL queries at compile time
    let _ = sqlx::query!(
        r#"INSERT INTO clips (
            clip_id,
            shortcode,
            content,
            title,
            posted,
            expires,
            password,
            hits
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0
    )
    .execute(pool)
    .await?;

    // NOTE .await will run the get_clip asynchronously, then this returns a Result<model::Clip>
    get_clip(model.shortcode, pool).await
}

pub async fn update_clip<M: Into<model::UpdateClip>>(
    model: M,
    pool: &DatabasePool,
) -> Result<model::Clip> {
    let model: UpdateClip = model.into();
    let _ = sqlx::query!(
        r#"UPDATE clips SET 
        content = ?, 
        expires = ?, 
        password = ?, 
        title = ? 
        WHERE shortcode = ?"#,
        model.content,
        model.expires,
        model.password,
        model.title,
        model.shortcode
    )
    .execute(pool)
    .await?;

    get_clip(model.shortcode, pool).await
}
