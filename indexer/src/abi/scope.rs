pub fn events() -> Vec<&'static str> {
    vec![
        /* AuctionRootTip3 */
        "AuctionDeployed",
        "AuctionDeclined",
        /* AuctionTip3 */
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
