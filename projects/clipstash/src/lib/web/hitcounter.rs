use crate::{
    data::DatabasePool,
    service::{self, ServiceErr},
    Shortcode,
};
use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use parking_lot::Mutex;
use std::{collections::HashMap, ops::DerefMut, sync::Arc, time::Duration};
use tokio::runtime::Handle;

#[derive(Debug, thiserror::Error)]
enum HitCountErr {
    #[error("service error: {0}")]
    Service(#[from] ServiceErr),
    #[error("channel communication error: {0}")]
    Channel(#[from] crossbeam_channel::SendError<HitCountMsg>),
}

enum HitCountMsg {
    Commit,
    Hit(Shortcode, u32),
}

type ModResult<T> = Result<T, HitCountErr>;

// NOTE The hit store type is a thread-safe, reference-counted, mutex-protected hashmap
type HitStore = Arc<Mutex<HashMap<Shortcode, u32>>>;

pub struct HitCounter {
    tx: Sender<HitCountMsg>,
}

impl HitCounter {
    /// Will commit the current hit counts to the database every 5 seconds after the channel is empty
    /// NOTE The performance gain from batching the hits is significant, from 400 RPS to 45000 RPS
    pub fn new(pool: DatabasePool, handle: Handle) -> Self {
        let (tx, rx) = unbounded::<HitCountMsg>();
        let tx_clone = tx.clone();
        let rx_clone = rx.clone();

        let _ = std::thread::spawn(move || {
            println!("HitCounter thread spawned");
            let store: HitStore = Arc::new(Mutex::new(HashMap::new()));
            loop {
                match rx_clone.try_recv() {
                    Ok(msg) => {
                        if let Err(e) =
                            Self::process_msg(msg, store.clone(), handle.clone(), pool.clone())
                        {
                            eprintln!("hit counter error: {e}");
                        }
                    }
                    Err(e) => match e {
                        TryRecvError::Empty => {
                            std::thread::sleep(Duration::from_secs(5));
                            if let Err(e) = tx_clone.send(HitCountMsg::Commit) {
                                eprintln!("error sending commit msg to hits channel: {e}");
                            }
                        }
                        _ => break,
                    },
                }
            }
        });
        Self { tx }
    }

    pub fn hit(&self, shortcode: Shortcode, count: u32) {
        // NOTE Sending a message to the channel to be processed by the background task
        // ? This is more performant than directly writing to the database
        if let Err(e) = self.tx.send(HitCountMsg::Hit(shortcode, count)) {
            eprintln!("hit count error: {e}");
        }
    }

    /// Commit the hits to the database and clears the hit store
    fn commit_hits(hits: HitStore, handle: Handle, pool: DatabasePool) -> ModResult<()> {
        // ? Desugars to Arc::clone(&hits)
        let hits = hits.clone();

        let hits: Vec<(Shortcode, u32)> = {
            // NOTE `hits` will be dropped at the end of this block, releasing the lock
            let mut hits = hits.lock();
            let hits_vec = hits.iter().map(|(k, v)| (k.clone(), *v)).collect();
            hits.clear();
            hits_vec
        };
        handle.block_on(async move {
            let transaction = service::action::begin_transaction(&pool).await?;
            for (shortcode, count) in hits {
                if let Err(e) = service::action::increase_hit_count(&shortcode, count, &pool).await
                {
                    eprintln!("error increasing hit count: {e}");
                };
            }
            Ok(service::action::end_transaction(transaction).await?)
        })
    }

    fn process_msg(
        msg: HitCountMsg,
        hits: HitStore,
        handle: Handle,
        pool: DatabasePool,
    ) -> ModResult<()> {
        match msg {
            HitCountMsg::Commit => Self::commit_hits(hits.clone(), handle.clone(), pool.clone())?,
            HitCountMsg::Hit(shortcode, count) => {
                let mut hitcount = hits.lock();
                // let hitcount = hitcount.entry(shortcode).or_insert(0);
                // *hitcount += count;
                // ? Equivalent to the above code
                hitcount
                    .entry(shortcode)
                    .and_modify(|hc| *hc += count)
                    .or_insert(0);
            }
        };
        Ok(())
    }
}
