use crate::{
    data::DatabasePool,
    service::{self, ServiceErr},
    Shortcode,
};
use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::runtime::Handle;
