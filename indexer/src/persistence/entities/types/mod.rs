use indexer_repo::types::{
    Address, AddressChangedDecoded, AuctionActiveDecoded, AuctionBidDecoded,
    AuctionCancelledDecoded, AuctionCompleteDecoded, CollectionFeeDecoded, DirectBuyDecoded,
    DirectSellDecoded, EventRecord, NftBurnedDecoded, NftCreateDecoded, NftPriceHistory,
};

pub enum Decoded {
    ShouldSkip,
    CreateNft(NftCreateDecoded),
    BurnNft(NftBurnedDecoded),
    OwnerChangedNft(AddressChangedDecoded),
    ManagerChangedNft(AddressChangedDecoded),
    AuctionDeployed(Address),
    AuctionCreated(AddressChangedDecoded),
    AuctionActive((AuctionActiveDecoded, NftPriceHistory)),
    AuctionBidPlaced((AuctionBidDecoded, NftPriceHistory)),
    AuctionBidDeclined(AuctionBidDecoded),
    AuctionComplete(AuctionCompleteDecoded),
    AuctionCancelled(AuctionCancelledDecoded),
    RawEventRecord(EventRecord),
    AuctionRulesChanged(CollectionFeeDecoded),
    DirectBuyStateChanged((DirectBuyDecoded, NftPriceHistory)),
    DirectSellStateChanged((DirectSellDecoded, NftPriceHistory)),
    DirectBuyDeployed(Address),
    DirectSellDeployed(Address),
}
