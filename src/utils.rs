pub use ore_utils::AccountDeserialize;
use {
    cached::proc_macro::cached,
    drillx::Solution,
    ore_api::{
        consts::{BUS_ADDRESSES, CONFIG_ADDRESS, MINT_ADDRESS, PROOF, TOKEN_DECIMALS, TREASURY_ADDRESS},
        instruction,
        state::{Config, Proof, Bus},
    },
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{clock::Clock, instruction::Instruction, pubkey::Pubkey, sysvar},
    spl_associated_token_account::get_associated_token_address,
    std::{io::Cursor, time::Duration},
    tracing::{error, info},
    rand::Rng, // Rand is being used, so it's kept
};

pub const ORE_TOKEN_DECIMALS: u8 = TOKEN_DECIMALS;

pub fn get_auth_ix(signer: Pubkey) -> Instruction {
    let proof = proof_pubkey(signer);
    instruction::auth(proof)
}

pub fn get_mine_ix(signer: Pubkey, solution: Solution, bus: usize) -> Instruction {
    instruction::mine(signer, signer, BUS_ADDRESSES[bus], solution)
}

pub fn get_register_ix(signer: Pubkey) -> Instruction {
    instruction::open(signer, signer, signer)
}

pub fn get_reset_ix(signer: Pubkey) -> Instruction {
    instruction::reset(signer)
}

#[cached(time = 120)]  // Cache für 2 Minuten
pub fn proof_pubkey(authority: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[PROOF, authority.as_ref()], &ore_api::ID).0
}

#[cached(time = 120)]  // Cache für 2 Minuten
pub fn treasury_tokens_pubkey() -> Pubkey {
    get_associated_token_address(&TREASURY_ADDRESS, &MINT_ADDRESS)
}

pub async fn _get_config(client: &RpcClient) -> Result<ore_api::state::Config, String> {
    let mut retry_count = 0;
    let mut retry_delay = 500; // Start mit 500ms

    loop {
        if retry_count >= 5 {
            return Err("Maximale Anzahl von Wiederholungen erreicht.".to_string());
        }

        let data = client.get_account_data(&CONFIG_ADDRESS).await;
        match data {
            Ok(data) => {
                let config = Config::try_from_bytes(&data);
                if let Ok(config) = config {
                    return Ok(*config);
                } else {
                    return Err("Failed to parse config account data".to_string());
                }
            },
            Err(e) => {
                error!("Failed to get config account: {:?}", e);
                info!("Retrying to get config account...");
                retry_count += 1;
                tokio::time::sleep(Duration::from_millis(retry_delay)).await;
                retry_delay = std::cmp::min(retry_delay * 2, 8000); // Exponentieller Backoff, max 8 Sekunden
            }
        }
    }
}

// Entferne den #[cached] Makro von dieser Funktion, da RpcClient nicht als Cache-Schlüssel verwendet werden kann
pub async fn get_clock(client: &RpcClient) -> Clock {
    let data: Vec<u8>;
    loop {
        match client.get_account_data(&sysvar::clock::ID).await {
            Ok(d) => {
                data = d;
                break;
            },
            Err(e) => {
                error!("get clock account error: {:?}", e);
                info!("retry to get clock account...");
            },
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    bincode::deserialize::<Clock>(&data).expect("Failed to deserialize clock")
}

pub async fn get_config_and_proof(client: &RpcClient, authority: Pubkey) -> Result<(Config, Proof), String> {
    let config = _get_config(client).await?;
    let proof = get_proof(client, authority).await?;
    Ok((config, proof))
}

pub async fn get_proof(client: &RpcClient, authority: Pubkey) -> Result<Proof, String> {
    // Existierende Implementierung
    loop {
        let proof_address = proof_pubkey(authority);
        let data = client.get_account_data(&proof_address).await;
        match data {
            Ok(data) => {
                let proof = Proof::try_from_bytes(&data);
                if let Ok(proof) = proof {
                    return Ok(*proof);
                } else {
                    return Err("Failed to parse proof account data".to_string());
                }
            },
            Err(e) => {
                error!("Failed to get proof account: {:?}", e);
                info!("Retrying to get proof account...");
            },
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

pub async fn get_cutoff(rpc_client: &RpcClient, proof: Proof, buffer_time: u64) -> i64 {
    let clock = get_clock(rpc_client).await;
    proof.last_hash_at + 60 as i64 - buffer_time as i64 - clock.unix_timestamp
}

pub async fn get_cutoff_with_risk(
    rpc_client: &RpcClient,
    proof: Proof,
    buffer_time: u64,
    risk_time: u64,
) -> i64 {
    let clock = get_clock(rpc_client).await;
    proof.last_hash_at + 60 as i64 + risk_time as i64 - buffer_time as i64 - clock.unix_timestamp
}

pub fn play_sound() {
    match rodio::OutputStream::try_default() {
        Ok((_stream, handle)) => {
            let sink = rodio::Sink::try_new(&handle).unwrap();
            let bytes = include_bytes!("../assets/success.mp3");  // Der Standard-Sound
            let cursor = Cursor::new(bytes);
            sink.append(rodio::Decoder::new(cursor).unwrap());
            sink.sleep_until_end();
        },
        Err(_) => print!("\x07"),  // Fallback Sound
    }
}

pub fn play_high_difficulty_sound() {
    match rodio::OutputStream::try_default() {
        Ok((_stream, handle)) => {
            let sink = rodio::Sink::try_new(&handle).unwrap();
            let bytes = include_bytes!("../assets/high_difficulty_success.mp3");  // Der Sound für hohe Difficulty
            let cursor = Cursor::new(bytes);
            sink.append(rodio::Decoder::new(cursor).unwrap());
            sink.sleep_until_end();
        },
        Err(_) => print!("\x07"),  // Fallback Sound
    }
}

