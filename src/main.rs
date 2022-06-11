

use chrono::NaiveDateTime;
use core::time;
use std::thread;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::Signer;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signature::write_keypair_file;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_transaction_status::UiTransactionEncoding;
use regex::Regex;
use solana_transaction_status::{EncodedTransaction,UiTransaction,UiMessage};
use solana_transaction_status::parse_accounts::ParsedAccount;
use std::cmp;
use std::str::FromStr;
use std::str;

//const URL: &str = "https://api.devnet.solana.com";
//const URL: &str = "https://solana-api.projectserum.com";

const URL: &str = "https://api.mainnet-beta.solana.com";

//const URL: &str = "https://api.devnet.solana.com";
//const URL: &str = "https://api.testnet.solana.com";
use base64::{encode, decode};
use solana_account_decoder::{UiAccountEncoding, UiDataSliceConfig};
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, MemcmpEncoding, RpcFilterType};
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use solana_sdk::pubkey::Pubkey;

use sha2::{Sha256, Sha512, Digest};

use hex;

use mpl_candy_machine::state;
use solana_program::borsh::try_from_slice_unchecked;

//use solana_sdk::borsh::try_from_slice_unchecked;
use anchor_lang::prelude::*;
//use anchor_lang::solana_program::borsh::try_from_slice_unchecked;


fn main(){


//let candyMachineV2Acc = Pubkey::from_str("sPMA6U6bcsiUJ17rWFyT2Jv7M5zJAJVRDu7rfkKDLoL").unwrap();
    let candyMachineV2Acc = Pubkey::from_str("ErJ4Lmw3EZrudif8dHso7qtGFEqdGTW6TA5QGFAr28iP").unwrap();

    let rpc_client = solana_client::rpc_client::RpcClient::new_with_commitment(URL,CommitmentConfig::confirmed());

    let candyMachineV2account = rpc_client.get_account(&candyMachineV2Acc).unwrap();
    println!("candyMachineV2account key !{:?}",candyMachineV2account);


    let candy_machine: mpl_candy_machine::state::CandyMachine =
        mpl_candy_machine::state::CandyMachine::try_deserialize(&mut candyMachineV2account.data.as_slice()).unwrap();

  //  let candyMachineV2account_DATA = rpc_client.get_account_data(&candyMachineV2Acc).unwrap();

    // let mut new_data = CandyMachine::discriminator().try_to_vec().unwrap();
  //  let candy_machine: mpl_candy_machine::state::CandyMachine = mpl_candy_machine::state::CandyMachine::try_deserialize(&mut candyMachineV2account_DATA.as_slice()).unwrap();

    //let candy : mpl_candy_machine::state::CandyMachine = try_from_slice_unchecked(&candyMachineV2account_DATA).unwrap();
    println!("candy key !{:?}",candy_machine);


}
fn main____(){


//let candyMachineV2Acc = Pubkey::from_str("sPMA6U6bcsiUJ17rWFyT2Jv7M5zJAJVRDu7rfkKDLoL").unwrap();
    let candyMachineV2Acc = Pubkey::from_str("GZHXaULUZt89qpotnpHF2ftQXRYjvMYbHrhBskjjAXtL").unwrap();

let rpc_client = solana_client::rpc_client::RpcClient::new_with_commitment(URL,CommitmentConfig::confirmed());

let candyMachineV2account = rpc_client.get_account(&candyMachineV2Acc).unwrap();
    println!("candyMachineV2account key !{:?}",candyMachineV2account);

   // let mut new_data = CandyMachine::discriminator().try_to_vec().unwrap();

   let candy : mpl_candy_machine::state::CandyMachine = try_from_slice_unchecked(&candyMachineV2account.data).unwrap();
println!("candyMachineV2account key !{:?}",candyMachineV2account);


    let mut hasher = Sha256::new();

    hasher.update(b"account:CandyMachine");

// read hash digest and consume hasher
    let result = hasher.finalize();
    //hasher.finalize().to_owned()[..8]
    let base64_bytes_val =bs58::encode(&result.as_slice()[0..8]).into_string();
    //"33adb17119f16dbd"//bf86c0dc92d8dc5f37194ead2e4fc9
   // let encoded = bs58::encode(char).into_string();
    println!("base64_bytes_val key !{:?}",base64_bytes_val);

}







