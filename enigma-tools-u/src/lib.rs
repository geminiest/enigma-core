#![crate_type = "lib"]

extern crate enigma_crypto;
#[macro_use]
extern crate failure;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate libc;
extern crate openssl;
extern crate rlp;
extern crate bigint;
extern crate rustc_hex as hex;
extern crate serde;
extern crate tiny_keccak;
// web3 utils
extern crate ethabi;
extern crate web3;
// SGX Libraries
extern crate sgx_types;
extern crate sgx_urts;
#[macro_use]
extern crate log;
#[macro_use]
extern crate log_derive;
extern crate ethereum_types;

pub mod attestation_service;
pub mod common_u;
pub mod esgx;
pub mod web3_utils;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
