use std::{error::Error, collections::HashMap};
use log::{info};
use crate::models::{CreateEvent, CompleteEvent, TradeEvent, BuyEvent, CreatePoolEvent, SellEvent};
use crate::parser::events::EventTrait;

#[derive(Clone)]
pub struct EventHandler {
    events: HashMap<String, Vec<(u64, String)>>,
}

#[derive(Debug)]
pub struct PumpEvents {
    pub create: Option<CreateEvent>,
    pub complete: Option<CompleteEvent>,
    pub trade: Option<TradeEvent>,
}

#[derive(Debug)]
pub struct PumpAmmEvents {
    pub buy: Option<BuyEvent>,
    pub sell: Option<SellEvent>,
    pub create_pool: Option<CreatePoolEvent>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }

    pub fn parse_pump_events(&self, logs: &[String]) -> PumpEvents {
        PumpEvents {
            create: CreateEvent::parse_logs::<CreateEvent>(logs),
            complete: CompleteEvent::parse_logs::<CompleteEvent>(logs),
            trade: TradeEvent::parse_logs::<TradeEvent>(logs),
        }
    }

    pub fn parse_pump_amm_events(&self, logs: &[String]) -> PumpAmmEvents {
        PumpAmmEvents {
            buy: BuyEvent::parse_logs::<BuyEvent>(logs),
            sell: SellEvent::parse_logs::<SellEvent>(logs),
            create_pool: CreatePoolEvent::parse_logs::<CreatePoolEvent>(logs),
        }
    }

    pub async fn handle_logs(&mut self, logs: &[String], slot: u64, signature: String) -> Result<(), Box<dyn Error>> {
        let mut tx_events = Vec::new();

        // 解析 Pump 事件
        let pump_events = self.parse_pump_events(logs);
        if let Some(create) = pump_events.create {
            tx_events.push(format!("{:?}", create));
        }
        if let Some(complete) = pump_events.complete {
            tx_events.push(format!("{:?}", complete));
        }
        if let Some(trade) = pump_events.trade {
            tx_events.push(format!("{:?}", trade));
        }

        // 解析 PumpAmm 事件
        let pump_amm_events = self.parse_pump_amm_events(logs);
        if let Some(buy) = pump_amm_events.buy {
            tx_events.push(format!("{:?}", buy));
        }
        if let Some(sell) = pump_amm_events.sell {
            tx_events.push(format!("{:?}", sell));
        }
        if let Some(create_pool) = pump_amm_events.create_pool {
            tx_events.push(format!("{:?}", create_pool));
        }

        if !tx_events.is_empty() {
            // 将事件添加到 HashMap
            for event_data in tx_events {
                let event_info = (slot, event_data);
                self.events
                    .entry(signature.clone())
                    .or_insert_with(Vec::new)
                    .push(event_info);
            }

            // 打印当前交易的所有事件
            if let Some(events) = self.events.get(&signature) {
                info!("-----------------------------------------------");
                info!("slot: {}", slot);
                info!("tx: {}", signature);
                info!("events:");
                for (_, event_data) in events {
                    info!("  - {}", event_data);
                }
                info!("-----------------------------------------------");
            }
        }

        Ok(())
    }
} 