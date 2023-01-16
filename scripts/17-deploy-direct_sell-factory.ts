import { Migration } from "./migration";
import { isValidEverAddress} from "../test/utils";
import {WalletTypes} from "locklift";

const prompts = require('prompts');
const migration = new Migration();

async function main() {
    const response = await prompts([
        {
            type: 'text',
            name: 'owner',
            message: 'FactoryDirectSell owner',
            validate: (value:any) => isValidEverAddress(value) || value === '' ? true : 'Invalid Everscale address'
        }
    ]);

    const signer = (await locklift.keystore.getSigner('0'));
    const account = await locklift.factory.accounts.addExistingAccount({
        type: WalletTypes.EverWallet,
        address: migration.getAddress('Account1')
    });

    const {contract: factoryDirectSell, tx } = await locklift.factory.deployContract({
        contract: "FactoryDirectSell",
        publicKey: (signer?.publicKey) as string,
        constructorParams: {
            _owner: account.address,
            sendGasTo: account.address
        },
        initParams: {
            nonce_: Math.random() * 6400 | 0
        },
        value: locklift.utils.toNano(10)
    });

    console.log(`FactoryDirectSell: ${factoryDirectSell.address}`);
    migration.store(factoryDirectSell.address, "FactoryDirectSell", "FactoryDirectSell");
    const DirectSell = (await locklift.factory.getContractArtifacts('DirectSell'));

    console.log(`Set code DirectSell`);
    await factoryDirectSell.methods.setCodeDirectSell({
        _directSellCode: DirectSell.code,
    }).send({
        from: account.address,
        amount: locklift.utils.toNano(1)
    })

    if (response.owner) {
        console.log(`Transfer ownership`);
        await factoryDirectSell.methods.transferOwnership({
            newOwner: response.owner
        }).send({
            from: account.address,
            amount: locklift.utils.toNano(1)
        })
    }
}

main()
    .then(() => process.exit(0))
    .catch(e => {
        console.log(e);
        process.exit(1);
    });
