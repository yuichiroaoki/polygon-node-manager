use crate::block_number::check_block_diff;
use crate::notification::send_notification;
use std::{thread, time};

pub async fn check_if_bor_synced() {
    loop {
        let five_mins = time::Duration::from_secs(300);
        let block_synced = check_block_diff(10).await;
        if block_synced {
            send_notification("Block states on bor is synced.").await;
            break;
        }
        thread::sleep(five_mins);
    }
}
