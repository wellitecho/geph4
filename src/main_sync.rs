use std::collections::HashMap;

use crate::{AuthOpt, CommonOpt};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, StructOpt, Deserialize, Serialize)]
pub struct SyncOpt {
    #[structopt(flatten)]
    common: CommonOpt,

    #[structopt(flatten)]
    auth: AuthOpt,

    /// Forces synchronization of fresh data.
    #[structopt(long)]
    force: bool,
}

pub async fn main_sync(opt: SyncOpt) -> anyhow::Result<()> {
    log::debug!("{}", sync_json(opt).await?);
    Ok(())
}

pub async fn sync_json(opt: SyncOpt) -> anyhow::Result<String> {
    let cbc = crate::to_cached_binder_client(&opt.common, &opt.auth).await?;
    log::info!("sync mode started (force = {})", opt.force);

    let res = cbc.sync(opt.force).await;
    match res {
        Ok(info) => {
            let json = serde_json::to_string(&(info.user_info, info.exits, info.exits_free))?;
            Ok(json)
        }
        Err(err) => {
            let mut haha = HashMap::new();
            haha.insert("error".to_string(), err.to_string());
            let json = serde_json::to_string(&haha)?;
            Ok(json)
        }
    }
}
