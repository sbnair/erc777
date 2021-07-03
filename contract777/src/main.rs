#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

//  use casper_contract::{
//    contract_api::{runtime, storage},
// };
// use casper_types::{Key, URef};

extern crate alloc;

use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use core::convert::TryInto;

 use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
 };
use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    runtime_args, CLType, CLTyped, CLValue, Group, Parameter, RuntimeArgs, URef, U256, Key
};


const KEY: &str = "special_value";
const ARG_MESSAGE: &str = "message";

fn store(value: String) {
    // Store `value` under a new unforgeable reference.
    let value_ref: URef = storage::new_uref(value);

    // Wrap the unforgeable reference in a value of type `Key`.
    let value_key: Key = value_ref.into();

    // Store this key under the name "special_value" in context-local storage.
    runtime::put_key(KEY, value_key);
}

#[no_mangle]
pub extern "C" fn name() {
    let val: String = get_key("name");
    ret(val)
}

#[no_mangle]
pub extern "C" fn symbol() {
    let val: String = get_key("symbol");
    ret(val)
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let val: U256 = get_key("total_supply");
    ret(val)
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let account: AccountHash = runtime::get_named_arg("account");
    let val: U256 = get_key(&balance_key(&account));
    ret(val)
}

#[no_mangle]
pub extern "C" fn granularity() {
    let val: U256 = get_key("granularity");
    ret(val)
}

#[no_mangle]
pub extern "C" fn default_operators() {
    let val: Vec<AccountHash> = Vec::new();
    ret(val)
}

#[no_mangle]
pub extern "C" fn authorize_operator() {
    let operator: AccountHash = runtime::get_named_arg("operator");
    

    
    set_key(&allowance_key(&operator, &runtime::get_caller()),U256::one());

    
    _authorize_operator(operator, runtime::get_caller());    
}

#[no_mangle]
pub extern "C" fn revoke_operator() {
    let operator: AccountHash = runtime::get_named_arg("operator");
   // require(operator != runtime::get_caller(), "Cannot revoke yourself as an operator"); 
   set_key(&allowance_key(&operator, &runtime::get_caller()),U256::zero());
   _revoke_operator(operator, runtime::get_caller());
}

#[no_mangle]
pub extern "C" fn is_operator_for() {
    let operator: AccountHash = runtime::get_named_arg("operator");
    let token_holder: AccountHash = runtime::get_named_arg("token_holder"); 
    _is_operator_for(operator, token_holder);
   // require(operator != runtime::get_caller(), "Cannot revoke yourself as an operator"); 
  // _revoke_operator(operator, runtime::get_caller());
}

#[no_mangle]
pub extern "C" fn send() {
    let to: AccountHash = runtime::get_named_arg("to");
    let amount: U256 = runtime::get_named_arg("amount");
    let data: Vec<u8> = runtime::get_named_arg("data");
    // _is_operator_for(operator, token_holder);
    do_send(&runtime::get_caller(), &runtime::get_caller(), &to, &amount, &data, &Vec::new());
   
}


#[no_mangle]
pub extern "C" fn burn() {
    let amount: U256 = runtime::get_named_arg("amount");
    let data: Vec<u8> = runtime::get_named_arg("data");
    do_burn(&runtime::get_caller(), &runtime::get_caller(), &amount, &data, &Vec::new());
}

#[no_mangle]
pub extern "C" fn disableERC20() {
     set_key(&erc20_compatibility_key(), U256::zero());                
}

#[no_mangle]      
pub extern "C" fn enableERC20() {
     set_key(&erc20_compatibility_key(), U256::one());         
              
}

#[no_mangle]
pub extern "C" fn operator_send() {
    let from: AccountHash = runtime::get_named_arg("from");
    let to: AccountHash = runtime::get_named_arg("to");
    let amount: U256 = runtime::get_named_arg("amount");
    let data: Vec<u8> = runtime::get_named_arg("data");
    let operator_data: Vec<u8> = runtime::get_named_arg("operator_data");
    // _is_operator_for(operator, token_holder);
    do_send(&runtime::get_caller(), &from, &to, &amount, &data, &operator_data);
}

#[no_mangle]
pub extern "C" fn operator_burn() {
    let from: AccountHash = runtime::get_named_arg("from");
   
    let amount: U256 = runtime::get_named_arg("amount");
    let data: Vec<u8> = runtime::get_named_arg("data");
    let operator_data: Vec<u8> = runtime::get_named_arg("operator_data");
    
    do_burn(&runtime::get_caller(), &from, &amount, &data, &operator_data);
}

