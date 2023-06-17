pragma ever-solidity >= 0.61.2;

import "../../errors/BaseErrors.sol";
import "../../interfaces/IEventsMarketFeeOffer.sol";
import "../../abstract/BaseRoot.sol";
import "./MarketFeeOffer.sol";
import "../../libraries/Gas.sol";

abstract contract MarketFeeRoot is BaseRoot {

    function setMarketFeeForChildContract(
        address _offer,
        MarketFee _fee
    )
        external
        onlyOwner
    {
        require(_fee.denominator > 0, BaseErrors.denominator_not_be_zero);
        MarketFeeOffer(_offer).setMarketFee{value: 0, flag: 64, bounce: false}(_fee, msg.sender);
    }

    function marketFee() external view returns (MarketFee) {
        return _getMarketFee();
    }

    function setMarketFee(
        MarketFee _fee
    )
        external
        onlyOwner
        reserve
    {
        require(_fee.denominator > 0, BaseErrors.denominator_not_be_zero);
        _setMarketFee(_fee);
        msg.sender.transfer({ value: 0, flag: 128 + 2, bounce: false });
    }

    function withdraw(
        address _tokenWallet,
        uint128 _amount,
        address _recipient,
        address _remainingGasTo
    )
        external
        onlyOwner
        reserve
    {
        require(_recipient.value != 0,  BaseErrors.wrong_recipient);
        require(msg.value >= Gas.WITHDRAW_VALUE, BaseErrors.low_gas);
        TvmCell emptyPayload;
        ITokenWallet(_tokenWallet).transfer{
            value: 0,
            flag: 128,
            bounce: false
        }(
            _amount,
            _recipient,
            Gas.DEPLOY_EMPTY_WALLET_GRAMS,
            _remainingGasTo,
            false,
            emptyPayload
        );
        emit MarketFeeWithdrawn(_recipient, _amount, _tokenWallet);
        msg.sender.transfer({ value: 0, flag: 128 + 2, bounce: false });
    }
}