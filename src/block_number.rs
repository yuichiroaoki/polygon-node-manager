use crate::request;
use crate::utils::get_env;
use log::info;

async fn get_block_number() -> Result<u64, reqwest::Error> {
    let url = format!("{}block/number", get_env("EXPRESS_SERVER_URL"));
    let res = request::send_get_request(&url).await;
    let current_block_number = res.unwrap().parse::<u64>().unwrap();
    info!("current block number: {}", current_block_number);
    Ok(current_block_number)
}

async fn get_local_block_number() -> Result<u64, reqwest::Error> {
    let hash = request::get_local_block_hash().await.unwrap();
    let url = format!("{}/block/from/{}", get_env("EXPRESS_SERVER_URL"), hash);
    let res = request::send_get_request(&url).await;
    let local_block_number = res.unwrap().parse::<u64>().unwrap();
    info!("local block number: {}", local_block_number);
    Ok(local_block_number)
}

pub async fn check_block_diff(max_diff: i64) -> Result<bool, reqwest::Error> {
    let current_block_number = get_block_number().await?;
    let local_block_number = get_local_block_number().await?;
    let diff: i64 = current_block_number as i64 - local_block_number as i64;
    info!("diff: {}", diff);
    // assert!(diff > max_diff, "diff is too large");
    // Ok(())
    Ok(max_diff > diff)
}
