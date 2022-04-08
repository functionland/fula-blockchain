mod block;
mod config;

use crate::block::Block;
use crate::substrate_contracts_node::RuntimeApi;
use envconfig::Envconfig;
use log::{debug, error, info};
use serde_json::json;
// ignore the highlighted errors here
#[cfg(feature = "ecdsa")]
use sp_core::ecdsa::Pair;
#[cfg(feature = "ed25519")]
use sp_core::ed25519::Pair;
#[cfg(feature = "sr25519")]
use sp_core::sr25519::Pair;

use subxt::sp_core::blake2_256;
use subxt::sp_core::crypto::Ss58Codec;
use subxt::sp_core::Pair as PairT;
use subxt::sp_runtime::scale_info::scale;
use subxt::sp_runtime::AccountId32;
use subxt::Config as ConfigT;
use subxt::{sp_core, ClientBuilder, DefaultConfig, DefaultExtra, PairSigner};

#[subxt::subxt(runtime_metadata_path = "chain_metadata/substrate_contracts_node_local.scale")]
pub mod substrate_contracts_node {}

// TODO: add validation logic beforehand
// returns either BlockConstructionError or subxt error
async fn submit_block<PairType: PairT>(
    api: RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>,
    signer: PairSigner<DefaultConfig, DefaultExtra<DefaultConfig>, PairType>,
    contract: &str,
    block: Block,
) -> Result<(), Box<dyn std::error::Error>>
where
    PairType::Signature: Into<<DefaultConfig as ConfigT>::Signature>,
{
    let tx_vector = block.txs();
    let tx_vector_len = tx_vector.len();
    debug!("Received {} txs in block", tx_vector_len);
    let extrinsics = tx_vector.iter().filter_map(|tx| {
        // TODO: filter the extrinsics which ONLY originate from this node
        let mut call_data = Vec::<u8>::new();
        //append the selector
        call_data.append(&mut (&blake2_256("PSP22::transfer".as_bytes())[0..4]).to_vec());
        //append the arguments
        call_data.append(&mut scale::Encode::encode(&(
            AccountId32::from_string(&tx.recipient_node_maintainer).ok()?,
            &tx.value,
            Vec::<u8>::new(),
        )));
        Some(api.tx().contracts().call(
            AccountId32::from_string(contract).ok()?.into(),
            0,
            20_000_000_000,
            None,
            call_data,
        ))
    });

    let xts = extrinsics
        .map(|xt| xt.sign_and_submit(&signer))
        .collect::<Vec<_>>();

    debug!("Out of them valid: {}", xts.len());
    info!("Submitting {} extrinsic(s) to {}", xts.len(), contract);

    let results = futures::future::join_all(xts).await;
    results
        .into_iter()
        .filter(|res| res.is_err())
        .for_each(|err| error!("Error submitting extrinsic: {:?}", err.unwrap_err()));

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env").unwrap_or_else(|_err| {
        dotenv::from_filename(".env.sample").expect("Missing .env or .env.sample!")
    });
    env_logger::init();
    let config = crate::config::ClientConfig::init_from_env()?;
    let node_url = config.node_url;

    let signer = PairSigner::new(
        Pair::from_string(
            &config.node_maintainer_phrase,
            config.node_maintainer_password.as_deref(),
        )
        .expect("Invalid phrase provided"),
    );

    info!("Started client service. Connecting to node at {}", node_url);

    let api = ClientBuilder::new()
        .set_url(node_url)
        .build()
        .await?
        .to_runtime_api::<substrate_contracts_node::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

    let test_block: Block = Block::from_json(json!(
        {
            "node_id": 1,
            "node_maintainer": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
            "txs": [{
                "author_node_id": 1,
                "author_node_maintainer": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                "recipient_node_id": 2,
                "recipient_node_maintainer": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                "tx_proof": "0x0000000000000000000000000000000000000000000000000000000000000000", // 0x-prefixed hex repr of a 256-bit hash,
                "value": "0x0000a0dec5adc9353600000000000000"
        }]
        }
    ))?;

    debug!("Signer: {}", subxt::Signer::account_id(&signer));

    // TODO: make contract configurable, make the configuration convenient
    submit_block(
        api,
        signer,
        "5FyG1SBTUkXSRMNGFRZ7F3zmrxVvZCPeZDMuVSrv4aaDr5Rw",
        test_block,
    )
    .await?;

    Ok(())
}
