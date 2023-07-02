use super::model::{self, GetClip, UpdateClip};
use crate::{
    data::{model::NewClip, DataErr, DatabasePool},
    web::api::ApiKey,
    Shortcode,
};
// ? Necessary to be able to call `Row::get`
use sqlx::Row;

type Result<T> = std::result::Result<T, DataErr>;

pub async fn increase_hit_count(
    shortcode: &Shortcode,
    hits: u32,
    pool: &DatabasePool,
) -> Result<()> {
    let shortcode = shortcode.as_str();
    Ok(sqlx::query!(
        "UPDATE clips SET hits = hits + ? WHERE shortcode = ?",
        hits,
        shortcode
    )
    .execute(pool)
    .await
    .map(|_| ())?)
}

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

pub async fn save_api_key(api_key: ApiKey, pool: &DatabasePool) -> Result<ApiKey> {
    let bytes = api_key.clone().into_inner();
    // ? Inserting the api key's raw bytes into the database
    let _ = sqlx::query!(r#"INSERT INTO api_keys (api_key) VALUES (?)"#, bytes)
        .execute(pool)
        .await
        .map(|_| ())?;
    Ok(api_key)
}

pub enum RevocationStatus {
    Revoked,
    NotFound,
}

pub async fn revoke_api_key(api_key: ApiKey, pool: &DatabasePool) -> Result<RevocationStatus> {
    let bytes = api_key.clone().into_inner();
    Ok(
        sqlx::query!("DELETE FROM api_keys WHERE api_key = ?", bytes)
            .execute(pool)
            .await
            .map(|res| match res.rows_affected() {
                0 => RevocationStatus::NotFound,
                _ => RevocationStatus::Revoked,
            })?,
    )
}

pub async fn api_key_is_valid(api_key: ApiKey, pool: &DatabasePool) -> Result<bool> {
    let bytes = api_key.clone().into_inner();
    Ok(
        sqlx::query("SELECT COUNT(api_key) FROM api_keys WHERE api_key = ?")
            .bind(bytes)
            .fetch_one(pool)
            .await
            .map(|row| {
                let count: u32 = row.get(0);
                count > 0
            })?,
    )
}