fn main__() {
    println!("Hello, world!");



// create a Sha256 object
    let mut hasher = Sha256::new();

// write input message
    hasher.update(b"account:CandyMachine");

// read hash digest and consume hasher
    let result = hasher.finalize();


    let rpc_client = RpcClient::new(URL);
    let input = "account:CandyMachine";
    let base64_bytes_val =encode(&result.as_slice()[0..8]);

    println!("{:?} ---{:?}",base64_bytes_val,MemcmpEncodedBytes::Base64(base64_bytes_val.to_string()))









}


fn main_________() {
    println!("Hello, world!");


// create a Sha256 object
    let mut hasher = Sha256::new();

// write input message
    hasher.update(b"account:CandyMachine");
    let result = hasher.finalize();

    let base64_bytes_val =bs58::encode(&result.as_slice()[0..8]).into_string();

    println!("discriminator {:?}",base64_bytes_val);



// read hash digest and consume hasher
  //  let result = hasher.finalize();

  //  let rpc_url = String::from("http://api.mainnet-beta.solana.com");
   // let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let rpc_client = RpcClient::new(URL);
   // let input = "account:CandyMachine";
    //let base64_bytes_val =&result.as_slice()[0..8];
 //   let base64_bytes_val =&result.as_slice()[0..];

    /*    let input = b"account:CandyMachine";
        let base64_bytes_val = digest_bytes(input).as_slice().try_into()[0..8];
    */

  //  let a = b"hello world";
  //  let b = "aGVsbG8gd29ybGQ=";

    /*    assert_eq!(encode(a), b);
        assert_eq!(a, &decode(b).unwrap()[..]);
str::from_utf8(base64_bytes_val).unwrap().to_string()
    */

/*    let char:  String  = hex::encode(&hasher.finalize()).chars().skip(0).take(8).collect();
    println!("char{:?}",&char);*/

/*    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(
        &hasher.finalize().to_owned()[..8],
       // &anchor_syn::hash::hash("account:CandyMachine".as_bytes()).to_bytes()[..8],
    );
    let SSS = hex::encode(&discriminator.to_vec()[0..8]);
    println!("discriminator {:?}",SSS);
*/




    // mpl_candy_machine::state::CandyMachine;
   // sha256("account:<MyAccountName>")[..8]
    //let encoded = bs58::encode(char).into_string();
   // println!("{:?}",hasher.finalize().as_slice().to_vec());
   // hex::encode(hasher.finalize());
  //  println!("{:?}",hex::encode(hasher.finalize()));
  // let SSS = String::from_utf8(hasher.finalize().as_slice().to_vec()).unwrap();
    let memcmp = RpcFilterType::Memcmp(Memcmp {
        offset: 0,
        bytes: MemcmpEncodedBytes::Base58("9eM5CfcKCCt".to_string()),//MY_WALLET_ADDRESS.to_string()),
        encoding: None,
    });
    let config = RpcProgramAccountsConfig {
    //  filters: None,

        filters: Some(vec![
            //   RpcFilterType::DataSize(128),
            memcmp,
        ]),
  /*      account_config: RpcAccountInfoConfig{
            encoding: Some(UiAccountEncoding::Base64),
            data_slice: None,
            commitment: Some(CommitmentConfig {
                commitment: CommitmentLevel::Confirmed,
            }),
            min_context_slot: None,
        },*/
       account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            data_slice: Some(UiDataSliceConfig {
                offset: 0,
                length: 0,
            }),
            commitment: Some(CommitmentConfig::processed()),
           // min_context_slot: Some(1234),
        },
     //   with_context: Some(false),
        with_context: None
    };


      let candyMachineV2Program = Pubkey::from_str("cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ").unwrap();

    //  let candyMachineV2Program = Pubkey::from_str("A91bwtRn4mZA5D2PoCJs2Gnf3DWdvqmFtSKJQofeaKCD").unwrap();

   let accounts = rpc_client.get_program_accounts_with_config(
        &candyMachineV2Program,
        config,
    ).unwrap();
