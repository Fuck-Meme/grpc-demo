use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use solana_sdk::signature::Signature;
use std::{
    cell::RefCell, collections::HashMap, error::Error, fmt::Write, ops::ControlFlow, sync::Arc,
    time::Duration,
};
use tokio::sync::Mutex;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient};
use yellowstone_grpc_proto::geyser::{
    subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
    SubscribeRequestFilterTransactions, SubscribeRequestPing,
};

use crate::{
    models::{
        BuyEvent, CompleteEvent, CreateEvent, CreatePoolEvent, CreateV2Event, SellEvent, TradeEvent,
    },
    parser::events::{visit_program_logs, EventTrait},
};

thread_local! {
    static EVENT_DEBUG_BUFFER: RefCell<String> = RefCell::new(String::with_capacity(1024));
}

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
                Ok(msg) => match msg.update_oneof {
                    Some(UpdateOneof::Transaction(sut)) => {
                        let slot = sut.slot;
                        if let Some(tx_info) = sut.transaction {
                            let index = tx_info.index;
                            let signature = Signature::try_from(tx_info.signature.as_slice())
                                .map_err(|err| -> Box<dyn Error> { Box::new(err) })?;
                            if let Some(meta) = tx_info.meta {
                                let start = std::time::Instant::now();
                                let logs = meta.log_messages;
                                if !logs.is_empty() {
                                    self.handle_logs(slot, index, &signature, &logs, start)
                                        .await?;
                                }
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
                    }
                    _ => {}
                },
                Err(e) => {
                    error!("Error: {:?}", e);
                    break;
                }
            }
        }
        Ok(())
    }

    async fn handle_logs(
        &self,
        slot: u64,
        tx_index: u64,
        signature: &Signature,
        logs: &[String],
        start_time: std::time::Instant,
    ) -> Result<(), Box<dyn Error>> {
        let mut logged_create = false;
        let mut logged_create_v2 = false;
        let mut logged_complete = false;
        let mut logged_trade = false;
        let mut logged_buy = false;
        let mut logged_create_pool = false;
        let mut logged_sell = false;
        let mut last_event_at = start_time;

        visit_program_logs(logs, |discriminator, data| {
            if !logged_create && CreateEvent::valid_discrminator(discriminator) {
                if let Ok(create_event) = CreateEvent::from_bytes(data) {
                    Self::log_event(
                        "CreateEvent",
                        &mut last_event_at,
                        slot,
                        tx_index,
                        signature,
                        &create_event,
                    );
                    logged_create = true;
                }
                return ControlFlow::Continue(());
            }

            if !logged_create_v2 && CreateV2Event::valid_discrminator(discriminator) {
                if let Ok(create_v2_event) = CreateV2Event::from_bytes(data) {
                    Self::log_event(
                        "CreateV2Event",
                        &mut last_event_at,
                        slot,
                        tx_index,
                        signature,
                        &create_v2_event,
                    );
                    logged_create_v2 = true;
                }
                return ControlFlow::Continue(());
            }

            if !logged_complete && CompleteEvent::valid_discrminator(discriminator) {
                if let Ok(complete_event) = CompleteEvent::from_bytes(data) {
                    Self::log_event(
                        "CompleteEvent",
                        &mut last_event_at,
                        slot,
                        tx_index,
                        signature,
                        &complete_event,
                    );
                    logged_complete = true;
                }
                return ControlFlow::Continue(());
            }

            if !logged_trade && TradeEvent::valid_discrminator(discriminator) {
                if let Ok(trade_event) = TradeEvent::from_bytes(data) {
                    Self::log_event(
                        "TradeEvent",
                        &mut last_event_at,
                        slot,
                        tx_index,
                        signature,
                        &trade_event,
                    );
                    logged_trade = true;
                }
                return ControlFlow::Continue(());
            }

            if !logged_buy && BuyEvent::valid_discrminator(discriminator) {
                if let Ok(buy_event) = BuyEvent::from_bytes(data) {
                    Self::log_event(
                        "BuyEvent",
                        &mut last_event_at,
                        slot,
                        tx_index,
                        signature,
                        &buy_event,
                    );
                    logged_buy = true;
                }
                return ControlFlow::Continue(());
            }

            if !logged_create_pool && CreatePoolEvent::valid_discrminator(discriminator) {
                if let Ok(create_pool_event) = CreatePoolEvent::from_bytes(data) {
                    Self::log_event(
                        "CreatePoolEvent",
                        &mut last_event_at,
                        slot,
                        tx_index,
                        signature,
                        &create_pool_event,
                    );
                    logged_create_pool = true;
                }
                return ControlFlow::Continue(());
            }

            if !logged_sell && SellEvent::valid_discrminator(discriminator) {
                if let Ok(sell_event) = SellEvent::from_bytes(data) {
                    Self::log_event(
                        "SellEvent",
                        &mut last_event_at,
                        slot,
                        tx_index,
                        signature,
                        &sell_event,
                    );
                    logged_sell = true;
                }
            }

            if logged_create
                && logged_complete
                && logged_trade
                && logged_buy
                && logged_create_pool
                && logged_sell
            {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        Ok(())
    }

    fn log_event<T: std::fmt::Debug>(
        event_name: &str,
        last_event_at: &mut std::time::Instant,
        slot: u64,
        tx_index: u64,
        signature: &Signature,
        event: &T,
    ) {
        EVENT_DEBUG_BUFFER.with(|buffer| {
            let mut buf = buffer.borrow_mut();
            buf.clear();
            let _ = write!(&mut *buf, "{:?}", event);

            let event_body = match buf.find('{') {
                Some(idx) => buf[idx..].trim(),
                None => buf.as_str(),
            };

            let event_duration = last_event_at.elapsed();
            info!(
                "{} {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{} }}",
                event_name, event_duration, slot, tx_index, signature, event_body
            );
            *last_event_at = std::time::Instant::now();
        });
    }
}
