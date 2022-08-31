import { Address, Contract, zeroAddress } from "locklift";
import { FactorySource } from "../../build/factorySource";
import {Account} from "locklift/build/factory";
import {CallbackType} from "../utils";

declare type AccountType = Account<FactorySource["Wallet"]>

export class NftC {
    public contract: Contract<FactorySource["Nft"]>;
    public owner: AccountType;
    public address: Address;

    constructor(nft_contract: Contract<FactorySource["Nft"]>, nft_owner: AccountType) {
        this.contract = nft_contract;
        this.owner = nft_owner;
        this.address = this.contract.address;
    }

    static async from_addr(addr: Address, owner: AccountType) {
        const contract = await locklift.factory.getDeployedContract('Nft', addr);
        return new NftC(contract, owner);
    }

    async changeManager(initiator: AccountType, newManager: Address, sendGasTo: Address, callbacks: CallbackType[]) {
        return await 
        // locklift.tracing.trace(
            initiator.runTarget(
            {
                contract: this.contract,
                value: locklift.utils.toNano(6),
                flags: 1
            },
            (dd) => dd.methods.changeManager({
                newManager,
                sendGasTo: sendGasTo.toString() == zeroAddress ? this.owner.address: sendGasTo,
                callbacks       
            })
        // )
        );
    }

    async getEvents(event_name: string) {
        return (await this.contract.getPastEvents({filter: (event) => event.event === event_name})).events;
    }
    
    async getEvent(event_name: string) {
        const last_event = (await this.getEvents(event_name)).shift();
        if (last_event) {
            return last_event.data;
        }
        return null;
    }
}