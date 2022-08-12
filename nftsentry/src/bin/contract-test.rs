use std::fs;
use clap::Parser;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{AccountId, testing_env, Balance};
use near_sdk::env;
use nftsentry::{Contract, JsonToken, NFTContractMetadata, NonFungibleTokenMetadata, TokenMetadata};
use near_sdk::serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};
use nftsentry::approval::NonFungibleTokenCore;
use nftsentry::nft_core::NonFungibleTokenCore as NFTCore;

/// Simple contract management tool
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short='a', long)]
    action: String,

    /// Token ID to mint/transfer
    #[clap(long="token-id", default_value = "0")]
    token_id: String,

    #[clap(long="base-uri")]
    base_uri: Option<String>,

    #[clap(long="name", default_value = "Test Contract")]
    name: String,

    #[clap(long="symbol", default_value = "SENTRY")]
    symbol: String,

    #[clap(long="owner-id", default_value = "nftsentry.testnet")]
    owner_id: String,

    #[clap(long="account-id", default_value = "")]
    account_id: String,

    #[clap(long="approval-id")]
    approval_id: Option<u64>,

    #[clap(long="new-owner-id", default_value = "ponchik.testnet")]
    new_owner_id: String,

    #[clap(long="media")]
    media: Option<String>,

    #[clap(short, long="description", default_value = "The tallest mountain in the charted solar system")]
    description: String,
}

const MINT_STORAGE_COST: u128 = 68900000000000000000000;

fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .current_account_id(accounts(2))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);
    builder
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractData {
    metadata: NFTContractMetadata,
    owner_id: AccountId,
}

fn save_state(contract: &Contract) {
    let filename = "contract.json";
    let filename_tokens = "tokens.json";

    let contract_data = ContractData{
        owner_id: contract.owner_id.clone(),
        metadata: contract.metadata.get().unwrap(),
    };
    let tokens = contract.nft_tokens(None, None, None);

    let mut f = File::create(filename).expect("create failed");
    let mut f_tokens = File::create(filename_tokens).expect("create failed");

    f.write(&serde_json::to_vec(&contract_data).unwrap()).expect("write failed");
    f_tokens.write(&serde_json::to_vec(&tokens).unwrap()).expect("write failed");
    // Close file
    drop(f);
    drop(f_tokens);
}

fn load_state(filename: &str) -> Contract {
    let filename_tokens = "tokens.json";
    if !Path::new(filename).exists() {

    }
    let mut f = File::open(filename).expect("open failed");
    let mut f_tokens = File::open(filename_tokens).expect("open failed");
    let metadata = fs::metadata(filename).expect("unable to read metadata");
    let metadata_tokens = fs::metadata(filename_tokens).expect("unable to read metadata");

    let mut buffer = vec![0; metadata.len() as usize];
    let mut buffer_tokens = vec![0; metadata_tokens.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    f_tokens.read(&mut buffer_tokens).expect("buffer overflow");

    let contract_data: ContractData = serde_json::from_slice(&buffer).unwrap();
    let mut read_contract= Contract::new(
        contract_data.owner_id,
        contract_data.metadata,
    );
    read_contract.disable_events = true;

    let tokens: Vec<JsonToken> = serde_json::from_slice(&buffer_tokens).unwrap();
    for token in tokens {
        read_contract.nft_mint(
            token.token_id.clone(),
            "id".to_string(),
            token.metadata,
            token.owner_id,
            None,
            None,
        );
        let mut t = read_contract.tokens_by_id.get(&token.token_id).unwrap();
        t.approved_account_ids = token.approved_account_ids.clone();
        if token.approved_account_ids.len() > 0 {
            // Override next_approval_id with maximum
            let next = *token.approved_account_ids.iter()
                .max_by_key(|&(&_, &b)| b).unwrap().1 + 1;
            t.next_approval_id = next;

            read_contract.tokens_by_id.remove(&token.token_id.clone());
            read_contract.tokens_by_id.insert(&token.token_id.clone(), &t);
        }
    }
    // let tokens = read_contract.nft_token("2".to_string());
    // println!("{:?}", tokens);

    // Close file
    drop(f);
    drop(f_tokens);

    read_contract
}

// fn clear_state(filename: &str) {
//     fs::remove_file(filename).expect("Delete file failed")
// }

fn prepare_ctx(deposit: Balance) {
    let mut context = get_context(accounts(0));
    testing_env!(context
    .storage_usage(env::storage_usage())
    .attached_deposit(deposit)
    .predecessor_account_id(accounts(0))
    .build());
}

fn init_contract(args: &Args, filename: Option<String>) -> Contract {
    // testing_env!(context.build());
    prepare_ctx(MINT_STORAGE_COST);

    let mut contract: Contract;
    if let Some(filename) = filename {
        contract = load_state(&filename);
    } else {
        contract = Contract::new(
            AccountId::new_unchecked(args.owner_id.to_string()),
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: args.name.to_string(),
                symbol: args.symbol.to_string(),
                icon: None,
                base_uri: args.base_uri.clone(),
                reference: None,
                reference_hash: None,
            }
        );
        contract.disable_events = true;
    }
    // let mut contract = Contract::new_default_meta(accounts(0).into());
    contract
}

