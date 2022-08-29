use bigdecimal::BigDecimal;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectSellStateChangedRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub from: i16,
    pub to: i16,

    pub factory: String,
    pub creator: String,
    pub token: String,
    pub nft: String,
    pub _time_tx: i128,
    pub start: i128,
    pub end: i128,
    pub _price: BigDecimal,
    pub wallet: String,
    pub status: i16,
    pub sender: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuctionActiveRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub auction_subject: String,
    pub subject_owner: String,
    pub payment_token_root: String,
    pub wallet_for_bids: String,
    pub start_time: i128,
    pub duration: i128,
    pub finish_time: i128,
    pub now_time: i128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuctionDeployedRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub offer_address: String,

    pub collection: String,
    pub nft_owner: String,
    pub nft: String,
    pub offer: String,
    pub price: BigDecimal,
    pub auction_duration: BigDecimal,
    pub deploy_nonce: i128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuctionDeclinedRecord {
    pub account_addr: String,
    pub created_lt: i128,
    pub created_at: i64,

    pub nft_owner: String,
    pub data_address: String,
}
