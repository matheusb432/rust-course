use crate::web::api::ApiKey;
use crate::{
    data::{query, DatabasePool, Transaction},
    domain::Clip,
    service::ask,
    Shortcode,
};
use std::convert::TryInto;

use super::ServiceErr;

type Result<T> = std::result::Result<T, ServiceErr>;

// NOTE The transactions will be used to defer database writes and batch them together
// ? This will lead to increased performance for increasing the hit count
pub async fn begin_transaction(pool: &DatabasePool) -> Result<Transaction<'_>> {
    Ok(pool.begin().await?)
}

pub async fn end_transaction(transaction: Transaction<'_>) -> Result<()> {
    Ok(transaction.commit().await?)
}

pub async fn increase_hit_count(
    shortcode: &Shortcode,
    hits: u32,
    pool: &DatabasePool,
) -> Result<()> {
    Ok(query::increase_hit_count(shortcode, hits, pool).await?)
}

pub async fn get_clip(req: ask::GetClip, pool: &DatabasePool) -> Result<Clip> {
    let user_password = req.password.clone();
    let clip: Clip = query::get_clip(req, pool).await?.try_into()?;
    let Clip { password, .. } = &clip;

    if password.is_valid(&user_password) {
        Ok(clip)
    } else {
        Err(ServiceErr::PermissionErr("Invalid password".to_owned()))
    }
}

pub async fn new_clip(req: ask::NewClip, pool: &DatabasePool) -> Result<Clip> {
    Ok(query::new_clip(req, pool).await?.try_into()?)
}

pub async fn update_clip(req: ask::UpdateClip, pool: &DatabasePool) -> Result<Clip> {
    Ok(query::update_clip(req, pool).await?.try_into()?)
}

pub async fn generate_api_key(pool: &DatabasePool) -> Result<ApiKey> {
    let api_key = ApiKey::new();
    Ok(query::save_api_key(api_key, pool).await?)
}

pub async fn revoke_api_key(
    api_key: ApiKey,
    pool: &DatabasePool,
) -> Result<query::RevocationStatus> {
    Ok(query::revoke_api_key(api_key, pool).await?)
}

pub async fn api_key_is_valid(api_key: ApiKey, pool: &DatabasePool) -> Result<bool> {
    Ok(query::api_key_is_valid(api_key, pool).await?)
}
