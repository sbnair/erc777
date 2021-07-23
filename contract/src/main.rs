/*
#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
*/
#![no_main]
extern crate alloc;

use alloc::{

  string::String,

};

use std::os::raw::c_char;

use contract::{

    contract_api::{runtime, storage},
};

use types::{

    account::AccountHash,

    bytesrepr::{Bytes},

    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},

    CLType, CLTyped, Parameter, U256

};

 mod utils;

use utils::helper_methods::*;

use utils::mappings::*;


// const KEY: &str = "special_value";
// const ARG_MESSAGE: &str = "message";

// fn store(value: String) {
    // Store `value` under a new unforgeable reference.
   // let value_ref: URef = storage::new_uref(value);

    // Wrap the unforgeable reference in a value of type `Key`.
   // let value_key: Key = value_ref.into();

    // Store this key under the name "special_value" in context-local storage.
   // runtime::put_key(KEY, value_key);
// }

#[no_mangle]
// Gets the name of token
pub extern "C" fn name() {
    let val: String = get_key("name");
    ret(val)
}

#[no_mangle]
// Gets the symbol of token
pub extern "C" fn symbol() {
    let val: String = get_key("symbol");
    ret(val)
}

#[no_mangle]
// Gets the decimals of token
pub extern "C" fn decimals() {
    let val: U256 = U256::from(18);
    ret(val)
}

#[no_mangle]
// Gets the total supply of token
pub extern "C" fn total_supply() {
    let val: U256 = get_key("total_supply");
    ret(val)
}

#[no_mangle]
// Gets the balance of account
pub extern "C" fn balance_of() {
    let account: AccountHash = runtime::get_named_arg("account");
    let val: U256 = get_key::<U256>(&balance_key(&account));
    ret(val)
}

#[no_mangle]
// Get the smallest part of the token that’s not divisible.
pub extern "C" fn granularity() {
    let val: U256 = U256::from(1);
    ret(val)
}

#[no_mangle]
// Get the list of default operators as defined by the token contract.
pub extern "C" fn default_operators() {
    let val: Vec<AccountHash> = Vec::new();
    ret(val);
}

#[no_mangle]
// Set a third party operator address as an operator of msg.sender to send and burn tokens on its behalf.
pub extern "C" fn authorize_operator() {
    let operator: AccountHash = runtime::get_named_arg("operator");
    
    _authorize_operator(operator, runtime::get_caller());    
}

#[no_mangle]
// Remove the right of the operator address to be an operator for msg.sender and to send and burn tokens on its behalf.
pub extern "C" fn revoke_operator() {
    let operator: AccountHash = runtime::get_named_arg("operator");
  
    _revoke_operator(operator, runtime::get_caller());
    
}


#[no_mangle]
// * Unlike `send`, `recipient` is _not_ required to implement the {IERC777Recipient}
//     * interface if it is a contract.
pub extern "C" fn transfer() -> *const c_char {
    let recipient: AccountHash = runtime::get_named_arg("recipient");

    let from: AccountHash = runtime::get_caller();

    let amount: U256 = runtime::get_named_arg("amount");

    if  _exists_owner(recipient) {

              return "ERC777: transfer to the zero address".as_ptr() as *const c_char;
    }

    _call_tokens_to_send(from, from, recipient, amount, Bytes::new(), Bytes::new());

    _move(from, from, recipient, amount, Bytes::new(), Bytes::new());

    _call_tokens_received(from, from, recipient, amount, Bytes::new(), Bytes::new(), false);    
  
    return "true".as_ptr() as *const c_char;
    
}


#[no_mangle]
// Note that operator and allowance concepts are orthogonal: operators cannot
//     * call `transferFrom` (unless they have allowance), and accounts with
//     * allowance cannot call `operatorSend` (unless they are operators).
pub extern "C" fn transfer_from() -> *const c_char {

    let holder: AccountHash = runtime::get_named_arg("holder");

    let recipient: AccountHash = runtime::get_named_arg("recipient");

    let spender: AccountHash = runtime::get_caller();

    let amount: U256 = runtime::get_named_arg("amount");

    if  _exists_owner(recipient) {

              return "ERC777: transfer to the zero address".as_ptr() as *const c_char;
    }

    if  _exists_owner(holder) {

              return "ERC777: transfer from the zero address".as_ptr() as *const c_char;
    }

    _call_tokens_to_send(spender, holder, recipient, amount, Bytes::new(), Bytes::new());

    _move(spender, holder, recipient, amount, Bytes::new(), Bytes::new());

    let current_allowance: U256 = get_key::<U256>(&allowance_key(&holder, &spender));

    if current_allowance >= amount {

        return "ERC777: transfer amount exceeds allowance".as_ptr() as *const c_char;
    }

    _approve(spender, holder, current_allowance.saturating_sub(amount));

     _call_tokens_received(spender, holder, recipient, amount, Bytes::new(), Bytes::new(), false); 

  
    return "true".as_ptr() as *const c_char;
    
}

