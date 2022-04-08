import {network} from "redspot";
import uploadTokenCode from "./01_token_code";
import deployProxy from "./02_proxy";
import setup from "./03_setup";
import {encodeAddress} from "@polkadot/util-crypto";

const {api} = network;

async function run() {
  await api.isReady;
  const tokenWasmHash = await uploadTokenCode(api);
  const proxyAddress = await deployProxy(api, tokenWasmHash);
  await setup(api, encodeAddress(proxyAddress.address));

  await api.disconnect();
}

run()
  .then(() => console.log('Deployed successfully.'))
  .catch((e) => { console.error('Unexpected error during deployment: ' + e.message); process.exit(1)})