import { patract, network } from "redspot";
import {U8aFixed} from "@polkadot/types-codec";
import {u8aToHex} from "@polkadot/util";

const { getContractFactory } = patract;
const { getSigners } = network;

export default async function uploadTokenCode(api): Promise<U8aFixed> {
  const signer = (await getSigners())[0]

  const tokenFactory = await getContractFactory("fula_token", signer);
  // dont believe the hinted signatures!
  await api.tx.contracts.uploadCode(u8aToHex(tokenFactory.wasm), null).signAndSend(signer.pair, async (res) => {
    if (res.status.isInBlock) {
      console.log('Uploaded code successfully.');
    }
  })
  const codeHash = tokenFactory.abi.info.source.wasmHash;
  console.log('TOKEN CODE HASH: ' + u8aToHex(codeHash))
  return codeHash;

}