#[no_mangle]
pub extern "C" fn mint() {
            
    let token_holder: AccountHash = runtime::get_named_arg("token_holder");

    let amount: U256 = runtime::get_named_arg("amount");
   
 
            
    set_key(&"total_supply",get_key::<U256>("total_supply").saturating_sub(amount));   
        
    set_key(&balance_key(&token_holder),get_key::<U256>(&balance_key(&token_holder)).saturating_sub(amount));
          
 
       
    if erc20_compatible() {
              //  self.Transfer(AccountHash::zero(), token_holder, amount);
     }
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
   
    let token_name: String = runtime::get_named_arg("token_name");
    let token_symbol: String = runtime::get_named_arg("token_symbol");
    let token_total_supply: U256 = runtime::get_named_arg("token_total_supply");
    let token_granularity: U256 = runtime::get_named_arg("token_granularity");
    let token_default_operators: Vec<AccountHash> = runtime::get_named_arg("token_default_operators"); 
    // Get the optional first argument supplied to the argument.
    let value: String = runtime::get_named_arg(ARG_MESSAGE);
    store(value);
    let mut entry_points = EntryPoints::new(); 
    entry_points.add_entry_point(endpoint("name", vec![], CLType::String));
    entry_points.add_entry_point(endpoint("symbol", vec![], CLType::String));
    entry_points.add_entry_point(endpoint("total_supply", vec![], CLType::U256));
    entry_points.add_entry_point(endpoint("granularity", vec![], CLType::U256));
    entry_points.add_entry_point(endpoint("default_operators", vec![], AccountHash::cl_type()));
    entry_points.add_entry_point(endpoint(
        "balance_of",
        vec![Parameter::new("account", AccountHash::cl_type())],
        CLType::U256,
    ));
     entry_points.add_entry_point(endpoint(
        "authorize_operator",
        vec![Parameter::new("operator", AccountHash::cl_type())],
        CLType::U256,
    ));
    entry_points.add_entry_point(endpoint(
        "revoke_operator",
        vec![Parameter::new("operator", AccountHash::cl_type())],
        CLType::U256,
    ));

    let mut named_keys = NamedKeys::new();
    named_keys.insert("name".to_string(), storage::new_uref(token_name).into());
    named_keys.insert("symbol".to_string(), storage::new_uref(token_symbol).into());
    named_keys.insert(
        "total_supply".to_string(),
        storage::new_uref(token_total_supply).into(),
    );
    named_keys.insert(
        "granularity".to_string(),
        storage::new_uref(token_granularity).into(),
    );
    named_keys.insert(
        "default_operators".to_string(),
        storage::new_uref(token_default_operators).into(),
    );
    named_keys.insert(
        balance_key(&runtime::get_caller()),
        storage::new_uref(token_total_supply).into(),
    );
    let (contract_hash, _) =
        storage::new_locked_contract(entry_points, Some(named_keys), None, None);
    runtime::put_key("ERC777", contract_hash.into());
  
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

fn balance_key(account: &AccountHash) -> String {
    format!("balances_{}", account)
}

fn erc20_compatibility_key() -> String {
   format!("erc20_compatibility_{}","erc20")
}

fn _authorize_operator(_operator: AccountHash, _holder: AccountHash) {
  
}

fn _revoke_operator(_operator: AccountHash, _holder: AccountHash) {
  
}


fn do_send(_operator: &AccountHash, from: &AccountHash, to: &AccountHash, amount: &U256, _data: &Vec<u8>, _operator_data: &Vec<u8>) {
          
           let from_value: AccountHash = *from;

           let amount_value: U256 = *amount;
            
            set_key(&balance_key(&from_value),get_key::<U256>(&balance_key(&from_value)).saturating_sub(amount_value)); 
         
            set_key(&balance_key(to), get_key::<U256>(&balance_key(&to)).saturating_sub(amount_value));
         

          
            if erc20_compatible() {
              //  self.Transfer(*from, *to, *amount);
            }
        }

        fn do_burn(_operator: &AccountHash, token_holder: &AccountHash, amount: &U256, _data: &Vec<u8>, _operator_data: &Vec<u8>) {
        
            set_key(&balance_key(token_holder),get_key::<U256>(&balance_key(&token_holder)).saturating_sub(*amount)); 

         
            set_key(&"total_supply",get_key::<U256>("total_supply").saturating_sub(*amount));
        

          //  self.Burned(*operator,
            //            *token_holder,
              //          *amount,
                //        data.clone(),
                  //      operator_data.clone());

            if erc20_compatible() {
              //  self.Transfer(*token_holder, AccountHash::zero(), *amount);
            }
        }

fn _is_operator_for(operator: AccountHash, token_holder: AccountHash) -> bool {
     if operator == token_holder {
                return true;
     }
     get_key::<U256>(&allowance_key(&operator, &token_holder)) == U256::one()
     
}


fn allowance_key(owner: &AccountHash, sender: &AccountHash) -> String {
    format!("allowances_{}_{}", owner, sender)
}

fn endpoint(name: &str, param: Vec<Parameter>, ret: CLType) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        param,
        ret,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}


/// Reads the current state of the ERC20 compatibility setting
pub fn erc20_compatible() -> bool {
    get_key::<U256>(&erc20_compatibility_key()) == U256::one()
}