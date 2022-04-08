import { expect } from "chai";
import { artifacts, network, patract } from "redspot";
import {parseFixed} from "@ethersproject/bignumber";


const { getContractFactory, getRandomSigner } = patract;

const { api, getSigners } = network;

describe("FULA token", () => {
  after(() => {
    return api.disconnect();
  });

  async function setup() {
    await api.isReady
    const signers = await getSigners();
    const defaultSigner = signers[0];
    const contractFactory = await getContractFactory("fula_token", defaultSigner.address);
    const contract = await contractFactory.deploy("new", parseFixed('1000000', 18));
    const abi = artifacts.readArtifact("erc20");
    const receiver = await getRandomSigner();

    return { contractFactory, contract, abi, receiver, defaultSigner };
  }

  it("Assigns initial balance", async () => {
    const { contract, defaultSigner } = await setup();
    const result = await contract.query['psp22::balanceOf'](defaultSigner.address);
    expect(result.output).to.equal(parseFixed('1000000', 18));
  });

  it("Transfer adds amount to destination account", async () => {
    const { contract, receiver } = await setup();
    const receiverBalanceBefore = (await contract.query["psp22::balanceOf"](receiver.address)).output
    await contract.tx['psp22::transfer'](receiver.address, 7, [])
    const receiverBalanceAfter = (await contract.query["psp22::balanceOf"](receiver.address)).output

    // @ts-ignore
    expect(receiverBalanceAfter).to.equal(+receiverBalanceBefore + 7)
  });

  it("Transfer emits event", async () => {
    const { contract, defaultSigner, receiver } = await setup();

    await expect(contract.tx['psp22::transfer'](receiver.address, 7, []))
      .to.emit(contract, "Transfer")
      .withArgs(defaultSigner.address, receiver.address, 7);
  });

  // it("Can not transfer from empty account", async () => {
  //   const { contract, Alice, sender } = await setup();
  //
  //   const emptyAccount = await getRandomSigner(Alice, "10 UNIT");
  //
  //   await expect(
  //     contract.connect(emptyAccount).tx.transfer(sender.address, 7)
  //   ).to.not.emit(contract, "Transfer");
  // });
});
