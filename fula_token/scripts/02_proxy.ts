import {network, patract} from "redspot";
import Contract from "@redspot/patract/contract";
import {U8aFixed} from "@polkadot/types-codec";

const { getContractFactory } = patract;
const { getSigners } = network;

export default async function deployProxy(api, implementationWasmHash: U8aFixed): Promise<Contract> {
  const signer = (await getSigners())[0]

  const proxyFactory = await getContractFactory("fula_proxy", signer);
  const contract = await proxyFactory.deploy('new', implementationWasmHash);
  console.log('PROXY ADDRESS: ' + contract.address);
  return contract;
}
