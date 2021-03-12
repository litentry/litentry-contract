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
    const sender = await getRandomSigner(Alice, one.muln(10000));
    const contractFactory = await getContractFactory('credit', sender);
    const contract = await contractFactory.deploy('new', '1000');
    const abi = artifacts.readArtifact('credit');
    const receiver = await getRandomSigner();

    return { sender, contractFactory, contract, abi, receiver, Alice, one };
  }

  it('Assigns initial balance', async () => {
    const { contract, sender } = await setup();
    const get = await contract.query.get();
    expect(get.output).to.equal("");
    await contract.tx.balance_of();
    const second_get = await contract.query.get();
    expect(second_get.output).to.equal("");
  });

});
