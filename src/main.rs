mod client;
mod models;
mod parser;

use client::grpc::GrpcClient;
use dotenvy::dotenv;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    pretty_env_logger::init();

    let url = std::env::var("YELLOWSTONE_GRPC_URL").expect("YELLOWSTONE_GRPC_URL must be set");
    let program_id1 = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P".to_string();
    let program_id2 = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA".to_string();

    let client = GrpcClient::new(url);
    info!("Starting subscription for Pump: {}", program_id1);
    info!("Starting subscription for PumpAmm: {}", program_id2);
    
    // 创建一个包含两个程序 ID 的向量
    let program_ids = vec![program_id1, program_id2];
    
    // 订阅所有程序
    for program_id in program_ids {
        let client = client.clone();
        tokio::spawn(async move {
            if let Err(e) = client.subscribe(program_id.clone()).await {
                log::error!("Error subscribing to program {}: {:?}", program_id, e);
            }
        });
    }

    // 保持主线程运行
    tokio::signal::ctrl_c().await?;
    Ok(())
}
