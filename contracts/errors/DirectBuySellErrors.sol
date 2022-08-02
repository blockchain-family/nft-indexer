pragma ton-solidity >=0.62.0;

library DirectBuySellErrors {
    uint16 constant NOT_FROM_SPENT_TOKEN_ROOT     = 300;
    uint16 constant NOT_SPENT_WALLET_TOKEN        = 301;
    uint16 constant NOT_NFT_OWNER                 = 302;
    uint16 constant NOT_NFT_MANAGER               = 303;
    uint16 constant NOT_CORRECT_ADDRESS_NFT       = 304;
    uint16 constant NOT_OWNER_DIRECT_BUY          = 305;
    uint16 constant NOT_ACTIVE_CURRENT_STATUS     = 306;
    uint16 constant NOT_FACTORY_MSG_SENDER_NFT    = 307;
    uint16 constant VALUE_TOO_LOW                 = 308;
    uint16 constant DIRECT_SELL_IN_STILL_PROGRESS = 309;
    uint16 constant NO_NFT                        = 310;
}