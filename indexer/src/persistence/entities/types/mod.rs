use indexer_repo::types::decoded::*;

pub enum Decoded {
    ShouldSkip,
    CreateNft(NftCreated),
    BurnNft(NftBurned),
    OwnerChangedNft(AddressChanged),
    ManagerChangedNft(AddressChanged),
    AuctionDeployed(String),
    AuctionCreated(AddressChanged),
    AuctionActive((AuctionActive, NftPriceHistory)),
    AuctionBidPlaced((AuctionBid, NftPriceHistory)),
    AuctionBidDeclined(AuctionBid),
    AuctionComplete(AuctionComplete),
    AuctionCancelled(AuctionCancelled),
    RawEventRecord(EventRecord),
    AuctionRulesChanged(CollectionFee),
    DirectBuyStateChanged((DirectBuy, NftPriceHistory)),
    DirectSellStateChanged((DirectSell, NftPriceHistory)),
    DirectBuyDeployed(String),
    DirectSellDeployed(String),
}
