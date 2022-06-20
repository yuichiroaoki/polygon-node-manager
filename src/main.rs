use clap::Parser;
use dotenv::dotenv;
use log::info;
use std::thread;

use clokwerk::{AsyncScheduler, TimeUnits};
use std::time::Duration;

mod block_number;
mod bor;
mod disk;
mod notification;
mod request;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Delete logs
    #[clap(long)]
    clean: bool,

    /// Check if bor is fully synced
    #[clap(long)]
    bor: bool,

    /// Check if disk is full
    #[clap(long)]
    disk: bool,

    /// Send a message with telegram
    #[clap(long)]
    msg: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    env_logger::init();
    info!("starting up");

    let args = Args::parse();

    if args.clean {
        disk::delete_logs();
    }

    if args.disk {
        let mut scheduler = AsyncScheduler::new();
        scheduler.every(1.hours()).run(|| async {
            let available_disk = disk::available_disk().unwrap();
            let available_disk_gb = utils::round_float(utils::byte_to_gb(available_disk), 2);
            info!("available disk space: {} GB", available_disk_gb);
            if available_disk < 10_000_000_000 {
                let msg = format!(
                    "Available disk space is less than 10GB. Available disk space: {available_disk_gb} GB",
                );

                disk::delete_logs();
                notification::send_notification(&msg).await;
            }
        });

        loop {
            scheduler.run_pending().await;
            thread::sleep(Duration::from_millis(10));
        }
    }

    if args.bor {
        bor::check_if_bor_synced().await;
    }

    if args.msg != None {
        notification::send_notification(&args.msg.unwrap()).await;
    }
}
