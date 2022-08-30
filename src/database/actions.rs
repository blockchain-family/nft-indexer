use crate::database::records::*;

use super::records::AuctionActiveRecord;

impl Put for AuctionDeployedRecord {}

// pub async fn get_limit_order_state_changed_records(
//     pool: &PgPool,
//     request: AuctionDeployedRequest,
// ) -> Result<(Vec<AuctionDeployedRecord>, i64)> {
//     todo!()
// }

impl Put for AuctionActiveRecord {}

impl Put for AuctionDeclinedRecord {}

impl Put for AuctionOwnershipTransferredRecord {}

impl Put for BidPlacedRecord {}

impl Put for BidDeclinedRecord {}

impl Put for AuctionCompleteRecord {}

impl Put for AuctionCancelledRecord {}

impl Put for DirectBuyDeployedRecord {}

impl Put for DirectBuyDeclinedRecord {}

impl Put for DirectBuyOwnershipTransferredRecord {}

impl Put for DirectSellDeployedRecord {}

impl Put for DirectSellDeclinedRecord {}

impl Put for DirectSellOwnershipTransferredRecord {}

impl Put for DirectBuyStateChangedRecord {}

impl Put for DirectSellStateChangedRecord {}
