use crate::{nft_messages::*, payment::*, Market, MarketEvent, BASE_PERCENT};
use gstd::{msg, prelude::*, ActorId};
use primitive_types::{H256, U256};

impl Market {
    pub async fn buy_item(&mut self, nft_contract_id: &ActorId, token_id: U256, event_id: u128) {
        let contract_and_token_id =
            format!("{}{token_id}", H256::from_slice(nft_contract_id.as_ref()));

        let idx = usize::try_from(event_id).unwrap();
        let mut item = self.events[idx]
            .items
            .get_mut(&contract_and_token_id)
            .expect("Item does not exist");

        // let event = &self.events[event_id];

        // if event.active != true {
        //     panic!("There is no active sell");
        // }
        let price = item.price.expect("The item is not on sale");

        check_attached_value(item.ft_contract_id, price);
        // fee for treasury
        let treasury_fee = price * (self.treasury_fee * BASE_PERCENT) as u128 / 10_000u128;

        transfer_payment(
            &msg::source(),
            &self.treasury_id,
            item.ft_contract_id,
            treasury_fee,
        )
        .await;

        // transfer NFT and pay royalties
        let payouts = nft_transfer(
            nft_contract_id,
            &msg::source(),
            token_id,
            price - treasury_fee,
        )
        .await;
        for (account, amount) in payouts.iter() {
            transfer_payment(&msg::source(), account, item.ft_contract_id, *amount).await;
        }

        item.owner_id = msg::source();
        item.price = None;

        msg::reply(
            MarketEvent::ItemSold {
                owner: msg::source(),
                nft_contract_id: *nft_contract_id,
                token_id,
            },
            0,
        )
        .expect("Error in reply [MarketEvent::ItemSold]");
    }

    // pub async fn withdraw(&mut self, nft_contract_id: &ActorId, token_id: U256, offer_hash: H256) {
    //     let contract_and_token_id =
    //         format!("{}{token_id}", H256::from_slice(nft_contract_id.as_ref()));
    //     let item = self
    //         .items
    //         .get_mut(&contract_and_token_id)
    //         .expect("Item does not exist");

    //     let mut offers = item.offers.clone();
    //     if let Some(offer) = offers.clone().iter().find(|offer| offer.hash == offer_hash) {
    //         if msg::source() != offer.id {
    //             panic!("can't withdraw other user's tokens");
    //         }
    //         transfer_payment(
    //             &exec::program_id(),
    //             &msg::source(),
    //             offer.ft_contract_id,
    //             offer.price,
    //         )
    //         .await;
    //         offers.retain(|offer| offer.hash != offer_hash);
    //         item.offers = offers;
    //         msg::reply(
    //             MarketEvent::TokensWithdrawn {
    //                 nft_contract_id: *nft_contract_id,
    //                 token_id,
    //                 price: offer.price,
    //             },
    //             0,
    //         )
    //         .expect("Error in reply [MarketEvent::TokensWithdrawn]");
    //     } else {
    //         panic!("The offer with that hash does not exist");
    //     }
    // }
}
