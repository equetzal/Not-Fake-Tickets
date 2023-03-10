#![no_std]

// use core::panicking::panic_str;

use core::u128;

use gstd::{exec, msg, prelude::*, ActorId};

pub use market_io::*;
use primitive_types::{H256, U256};
pub mod nft_messages;
use nft_messages::*;
pub mod payment;
pub mod sale;
pub mod state;
use state::*;

pub type ContractAndTokenId = String;

const MIN_TREASURY_FEE: u8 = 0;
const MAX_TREASURT_FEE: u8 = 5;
pub const BASE_PERCENT: u8 = 100;

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Market {
    pub admin_id: ActorId,
    pub approved_nft_contracts: BTreeSet<ActorId>,
    pub approved_ft_contracts: BTreeSet<ActorId>,
    pub approved_wallets: BTreeSet<ActorId>,
    pub treasury_id: ActorId,
    pub treasury_fee: u8,
    pub events: Vec<Event>,
    pub items: BTreeMap<ContractAndTokenId, Item>,
}

static mut MARKET: Option<Market> = None;

impl Market {
    fn add_nft_contract(&mut self, nft_contract_id: &ActorId) {
        self.check_admin();
        self.approved_nft_contracts.insert(*nft_contract_id);
    }

    fn add_ft_contract(&mut self, ft_contract_id: &ActorId) {
        self.check_admin();
        self.approved_ft_contracts.insert(*ft_contract_id);
    }

    fn add_wallet(&mut self, wallet_id: &ActorId) {
        self.check_admin();
        self.approved_wallets.insert(*wallet_id);
    }

    pub async fn add_market_data(
        &mut self,
        nft_contract_id: &ActorId,
        ft_contract_id: Option<ActorId>,
        token_id: U256,
        price: Option<u128>,
        event_id: u128,
    ) {
        self.check_approved_nft_contract(nft_contract_id);
        self.check_approved_ft_contract(ft_contract_id);
        let contract_and_token_id =
            format!("{}{token_id}", H256::from_slice(nft_contract_id.as_ref()));
        // self.on_auction(&contract_and_token_id);
        nft_approve(nft_contract_id, &exec::program_id(), token_id).await;

        let idx = usize::try_from(event_id).unwrap();
        self.events[idx]
            .items
            .entry(contract_and_token_id)
            .and_modify(|item| {
                item.price = price;
                item.ft_contract_id = ft_contract_id
            })
            .or_insert(Item {
                owner_id: msg::source(),
                ft_contract_id,
                price,
            });

        msg::reply(
            MarketEvent::MarketDataAdded {
                nft_contract_id: *nft_contract_id,
                owner: msg::source(),
                token_id,
                price,
            },
            0,
        )
        .expect("Error in reply [MarketEvent::MarketDataAdded]");
    }
    pub fn add_market_event(&mut self, metadata_event: TokenMetadata) {
        self.check_approved_wallet(msg::source());
        self.create_event(msg::source(), metadata_event);
    }

    pub fn check_admin(&self) {
        if msg::source() != self.admin_id {
            panic!("Only owner can make that action");
        }
    }

    pub fn check_approved_nft_contract(&self, nft_contract_id: &ActorId) {
        if !self.approved_nft_contracts.contains(nft_contract_id) {
            panic!("that nft contract is not approved");
        }
    }

    pub fn check_approved_ft_contract(&self, ft_contract_id: Option<ActorId>) {
        if ft_contract_id.is_some()
            && !self
                .approved_ft_contracts
                .contains(&ft_contract_id.expect("Must not be an error here"))
        {
            panic!("that ft contract is not approved");
        }
    }
    pub fn check_approved_wallet(&self, wallet_id: ActorId) {
        if !self.approved_wallets.contains(&wallet_id) {
            panic!("That wallet is not approved to create nft");
        }
    }
    pub fn create_event(&mut self, creator: ActorId, metadata: TokenMetadata) {
        let active = true;
        let items = BTreeMap::new();
        self.events.push(Event {
            creator,
            metadata,
            active,
            items,
        });
    }
    pub fn close_event(&mut self, event_id: u128) {
        let idx = usize::try_from(event_id).unwrap();
        self.events[idx].active = false;
    }
}

#[gstd::async_main]
async fn main() {
    let action: MarketAction = msg::load().expect("Could not load Action");
    let market: &mut Market = unsafe { MARKET.get_or_insert(Market::default()) };
    match action {
        MarketAction::AddNftContract(nft_contract_id) => {
            market.add_nft_contract(&nft_contract_id);
        }
        MarketAction::AddFTContract(nft_contract_id) => {
            market.add_ft_contract(&nft_contract_id);
        }
        MarketAction::AddWallet(wallet_id) => {
            market.add_wallet(&wallet_id);
        }
        MarketAction::CreateEvent(event_metadata) => {
            market.create_event(msg::source(), event_metadata);
        }
        MarketAction::CloseEvent(event_id) => {
            market.close_event(event_id);
        }
        MarketAction::AddMarketData {
            nft_contract_id,
            ft_contract_id,
            token_id,
            price,
            event_id,
        } => {
            market
                .add_market_data(&nft_contract_id, ft_contract_id, token_id, price, event_id)
                .await;
        }

        MarketAction::BuyItem {
            nft_contract_id,
            token_id,
            event_id
        } => {
            market.buy_item(&nft_contract_id, token_id,event_id).await;
        }

        MarketAction::Item {
            nft_contract_id,
            token_id,
        } => {
            let contract_and_token_id =
                format!("{}{token_id}", H256::from_slice(nft_contract_id.as_ref()));
            let item = market
                .items
                .get(&contract_and_token_id)
                .expect("Item does not exist")
                .clone();
            msg::reply(MarketEvent::ItemInfo(item), 0)
                .expect("Error in reply [MarketEvent::ItemInfo]");
        }
        // MarketAction::Withdraw {
        //     nft_contract_id,
        //     token_id,
        //     hash,
        // } => market.withdraw(&nft_contract_id, token_id, hash).await,
    }
}

#[no_mangle]
extern "C" fn init() {
    let config: InitMarket = msg::load().expect("Unable to decode InitConfig");
    if config.treasury_fee == MIN_TREASURY_FEE || config.treasury_fee > MAX_TREASURT_FEE {
        panic!("Wrong treasury fee");
    }
    let market = Market {
        admin_id: config.admin_id,
        ..Default::default()
    };
    unsafe { MARKET = Some(market) };
}

gstd::metadata! {
title: "NFTMarketplace",
    init:
        input: InitMarket,
    handle:
        input: MarketAction,
        output: MarketEvent,
    state:
        input: State,
        output: StateReply,
}

// #[no_mangle]
// extern "C" fn meta_state() -> *mut [i32; 2] {
//     let state: State = msg::load().expect("failed to decode input argument");
//     let market: &mut Market = unsafe { MARKET.get_or_insert(Market::default()) };
//     let encoded = match state {
//         State::AllItems => StateReply::AllItems(market.items.values().cloned().collect()).encode(),
//         State::ItemInfo {
//             nft_contract_id,
//             token_id,
//         } => {
//             let contract_and_token_id =
//                 format!("{}{token_id}", H256::from_slice(nft_contract_id.as_ref()));
//             if let Some(item) = market.items.get(&contract_and_token_id) {
//                 StateReply::ItemInfo(item.clone()).encode()
//             } else {
//                 StateReply::ItemInfo(Item::default()).encode()
//             }
//         }
//     };
//     gstd::util::to_leak_ptr(encoded)
// }
