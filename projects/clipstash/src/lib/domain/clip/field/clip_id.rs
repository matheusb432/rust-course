use derive_more::{Constructor, Into};
use serde::{Deserialize, Serialize};

use crate::data::DbId;

#[derive(Clone, Debug, Constructor, Serialize, Deserialize, Into)]
pub struct ClipId(DbId);
impl ClipId {
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl From<DbId> for ClipId {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}
