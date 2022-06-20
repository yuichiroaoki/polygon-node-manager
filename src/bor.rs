use crate::block_number::check_block_diff;
use crate::notification::send_notification;
use log::error;
use std::{thread, time};

pub async fn check_if_bor_synced() {
    loop {
        let five_mins = time::Duration::from_secs(300);
        let block_synced = check_block_diff(10).await;

        match block_synced {
            Ok(true) => {
                send_notification("Block states on bor is synced.").await;
                break;
            }
            Ok(false) => (),
            Err(e) => {
                error!("Failed to check bor status, ({:?})", e);
            }
        }
        thread::sleep(five_mins);
    }
}
