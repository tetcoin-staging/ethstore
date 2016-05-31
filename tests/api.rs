extern crate rand;
extern crate ethstore;
extern crate ethkey;

mod util;

use ethstore::{SecretStore, EthStore};
use ethkey::Random;
use util::TransientDir;

#[test]
fn secret_store_create() {
	let dir = TransientDir::create().unwrap();
	let _ = EthStore::open(dir).unwrap();
}

#[test]
#[should_panic]
fn secret_store_open_not_existing() {
	let dir = TransientDir::open();
	let _ = EthStore::open(dir).unwrap();
}

#[test]
fn secret_store_create_account() {
	let dir = TransientDir::create().unwrap();
	let store = EthStore::open(dir).unwrap();
	assert_eq!(store.accounts().len(), 0);
	assert!(store.create_account(Random, "").is_ok());
	assert_eq!(store.accounts().len(), 1);
	assert!(store.create_account(Random, "").is_ok());
	assert_eq!(store.accounts().len(), 2);
}

#[test]
fn secret_store_sign() {
	let dir = TransientDir::create().unwrap();
	let store = EthStore::open(dir).unwrap();
	assert!(store.create_account(Random, "").is_ok());
	let accounts = store.accounts();
	assert_eq!(accounts.len(), 1);
	assert!(store.sign(&accounts[0], "", &Default::default()).is_ok());
	assert!(store.sign(&accounts[0], "1", &Default::default()).is_err());
}

#[test]
fn secret_store_change_password() {
	let dir = TransientDir::create().unwrap();
	let store = EthStore::open(dir).unwrap();
	assert!(store.create_account(Random, "").is_ok());
	let accounts = store.accounts();
	assert_eq!(accounts.len(), 1);
	assert!(store.sign(&accounts[0], "", &Default::default()).is_ok());
	assert!(store.change_password(&accounts[0], "", "1").is_ok());
	assert!(store.sign(&accounts[0], "", &Default::default()).is_err());
	assert!(store.sign(&accounts[0], "1", &Default::default()).is_ok());
}

#[test]
fn secret_store_remove_account() {
	let dir = TransientDir::create().unwrap();
	let store = EthStore::open(dir).unwrap();
	assert!(store.create_account(Random, "").is_ok());
	let accounts = store.accounts();
	assert_eq!(accounts.len(), 1);
	assert!(store.remove_account(&accounts[0], "").is_ok());
	assert_eq!(store.accounts().len(), 0);
	assert!(store.remove_account(&accounts[0], "").is_err());
}
