## Fula `ink!`-based token

### What this is

This is a project containing a PSP-22 (ERC-20 analogue) token, written in 
`ink!` language and compiled to WASM. Additionally, this project contains basic
tests for the token, as well as convenient deploy scripts for deployment to different networks.

### Prerequisites
node.js, yarn/npm, [Rust+cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html),
[`cargo-contract`](https://github.com/paritytech/cargo-contract#installation).

### Workflow

The workflow is as follows:
1. Develop your contracts, add some logic in `contracts/` 
2. Build them (`yarn build`)
3. Test them (`yarn test`)
4. Deploy them (`yarn deploy`)

You can specify the deployment network via `--network` argument. Possible values:
`development`, `shibuya`, potentially others.

### Caveats

This token is implemented as an upgradeable token, which might not be desirable
for a token in general. BUT, it's easier to deploy this token as non-upgradeable
than to add upgradeablility to it :)

By default, the deploy scripts upload token code onto the chain, then deploy
the proxy contract, which does delegated calls to the token code.
In the `setup` file we specify everything for our setup. This includes the initialization
of the token.

If you for some reason want to deploy the token as non-upgradeable one, you should:
1. Remove the `initialize` function from the `fula_token` contract
2. Rework [token code upload](scripts/01_token_code.ts) to actually upload the code
AND instantiate (done with `contractFactory.deploy()` method)
3. Comment out proxy deployment [here](scripts/03_setup.ts).

## You can interact with the deployed contracts [here](https://polkadot.js.org/apps/#/contracts).