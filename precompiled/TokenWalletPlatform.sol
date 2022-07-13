pragma ton-solidity >= 0.57.0;

contract TokenWalletPlatform {
    address static root;
    address static owner;

    constructor(TvmCell walletCode, uint32 walletVersion, address sender, address remainingGasTo)
        public
        functionID(0x15A038FB)
    {
        if (msg.sender == root || (sender.value != 0 && _getExpectedAddress(sender) == msg.sender)) {
           initialize(walletCode, walletVersion, remainingGasTo);
        } else {
            remainingGasTo.transfer({
                value: 0,
                flag: 128 + 32,
                bounce: false
            });
        }
    }

    function _getExpectedAddress(address owner_) private view returns (address) {
        TvmCell stateInit = tvm.buildStateInit({
            contr: TokenWalletPlatform,
            varInit: {
                root: root,
                owner: owner_
            },
            pubkey: 0,
            code: tvm.code()
        });

        return address(tvm.hash(stateInit));
    }

    function initialize(TvmCell walletCode, uint32 walletVersion, address remainingGasTo) private {
        TvmBuilder builder;

        builder.store(root);
        builder.store(owner);
        builder.store(uint128(0));
        builder.store(uint32(0));
        builder.store(walletVersion);
        builder.store(remainingGasTo);

        builder.store(tvm.code());

        tvm.setcode(walletCode);
        tvm.setCurrentCode(walletCode);

        onCodeUpgrade(builder.toCell());
    }

    function onCodeUpgrade(TvmCell data) private {}
}
