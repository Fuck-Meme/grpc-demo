use std::{collections::HashMap, error::Error, sync::Arc, time::Duration};
use futures_util::{StreamExt, SinkExt};
use yellowstone_grpc_client::{GeyserGrpcClient, ClientTlsConfig};
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequest, SubscribeRequestFilterTransactions, SubscribeRequestPing,
    subscribe_update::UpdateOneof,
};
use tokio::sync::Mutex;
use log::{error, info};

use crate::{
    models::{CreateEvent, CompleteEvent, TradeEvent, BuyEvent, CreatePoolEvent, SellEvent},
    parser::events::EventTrait,
};

#[derive(Clone)]
pub struct GrpcClient {
    url: String,
}

impl GrpcClient {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn subscribe(&self, program_id: String) -> Result<(), Box<dyn Error>> {
        let client = GeyserGrpcClient::build_from_shared(self.url.clone())?
            .tls_config(ClientTlsConfig::new().with_native_roots())?
            .connect_timeout(Duration::from_secs(10))
            .keep_alive_while_idle(true)
            .timeout(Duration::from_secs(60))
            .connect()
            .await?;

        let client = Arc::new(Mutex::new(client));

        let addrs = vec![program_id];
        let subscribe_request = SubscribeRequest {
            transactions: HashMap::from([(
                "client".to_string(),
                SubscribeRequestFilterTransactions {
                    vote: Some(false),
                    failed: Some(false),
                    signature: None,
                    account_include: addrs,
                    account_exclude: vec![],
                    account_required: vec![],
                },
            )]),
            commitment: Some(CommitmentLevel::Processed.into()),
            ..Default::default()
        };

        let (mut subscribe_tx, mut stream) = client
            .lock()
            .await
            .subscribe_with_request(Some(subscribe_request))
            .await?;

        while let Some(message) = stream.next().await {
            match message {
                Ok(msg) => {
                    match msg.update_oneof {
                        Some(UpdateOneof::Transaction(sut)) => {
                            if let Some(meta) = sut.transaction.and_then(|t| t.meta) {
                                let logs = meta.log_messages;
                                if !logs.is_empty() {
                                    self.handle_logs(&logs).await?;
                                }
                            }
                        }
                        Some(UpdateOneof::Ping(_)) => {
                            let _ = subscribe_tx
                                .send(SubscribeRequest {
                                    ping: Some(SubscribeRequestPing { id: 1 }),
                                    ..Default::default()
                                })
                                .await;
                            info!("Ping sent");
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    error!("Error: {:?}", e);
                    break;
                }
            }
        }
        Ok(())
    }

    async fn handle_logs(&self, logs: &[String]) -> Result<(), Box<dyn Error>> {
        //Pump创建代币
        if let Some(create_event) = CreateEvent::parse_logs::<CreateEvent>(logs) {
            info!("{:?}", create_event);
        }
        //Pump曲线完成
        if let Some(complete_event) = CompleteEvent::parse_logs::<CompleteEvent>(logs) {
            info!("{:?}", complete_event);
        }
        //Pump交易
        if let Some(trade_event) = TradeEvent::parse_logs::<TradeEvent>(logs) {
            info!("{:?}", trade_event);
        }
        //PumpAmm买入
        if let Some(buy_event) = BuyEvent::parse_logs::<BuyEvent>(logs) {
            info!("{:?}", buy_event);
        }
        //PumpAmm创建池
        if let Some(create_pool_event) = CreatePoolEvent::parse_logs::<CreatePoolEvent>(logs) {
            info!("{:?}", create_pool_event);
        }
        //PumpAmm卖出
        if let Some(sell_event) = SellEvent::parse_logs::<SellEvent>(logs) {
            info!("{:?}", sell_event);
        }
        Ok(())
    }
} 