fn test_mint(args: Args, contract: &mut Contract) {
    let token_id = args.token_id.to_string();

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let token = contract.nft_mint(
        token_id.clone(),
        "id".to_string(),
        TokenMetadata{
            title: Some(args.name),
            description: Some(args.description),
            media: args.media,
            media_hash: None,
            copies: Some(1u64),
            issued_at: Some(since_the_epoch.as_millis() as u64),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None
        },
        AccountId::new_unchecked(args.owner_id),
        None,
        None,
    );
    println!("{}", serde_json::to_string(&token).unwrap());
}

fn test_transfer(args: Args, contract: &mut Contract) {
    contract.nft_transfer(
        AccountId::new_unchecked(args.new_owner_id),
        args.token_id.to_string(),
        None,
        None,
    );
    let new_token = contract.nft_token(args.token_id.to_string());
    println!("{}", serde_json::to_string(&new_token).unwrap());
}

fn test_approve(args: Args, contract: &mut Contract) {
    contract.nft_approve(
        args.token_id.to_string(),
        AccountId::new_unchecked(args.account_id),
        None,
    );
    let new_token = contract.nft_token(args.token_id.to_string());
    println!("{}", serde_json::to_string(&new_token).unwrap());
}

fn test_is_approved(args: Args, contract: &mut Contract) {
    let approved = contract.nft_is_approved(
        args.token_id.to_string(),
        AccountId::new_unchecked(args.account_id),
        args.approval_id,
    );
    println!("{}", approved);
}

fn main() {
    let args = Args::parse();
    let filename = "contract.json";

    let mut contract: Contract;
    if args.action == "new" {
        contract = init_contract(&args, None);
        println!("{}", serde_json::to_string(&contract.nft_metadata()).unwrap());
    } else {
        contract = init_contract(&args, Some(filename.to_string()));
    }

    if args.action == "mint" {
        test_mint(args.clone(), &mut contract);
    } else if args.action == "transfer" {
        prepare_ctx(1);
        test_transfer(args.clone(), &mut contract);
    } else if args.action == "approve" {
        test_approve(args.clone(), &mut contract);
    } else if args.action == "is_approved" {
        test_is_approved(args.clone(), &mut contract);
    } else if args.action == "view" {

    } else if args.action == "nft_metadata" {
        println!("{}", serde_json::to_string(&contract.nft_metadata()).unwrap());
    } else if args.action == "nft_tokens" {
        println!("{}", serde_json::to_string(&contract.nft_tokens(None, None, None)).unwrap());
    } else if args.action == "nft_tokens_for_owner" {
        println!("{}", serde_json::to_string(&contract.nft_tokens_for_owner(
            AccountId::new_unchecked(args.account_id), None, None)
        ).unwrap());
    } else if args.action == "nft_token" {
        let token = contract.nft_token(args.token_id.clone());
        if let Some(token) = token {
            println!("{}", serde_json::to_string(&token).unwrap())
        } else {
            println!("Token {} does not exist!", args.token_id);
            exit(1);
        }
    }
    save_state(&contract);
}