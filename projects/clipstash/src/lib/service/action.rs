use crate::{
    data::{query, DatabasePool},
    domain::Clip,
    service::ask,
};
use std::convert::TryInto;

use super::ServiceErr;

type ActionResult<T> = Result<T, ServiceErr>;

pub async fn get_clip(req: ask::GetClip, pool: &DatabasePool) -> ActionResult<Clip> {
    let user_password = req.password.clone();
    let clip: Clip = query::get_clip(req, pool).await?.try_into()?;
    let Clip { password, .. } = &clip;

    if password.is_valid(&user_password) {
        Ok(clip)
    } else {
        Err(ServiceErr::PermissionErr("Invalid password".to_owned()))
    }
}

pub async fn new_clip(req: ask::NewClip, pool: &DatabasePool) -> ActionResult<Clip> {
    Ok(query::new_clip(req, pool).await?.try_into()?)
}

pub async fn update_clip(req: ask::UpdateClip, pool: &DatabasePool) -> ActionResult<Clip> {
    Ok(query::update_clip(req, pool).await?.try_into()?)
}