/*    let accounts = rpc_client.get_program_accounts(
        &candyMachineV2Program,
    ).unwrap();*/
    println!("有多少个：{:?}",accounts.len());



}





fn main_获得这里个() {
    println!("Hello, world!");


// create a Sha256 object
    let mut hasher = Sha256::new();

// write input message
    hasher.update(b"account:CandyMachine");

// read hash digest and consume hasher
    //  let result = hasher.finalize();

    //  let rpc_url = String::from("http://api.mainnet-beta.solana.com");
    // let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let rpc_client = RpcClient::new(URL);
    let input = "account:CandyMachine";
    //let base64_bytes_val =&result.as_slice()[0..8];
    //   let base64_bytes_val =&result.as_slice()[0..];

    /*    let input = b"account:CandyMachine";
        let base64_bytes_val = digest_bytes(input).as_slice().try_into()[0..8];
    */

    let a = b"hello world";
    let b = "aGVsbG8gd29ybGQ=";

    /*    assert_eq!(encode(a), b);
        assert_eq!(a, &decode(b).unwrap()[..]);
str::from_utf8(base64_bytes_val).unwrap().to_string()
    */

    /*    let char:  String  = hex::encode(&hasher.finalize()).chars().skip(0).take(8).collect();
        println!("char{:?}",&char);*/

    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(
        &hasher.finalize().to_owned()[..8],
        // &anchor_syn::hash::hash("account:CandyMachine".as_bytes()).to_bytes()[..8],
    );
        let SSS = hex::encode(&discriminator.to_vec()[0..8]);



  //  let SSS = hex::encode(&hasher.finalize().to_owned()[..8]);



    println!("discriminator {:?}",SSS);
    // mpl_candy_machine::state::CandyMachine;
    // sha256("account:<MyAccountName>")[..8]
    //let encoded = bs58::encode(char).into_string();
    // println!("{:?}",hasher.finalize().as_slice().to_vec());
    // hex::encode(hasher.finalize());
    //  println!("{:?}",hex::encode(hasher.finalize()));
    // let SSS = String::from_utf8(hasher.finalize().as_slice().to_vec()).unwrap();
    let memcmp = RpcFilterType::Memcmp(Memcmp {
        offset: 0,
        bytes: MemcmpEncodedBytes::Base58("9eM5CfcKCCt".to_string()),//MY_WALLET_ADDRESS.to_string()),
        encoding: None,
    });


    let config = RpcProgramAccountsConfig {
        //  filters: None,

        filters: Some(vec![
            //   RpcFilterType::DataSize(128),
            memcmp,
        ]),
        /*      account_config: RpcAccountInfoConfig{
                  encoding: Some(UiAccountEncoding::Base64),
                  data_slice: None,
                  commitment: Some(CommitmentConfig {
                      commitment: CommitmentLevel::Confirmed,
                  }),
                  min_context_slot: None,
              },*/
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            data_slice: Some(UiDataSliceConfig {
                offset: 0,
                length: 0,
            }),
            commitment: Some(CommitmentConfig::processed()),
            // min_context_slot: Some(1234),
        },
        //   with_context: Some(false),
        with_context: None
    };


    let candyMachineV2Program = Pubkey::from_str("cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ").unwrap();

    //  let candyMachineV2Program = Pubkey::from_str("A91bwtRn4mZA5D2PoCJs2Gnf3DWdvqmFtSKJQofeaKCD").unwrap();

    let accounts = rpc_client.get_program_accounts_with_config(
        &candyMachineV2Program,
        config,
    ).unwrap();
    /*    let accounts = rpc_client.get_program_accounts(
            &candyMachineV2Program,
        ).unwrap();*/
    println!("有多少个：{:?}",accounts.len());



}

