import {network, patract} from "redspot";
import {FULA_TOKEN_SUPPLY} from "./constants";

const { getContractFactory } = patract;
const { getSigners } = network;

export default async function setup(api, fulaTokenAddress: string): Promise<void> {

  const signer = (await getSigners())[0]

  const tokenFactory = await getContractFactory("fula_token", signer);
  const contract = await tokenFactory.attach(fulaTokenAddress);

  await contract.tx.initialize(FULA_TOKEN_SUPPLY);

  console.log('Initialized FULA token at ' + contract.address);
}
