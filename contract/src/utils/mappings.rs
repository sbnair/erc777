use core::convert::TryInto;

use alloc::{

    string::String,

};

use contract::{

    contract_api::{runtime, storage},

    unwrap_or_revert::UnwrapOrRevert,

};

use types::{
    
    account::AccountHash,
    
    bytesrepr::{ToBytes, FromBytes},
    
    CLTyped, U256, CLValue

};




pub fn balance_key(account: &AccountHash) -> String {
    format!("_balance_{}", account)
}


pub fn allowance_key(owner: &AccountHash, sender: &AccountHash) -> String {
    format!("allowances_{}_{}", owner, sender)
}

pub fn owner_key(token_id: U256) -> String {
    format!("_owner_{}",token_id);
}

pub fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

pub fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert();
        }
    }
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}
