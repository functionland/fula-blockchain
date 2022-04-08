import { RedspotUserConfig } from "redspot/types";
import "@redspot/patract";
import "@redspot/chai";

import * as dotenv from 'dotenv';

dotenv.config({path: __dirname + '/.env'})


export default {
  defaultNetwork: "development",
  contract: {
    ink: {
      docker: false,
      toolchain: "nightly",
      sources: ["contracts/**/*"],
    },
  },
  networks: {
    development: {
      endpoint: "ws://127.0.0.1:9944",
      gasLimit: "400000000000",
      accounts: ["//Alice", "//Bob", "//Charlie"],
      types: {},
    },
    shibuya: {
      endpoint: "wss://rpc.shibuya.astar.network",
      gasLimit: "400000000000",
      accounts: [process.env.FULA_DEPLOYER_MNEMONIC],
      types: {},
    },
  },
  mocha: {
    timeout: 60000,
  },
  docker: {
    sudo: false,
    runTestnet:
      "docker run -p 9944:9944 --rm redspot/contract /bin/bash -c 'canvas --rpc-cors all --tmp --dev --ws-port=9944 --ws-external'",
  },
} as RedspotUserConfig;
