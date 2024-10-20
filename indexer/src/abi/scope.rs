pub fn events() -> Vec<&'static str> {
    vec![
        /* FactoryAuction */
        "AuctionDeployed",
        "AuctionDeclined",
        /* Auction */
        "AuctionCreated",
        "AuctionActive",
        "BidPlaced",
        "BidDeclined",
        "AuctionComplete",
        "AuctionCancelled",
        /* Collection */
        "NftCreated",
        "NftBurned",
        /* DirectBuy */
        "DirectBuyStateChanged",
        /* DirectSell */
        "DirectSellStateChanged",
        /* FactoryDirectBuy */
        "DirectBuyDeployed",
        "DirectBuyDeclined",
        /* FactoryDirectSell */
        "DirectSellDeployed",
        "DirectSellDeclined",
        /* Nft */
        "ManagerChanged",
        "OwnerChanged",
        /* Collection 4.2.2 */
        "NftMetadataUpdated",
        "CollectionMetadataUpdated",
        /* Nft 4.2.2 */
        "MetadataUpdated",
        /* common for all events */
        "OwnershipTransferred",
        "MarketFeeDefaultChanged",
        "MarketFeeChanged",
        "AddCollectionRules",
        "RemoveCollectionRules",
    ]
}

pub fn functions() -> Vec<&'static str> {
    vec![]
}