#[no_mangle]
// Indicate whether the operator address is an operator of the holder address.
pub extern "C" fn is_operator_for() {

    let operator: AccountHash = runtime::get_named_arg("operator");
    
    let token_holder: AccountHash = runtime::get_named_arg("token_holder"); 
    
    ret(_is_operator_for(operator, token_holder));
  
}

#[no_mangle]
//  * Note that accounts cannot have allowance issued by their operators.
pub extern "C" fn approve() {
   
    let holder: AccountHash = runtime::get_caller();
    
    let spender: AccountHash = runtime::get_named_arg("spender");
    
    let value: U256 = runtime::get_named_arg("value");
    
    _approve(holder, spender, value);
   
}


#[no_mangle]
//  * @dev Send tokens
//     * @param from address token holder address
//     * @param to address recipient address
//     * @param amount uint256 amount of tokens to transfer
//     * @param userData bytes extra information provided by the token holder (if any)
//     * @param operatorData bytes extra information provided by the operator (if any)
//     * @param requireReceptionAck if true, contract recipients are required to implement ERC777TokensRecipient
pub extern "C" fn send() {
   
    let to: AccountHash = runtime::get_named_arg("to");
   
    let amount: U256 = runtime::get_named_arg("amount");
   
    let _data:Bytes = runtime::get_named_arg("data");
   
    _send(runtime::get_caller(), to, amount, _data, Bytes::new(), true);
   
}


#[no_mangle]
//  * @dev Burn tokens
//     * @param from address token holder address
//     * @param amount uint256 amount of tokens to burn
//     * @param data bytes extra information provided by the token holder
//     * @param operatorData bytes extra information provided by the operator (if any)
pub extern "C" fn burn() {
    
    let amount: U256 = runtime::get_named_arg("amount");
    
    let _data:Bytes = runtime::get_named_arg("data");
    
    _burn(runtime::get_caller(), amount, _data, Bytes::new());
}



#[no_mangle]
// Send the amount of tokens on behalf of the address from to the address to.
pub extern "C" fn operator_send() -> *const c_char {
    
    let to: AccountHash = runtime::get_named_arg("recipient");
    
    let amount: U256 = runtime::get_named_arg("amount");
    
    let _data:Bytes = runtime::get_named_arg("data");
    
    let _operator_data:Bytes = runtime::get_named_arg("operator_data");

     if _is_operator_for(runtime::get_caller(), to) {
       
         return "ERC777: caller is not an operator for holder".as_ptr() as *const c_char;
     
     }   
    
    _send(runtime::get_caller(), to, amount, _data, _operator_data, true);

    return "true".as_ptr() as *const c_char;
}

#[no_mangle]
// The operator and the holder MUST both be the msg.sender.
pub extern "C" fn operator_burn() -> *const c_char {
    
    let account: AccountHash = runtime::get_named_arg("account");
   
    let amount: U256 = runtime::get_named_arg("amount");
    
    let _data:Bytes = runtime::get_named_arg("data");
    
    let _operator_data:Bytes = runtime::get_named_arg("operator_data");

    if _is_operator_for(runtime::get_caller(), account) {
       return "ERC777: caller is not an operator for holder".as_ptr() as *const c_char; 
    }
    
    _burn(runtime::get_caller(), amount, _data, _operator_data);

    return "true".as_ptr() as *const c_char;
}


#[no_mangle]
// * Note that operator and allowance concepts are orthogonal: operators may
//    * not have allowance, and accounts with allowance may not be operators
//     * themselves.
pub extern "C" fn allowance() {
    let holder: AccountHash = runtime::get_caller();
   
    let spender: AccountHash = runtime::get_named_arg("spender");
    
    _allowance(holder, spender);
}

