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
    
    CLTyped, CLValue

};




pub fn balance_key(account: &AccountHash) -> String {
    format!("_balance_{}", account)
}

pub fn logging_key() -> String {
    format!("_logging_{}", 1)
}

pub fn default_operator_key() -> String {
    format!("_default_operator_{}", "s")
}

pub fn revoke_operator_key(sender: &AccountHash, operator: &AccountHash) -> String {
    format!("_revoke_operator_{}_{}", sender, operator)
}

pub fn is_operator_for_key(holder: &AccountHash, token_holder: &AccountHash) -> String {
    format!("_is_operator_for_{}_{}", holder, token_holder)
}

pub fn is_operator_for_main(holder: &AccountHash, token_holder: &AccountHash) -> String {
    format!("_is_operator_for_main_{}_{}", holder, token_holder)
}

pub fn allowance_key(owner: &AccountHash, sender: &AccountHash) -> String {
    format!("_allowance_{}_{}", owner, sender)
}

pub fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

pub fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
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

pub fn remove_key(name: &str) {
     
    runtime::remove_key(name);

}
