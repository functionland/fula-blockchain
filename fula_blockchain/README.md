## Fula-on-polkadot L3 protocol

### What this is

This project contains the blockchain client and consensus services. 

### Configuration
1. Create a `.env` from the `.env.sample` file.
2. Create a new account / specify the existing one in the `.env`, fill out
all the environment variables.
3. Determine which keyring to use (you can check your private key with `subkey`).
4. Specify the L2 token address.

### Running
Run the following way:
```
cargo run --release --features <YOUR_KEYRING_TYPE>
```

The `<YOUR_KEYRING_TYPE>` can be either "ed25519", "sr25519", or "ecdsa".

### Connecting to a custom chain

To connect to a custom chain, do the following:
1. Install the `subxt` cli.
2. Generate metadata (guide [here](https://github.com/paritytech/subxt#downloading-metadata-from-a-substrate-node)).
3. Change `NODE_URL` in the `.env` file, as well as the accounts.

Then, run as usual.