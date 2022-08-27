use crate::utils::get_env;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use log::info;
use std::convert::TryFrom;

async fn get_block_number() -> Result<u64, Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(get_env("ALCHEMY_JSON_RPC_URL"))
        .expect("could not instantiate HTTP Provider");
    let block_num = provider.get_block_number().await?;
    println!("Current block number: {}", block_num);
    Ok(block_num.as_u64())
}

async fn get_local_block_number() -> Result<u64, Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(get_env("JSON_RPC_URL"))
        .expect("could not instantiate HTTP Provider");
    let block_num = provider.get_block_number().await?;
    println!("Local block number: {}", block_num);
    Ok(block_num.as_u64())
}

pub async fn check_block_diff(max_diff: i64) -> Result<bool, Box<dyn std::error::Error>> {
    let current_block_number = get_block_number().await?;
    let local_block_number = get_local_block_number().await?;
    let diff: i64 = current_block_number as i64 - local_block_number as i64;
    info!("diff: {}", diff);
    // assert!(diff > max_diff, "diff is too large");
    // Ok(())
    Ok(max_diff > diff)
}

#[cfg(test)]
mod test {
    use crate::block_number;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_get_block_number() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();
        let block_number = block_number::get_block_number().await?;
        assert_eq!(block_number > 0, true);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_local_block_number() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();
        let block_number = block_number::get_local_block_number().await?;
        assert_eq!(block_number > 0, true);
        Ok(())
    }

    #[tokio::test]
    async fn test_check_block_diff() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();
        let block_synced = block_number::check_block_diff(10).await?;
        assert_eq!(block_synced, true);
        Ok(())
    }
}
