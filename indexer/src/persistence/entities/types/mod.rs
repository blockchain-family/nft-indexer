use indexer_repo::types::decoded::*;

#[derive(Debug, Clone)]
pub enum Decoded {
    ShouldSkip,
    CreateNft(NftCreated),
    BurnNft(NftBurned),
    OwnerChangedNft(AddressChanged),
    ManagerChangedNft(AddressChanged),
    MetadataUpdatedNft(MetadataUpdated),
    CollectionNftMetadataUpdated(NftMetadataUpdated),
    CollectionMetadataUpdated(CollectionMetadataUpdated),
    AuctionDeployed((AuctionDeployed, OfferDeployed)),
    AuctionActive(AuctionActive),
    AuctionBidPlaced(AuctionBid),
    AuctionBidDeclined(AuctionBid),
    AuctionComplete((AuctionComplete, NftPriceHistory)),
    AuctionCancelled(AuctionCancelled),
    RawEventRecord(EventRecord),
    AuctionRulesChanged(CollectionFee),
    DirectBuyDeployed((DirectBuy, OfferDeployed)),
    DirectBuyStateChanged((DirectBuy, Option<NftPriceHistory>)),
    DirectSellDeployed((DirectSell, OfferDeployed)),
    DirectSellStateChanged((DirectSell, Option<NftPriceHistory>)),
    RoyaltySet(SetRoyalty),
}
