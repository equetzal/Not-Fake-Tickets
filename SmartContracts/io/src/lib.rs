#![no_std]

use gstd::{prelude::*, ActorId};
use primitive_types::{H256, U256};
pub type ContractAndTokenId = String;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitMarket {
    pub admin_id: ActorId,
    pub treasury_id: ActorId,
    pub treasury_fee: u8,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone, Default)]
pub struct Item {
    pub owner_id: ActorId,
    pub ft_contract_id: Option<ActorId>,
    pub price: Option<u128>,
}
#[derive(Debug, Decode, Encode, TypeInfo, Default, Clone, PartialEq, Eq)]
pub struct TokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub reference: Option<String>,
}
#[derive(Debug, Encode, Decode, TypeInfo, Clone, Default)]
pub struct Event {
    pub creator: ActorId,
    pub metadata: TokenMetadata,
    pub active: bool,
    pub items: BTreeMap<ContractAndTokenId, Item>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MarketAction {
    /// Adds NFT contract addresses that can be listed on marketplace.
    ///
    /// # Requirements:
    /// Only admin can add approved NFT accounts.
    ///
    /// # Arguments:
    /// * `nft_contract_id`: the NFT contract address
    AddNftContract(ActorId),

    /// Adds the contract addresses of fungible tokens with which users can pay for NFTs.
    ///
    /// # Requirements:
    /// Only admin can add approved fungible-token accounts.
    ///
    /// # Arguments:
    /// * `ft_contract_id`: the FT contract address
    AddFTContract(ActorId),

    // Add wallet which can create NFT
    AddWallet(ActorId),
    /// Adds data on market item.
    /// If the item of that NFT does not exist on the marketplace then it will be listed.
    /// If the item exists then that action is used to change the price or suspend the sale.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be the NFT owner
    /// * `nft_contract_id` must be in the list of `approved_nft_contracts`
    /// * if item already exists, then it cannot be changed if there is an active auction
    ///
    /// Arguments:
    /// * `nft_contract_id`: the NFT contract address
    /// * `token_id`: the NFT id
    /// * `price`: the NFT price (if it is `None` then the item is not on the sale)
    ///
    /// On success replies [`MarketEvent::MarketDataAdded`].
    AddMarketData {
        nft_contract_id: ActorId,
        ft_contract_id: Option<ActorId>,
        token_id: U256,
        price: Option<u128>,
        event_id: u128,
    },

    CreateEvent(TokenMetadata),
    CloseEvent(u128),
    /// Sells the NFT.
    ///
    /// # Requirements:
    /// * The NFT item must exists and be on sale.
    /// * If the NFT is sold for a native Gear value, then a buyer must attach value equals to the price.
    /// * If the NFT is sold for fungible tokens then a buyer must have enough tokens in the fungible token contract.
    /// * There must be no an opened auction on the item.
    ///
    /// Arguments:
    /// * `nft_contract_id`: NFT contract address
    /// * `token_id`: the token ID
    ///
    /// On success replies [`MarketEvent::ItemSold`].
    BuyItem {
        nft_contract_id: ActorId,
        token_id: U256,
        event_id: u128,
    },

    /// Creates an auction for selected item.
    /// If the NFT item doesn't exist on the marketplace then it will be listed
    ///
    /// Requirements:
    /// * Only the item owner can start auction.
    /// * `nft_contract_id` must be in the list of `approved_nft_contracts`
    /// *  There must be no active auction.
    /// Adds a price offer to the item.
    ///

    /// Withdraws tokens.
    ///
    /// Requirements:
    /// * NFT item must exists and be listed on the marketplace.
    /// * Only the offer creator can withdraw his tokens.
    /// * The offer with indicated hash must exist.
    ///
    /// Arguments:
    /// * `nft_contract_id`: the NFT contract address
    /// * `token_id`: the NFT id
    /// * `offer_hash`: the offer hash that includes the offer price and the address of fungible token contract.
    ///
    /// On success replies [`MarketEvent::TokensWithdrawn`].
    // Withdraw {
    //     nft_contract_id: ActorId,
    //     token_id: U256,
    //     hash: H256,
    // },

    /// Accepts an offer.
    ///
    /// Requirements:
    /// * NFT item must exists and be listed on the marketplace.
    /// * Only owner can accept offer.
    /// * There must be no ongoing auction.
    /// * The offer with indicated hash must exist.
    ///
    /// Arguments:
    /// * `nft_contract_id`: the NFT contract address
    /// * `token_id`: the NFT id
    /// * `offer_hash`: the offer hash that includes the offer price and the address of fungible token contract.
    ///
    /// On success replies [`MarketEvent::OfferAccepted`].
    Item {
        nft_contract_id: ActorId,
        token_id: U256,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MarketEvent {
    MarketDataAdded {
        nft_contract_id: ActorId,
        owner: ActorId,
        token_id: U256,
        price: Option<u128>,
    },
    ItemSold {
        owner: ActorId,
        nft_contract_id: ActorId,
        token_id: U256,
    },
    NFTListed {
        nft_contract_id: ActorId,
        owner: ActorId,
        token_id: U256,
        price: Option<u128>,
    },
    ItemInfo(Item),
    TokensWithdrawn {
        nft_contract_id: ActorId,
        token_id: U256,
        price: u128,
    },
}
