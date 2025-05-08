use std::error::Error;
use base64::{Engine, engine::general_purpose};
use borsh::BorshDeserialize;
use crate::models::{CreateEvent, TradeEvent, CompleteEvent, BuyEvent, CreatePoolEvent, SellEvent};

const PROGRAM_DATA: &str = "Program data: ";

pub trait EventTrait: Sized + std::fmt::Debug {
    fn discriminator() -> [u8; 8];
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>>;
    fn valid_discrminator(head: &[u8]) -> bool;

    fn parse_logs<T: EventTrait + Clone>(logs: &[String]) -> Option<T> {
        logs.iter().rev().find_map(|log| {
            let payload = log.strip_prefix(PROGRAM_DATA)?;
            let bytes = general_purpose::STANDARD
                .decode(payload)
                .map_err(|e| Box::new(e) as Box<dyn Error>)
                .ok()?;

            let (discr, rest) = bytes.split_at(8);
            if Self::valid_discrminator(discr) {
                T::from_bytes(rest).ok()
            } else {
                None
            }
        })
    }
}

impl EventTrait for CreateEvent {
    fn discriminator() -> [u8; 8] {
        [27, 114, 169, 77, 222, 235, 99, 118]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == Self::discriminator()
    }
}

impl EventTrait for CompleteEvent {
    fn discriminator() -> [u8; 8] {
        [95, 114, 97, 156, 212, 46, 152, 8]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == Self::discriminator()
    }
}

impl EventTrait for TradeEvent {
    fn discriminator() -> [u8; 8] {
        [189, 219, 127, 211, 78, 230, 97, 238]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == Self::discriminator()
    }
}

impl EventTrait for BuyEvent {
    fn discriminator() -> [u8; 8] {
        [103, 244, 82, 31, 44, 245, 119, 119]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == Self::discriminator()
    }
}

impl EventTrait for CreatePoolEvent {
    fn discriminator() -> [u8; 8] {
        [177, 49, 12, 210, 160, 118, 167, 116]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == Self::discriminator()
    }
}

impl EventTrait for SellEvent {
    fn discriminator() -> [u8; 8] {
        [62, 47, 55, 10, 165, 3, 220, 42]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == Self::discriminator()
    }
} 