use std::{collections::HashMap, error::Error, sync::Arc, time::Duration};
use futures_util::{StreamExt, SinkExt};
use yellowstone_grpc_client::{GeyserGrpcClient, ClientTlsConfig};
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequest, SubscribeRequestFilterTransactions, SubscribeRequestPing,
    subscribe_update::UpdateOneof,
};
use tokio::sync::Mutex;
use log::{error, debug};
use bs58;

use crate::handle::EventHandler;

#[derive(Clone)]
pub struct GrpcClient {
    url: String,
    event_handler: Arc<Mutex<EventHandler>>,
}

impl GrpcClient {
    pub fn new(url: String) -> Self {
        Self { 
            url,
            event_handler: Arc::new(Mutex::new(EventHandler::new())),
        }
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
                            if let Some(meta) = sut.transaction.clone().and_then(|t| t.meta) {
                                let logs = meta.log_messages;
                                if !logs.is_empty() {
                                    let slot = sut.slot;
                                    let signature = sut.transaction
                                        .and_then(|t| t.transaction)
                                        .and_then(|t| t.signatures.first().cloned())
                                        .map(|sig| bs58::encode(sig).into_string())
                                        .unwrap_or_else(|| "unknown".to_string());
                                    
                                    let mut event_handler = self.event_handler.lock().await;
                                    event_handler.handle_logs(&logs, slot, signature).await?;
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
                            debug!("Ping sent");
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
} 