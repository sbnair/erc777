use alloc::{
    string::String,
};

use contract::{
    contract_api::{runtime},
    unwrap_or_revert::UnwrapOrRevert,
};

use types::{
    account::AccountHash,
    bytesrepr::{Bytes, ToBytes},
    CLTyped, U256, CLValue
};

use super::mappings::*;


pub fn _exists(token_id: U256) -> bool {

	let zero_addr: AccountHash = AccountHash::from_formatted_str("account.hash.000000000000000000000000000000000000000000").unwrap_or_default();

	let owner: AccountHash = get_key(&owner_key(token_id));

	owner != zero_addr
  
}

pub fn _set_allowance_key(operator: &AccountHash, sender: &AccountHash) {

	 set_key(&allowance_key(&operator, &sender),U256::one());

}


pub fn _call_tokens_to_send(operator: &AccountHash, from: &AccountHash, to: &AccountHash, amount: &U256, data: &Vec<u8>, operator_data: &Vec<u8>) {

	// set_key(&allowance_key(&operator, &sender),U256::one());

}

pub fn _call_tokens_received(operator: &AccountHash, from: &AccountHash, to: &AccountHash, amount: &U256, data: &Vec<u8>, operator_data: &Vec<u8>, bool require_reception_ack) {

	// set_key(&allowance_key(&operator, &sender),U256::one());

}

pub fn _move(operator: &AccountHash, sender: &AccountHash) {

	// set_key(&allowance_key(&operator, &sender),U256::one());

}

pub fn _mint(account: &AccountHash, amount: &U256, data: &Vec<u8>, operator_data: &Vec<u8>, bool require_reception_ack) {

	// set_key(&allowance_key(&operator, &sender),U256::one());
    if ! _exists(account) {

    	 return "ERC777: mint to the zero address";
    }

	set_key(&"total_supply",get_key::<U256>("total_supply").saturating_sub(amount));   
        
    set_key(&balance_key(&token_holder),get_key::<U256>(&balance_key(&token_holder)).saturating_sub(amount));

}

pub fn _send(_operator: &AccountHash, from: &AccountHash, to: &AccountHash, amount: &U256, _data: &Vec<u8>, _operator_data: &Vec<u8>) {
          
           let from_value: AccountHash = *from;

           let amount_value: U256 = *amount;
            
            // set_key(&balance_key(&from_value),get_key::<U256>(&balance_key(&from_value)).saturating_sub(amount_value)); 
         
            // set_key(&balance_key(to), get_key::<U256>(&balance_key(&to)).saturating_sub(amount_value));
         

          
            if erc20_compatible() {

            }
}

pub fn _burn(operator: &AccountHash, token_holder: &AccountHash, amount: &U256, _data: &Vec<u8>, _operator_data: &Vec<u8>) {
        
                

}

pub fn _approve(holder: &AccountHash, spender: &AccountHash, value: &U256) {
        
            

}




fn allowance_key(owner: &AccountHash, sender: &AccountHash) -> String {
    format!("allowances_{}_{}", owner, sender)
}

fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
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