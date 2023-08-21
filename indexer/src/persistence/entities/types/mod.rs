use indexer_repo::types::decoded::*;

pub enum Decoded {
    ShouldSkip,
    CreateNft(NftCreated),
    BurnNft(NftBurned),
    OwnerChangedNft(AddressChanged),
    ManagerChangedNft(AddressChanged),
    AuctionDeployed(AuctionDeployed),
    AuctionActive(AuctionActive),
    AuctionBidPlaced(AuctionBid),
    AuctionBidDeclined(AuctionBid),
    AuctionComplete((AuctionComplete, NftPriceHistory)),
    AuctionCancelled(AuctionCancelled),
    RawEventRecord(EventRecord),
    AuctionRulesChanged(CollectionFee),
    DirectBuyStateChanged((DirectBuy, Option<NftPriceHistory>)),
    DirectSellStateChanged((DirectSell, Option<NftPriceHistory>)),
}