#[no_mangle]
// * @dev Creates `amount` tokens and assigns them to `account`, increasing
//     * the total supply.
//     *
//     * If a send hook is registered for `account`, the corresponding function
//     * will be called with `operator`, `data` and `operatorData`.
pub extern "C" fn mint() {
            
    let token_holder: AccountHash = runtime::get_named_arg("token_holder");

    let amount: U256 = runtime::get_named_arg("amount");

    let _data:Bytes = runtime::get_named_arg("data");
    
    let _operator_data:Bytes = runtime::get_named_arg("operator_data");
   
          
    _mint(token_holder, amount, _data, _operator_data);
       
    
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
   
    let token_name: String = runtime::get_named_arg("token_name");
   
    let token_symbol: String = runtime::get_named_arg("token_symbol");
   
    let token_total_supply: U256 = runtime::get_named_arg("token_total_supply");
   
  //  let token_granularity: U256 = runtime::get_named_arg("token_granularity");
   
  //  let token_default_operators: Vec<AccountHash> = runtime::get_named_arg("token_default_operators"); 
    // Get the optional first argument supplied to the argument.
 //   let value: String = runtime::get_named_arg(ARG_MESSAGE);
  //  store(value);
    let mut entry_points = EntryPoints::new(); 
    entry_points.add_entry_point(endpoint("name", vec![], CLType::Unit));

    entry_points.add_entry_point(endpoint("symbol", vec![], CLType::Unit));
    
    entry_points.add_entry_point(endpoint("total_supply", vec![], CLType::U256));
    
    entry_points.add_entry_point(endpoint("granularity", vec![], CLType::U256));
    
    entry_points.add_entry_point(endpoint("default_operators", vec![], AccountHash::cl_type()));
    

    entry_points.add_entry_point(endpoint(
        "balance_of",
       vec![Parameter::new("account", AccountHash::cl_type())],
        CLType::Unit,
    ));
    
     entry_points.add_entry_point(endpoint(
        "authorize_operator",
        vec![Parameter::new("operator", AccountHash::cl_type())],
        CLType::Unit,
    ));

    entry_points.add_entry_point(endpoint(
        "revoke_operator",
        vec![Parameter::new("operator", AccountHash::cl_type())],
        CLType::Unit,
    ));

    entry_points.add_entry_point(endpoint(
        "transfer",
        vec![Parameter::new("recipient", AccountHash::cl_type()),

             Parameter::new("from", AccountHash::cl_type()),
             
             Parameter::new("data", CLType::U256),          
        ],
        CLType::Unit,
    ));

    entry_points.add_entry_point(endpoint(
        "transfer_from",
        vec![Parameter::new("holder", AccountHash::cl_type()),

             Parameter::new("recipient", AccountHash::cl_type()),
             
             Parameter::new("sender", AccountHash::cl_type()),  
             
             Parameter::new("amount", CLType::U256),

        ],
        CLType::Unit,
    ));

    entry_points.add_entry_point(endpoint(
        "is_operator_for",
        vec![Parameter::new("holder", AccountHash::cl_type()),

             Parameter::new("recipient", AccountHash::cl_type()),
            
        ],
        CLType::Unit,
    ));


    entry_points.add_entry_point(endpoint(
        "approve",
        vec![Parameter::new("holder", AccountHash::cl_type()),

             Parameter::new("spender", AccountHash::cl_type()),
             
             Parameter::new("value", CLType::U256),  
        ],
        CLType::Unit,
    ));

    entry_points.add_entry_point(endpoint(
        "send",
        vec![Parameter::new("to", AccountHash::cl_type()),

             Parameter::new("amount", CLType::U256),
             
             Parameter::new("_data", Bytes::cl_type()),  
        ],
        CLType::Unit,
    ));

    entry_points.add_entry_point(endpoint(
        "burn",
        vec![Parameter::new("amount", CLType::U256),
             
             Parameter::new("_data", Bytes::cl_type()),  
        ],
        CLType::Unit,
    ));

    entry_points.add_entry_point(endpoint(
        "operator_send",
        vec![Parameter::new("sender", AccountHash::cl_type()),

             Parameter::new("recipient", AccountHash::cl_type()),
             
             Parameter::new("amount", CLType::U256), 

             Parameter::new("_data", Bytes::cl_type()),  

             Parameter::new("_operator_data", Bytes::cl_type()),  
             
        ],
        CLType::Unit,
    ));

    entry_points.add_entry_point(endpoint(
        "operator_burn",
        vec![Parameter::new("account", AccountHash::cl_type()),

             Parameter::new("amount", CLType::U256), 

             Parameter::new("_data", Bytes::cl_type()),  

             Parameter::new("_operator_data", Bytes::cl_type()),  
             
        ],
        CLType::Unit,
    ));

    entry_points.add_entry_point(endpoint(
        "allowance",
        vec![Parameter::new("holder", AccountHash::cl_type()),

             Parameter::new("spender", CLType::U256), 
        ],
        CLType::Unit,
    ));

    entry_points.add_entry_point(endpoint(
        "mint",
        vec![Parameter::new("token_holder", AccountHash::cl_type()),

             Parameter::new("amount", CLType::U256), 

             Parameter::new("_data", Bytes::cl_type()),  

             Parameter::new("_operator_data", Bytes::cl_type()),  
        ],
        CLType::Unit,
    ));

    let mut named_keys = NamedKeys::new();
    
    named_keys.insert("name".to_string(), storage::new_uref(token_name).into());
    
    named_keys.insert("symbol".to_string(), storage::new_uref(token_symbol).into());
    
    named_keys.insert(
        "total_supply".to_string(),
       storage::new_uref(token_total_supply).into(),
    );
    
   // named_keys.insert(
     //   "granularity".to_string(),
      //  storage::new_uref(token_granularity).into(),
   // );
    
   // named_keys.insert(
     //   "default_operators".to_string(),
     //   storage::new_uref(token_default_operators).into(),
   // );
    
    named_keys.insert(
       balance_key(&runtime::get_caller()),
       storage::new_uref(token_total_supply).into(),
    );
    
    let (contract_hash, _) =
        storage::new_locked_contract(entry_points, Some(named_keys), None, None);
    
    runtime::put_key("ERC777", contract_hash.into());
    
    runtime::put_key("ERC777_hash", storage::new_uref(contract_hash).into());
  
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

