import BN from 'bn.js';
import { expect } from 'chai';
import { patract, network, artifacts } from 'redspot';

const { getContractFactory, getRandomSigner } = patract;

const { api, getSigners } = network;

describe('Credit', () => {
  after(() => {
    return api.disconnect();
  });

  async function setup() {
    const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]));
    const signers = await getSigners();
    const Alice = signers[0];
    const sender = await getRandomSigner(Alice, one.muln(1000));
    const contractFactory = await getContractFactory('credit', sender);
    const contract = await contractFactory.deploy('new');
    const abi = artifacts.readArtifact('credit');
    const receiver = await getRandomSigner();

    return { sender, contractFactory, contract, abi, receiver, Alice, one };
  }

  it('Get the btc and eth balance', async () => {
    const { contract, sender } = await setup();
    const result = await contract.tx.balanceOf(sender.address);
    console.log("result is", result);
    expect(result["result"]["dispatchError"]).to.equal(undefined);

    const balances = await contract.query.get();

    const btc_balance = balances.output[0].toString();
    expect(btc_balance).to.equal("1");

    const eth_balance = balances.output[1].toString();
    expect(eth_balance).to.equal("1");
  });

});
