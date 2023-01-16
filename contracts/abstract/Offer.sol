pragma ever-solidity >= 0.61.2;

pragma AbiHeader expire;
pragma AbiHeader pubkey;
pragma AbiHeader time;

import '../errors/BaseErrors.sol';
import '../errors/OffersBaseErrors.sol';

import '../interfaces/IOffersRoot.sol';
import "../interfaces/IOffer.sol";

abstract contract Offer is IOffer {

    uint64 static nonce_;
    address public static nft;

    uint128 public price;
    address public markerRootAddr;
    address public tokenRootAddr;
    address public nftOwner;

    uint128 public deploymentFee;
    // Market fee
    MarketFee fee;

    function setDefaultProperties(
        uint128 _price,
        address _markerRootAddr,
        address _tokenRootAddr,
        address _nftOwner,
        uint128 _deploymentFee,
        MarketFee _fee
    ) 
        internal 
    {   
        price = _price;
        markerRootAddr = _markerRootAddr;
        tokenRootAddr = _tokenRootAddr;
        nftOwner = _nftOwner;
        deploymentFee = _deploymentFee;
        fee = _fee;
    }

    modifier onlyOwner() {
        require(
            msg.sender.value != 0 &&
            msg.sender == nftOwner, 
            BaseErrors.message_sender_is_not_my_owner
        );

        _;
    }

    modifier onlyMarketRoot() {
        require(
            msg.sender.value != 0 && 
            msg.sender == markerRootAddr, 
            OffersBaseErrors.message_sender_is_not_my_root
        );

        _;
    }

    function getMarketFee() external view override returns (MarketFee) {
        return fee;
    }

    function setMarketFee(MarketFee _fee) external override onlyMarketRoot {
        require(_fee.denominator > 0, BaseErrors.denominator_not_be_zero);
        fee = _fee;
    }
}
