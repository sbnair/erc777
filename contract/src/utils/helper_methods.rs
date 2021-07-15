
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

pub fn _is_operator_for(operator: AccountHash, token_holder: AccountHash) -> bool {
     if operator == token_holder {
                return true;
     }
     get_key::<U256>(&allowance_key(&operator, &token_holder)) == U256::one()
     
}

pub fn _authorize_operator(operator: AccountHash, holder: AccountHash) {
   
    if (operator != holder)
        return "ERC777: authorizing self as operator"; 
}

pub fn _allowance(holder: AccountHash, spender: AccountHash) {
   
    let val: U256 = get_key::<U256>(&allowance_key(&operator, &token_holder));
    ret(val)   
}

pub fn _revoke_operator(operator: AccountHash, holder: AccountHash) {
     
    if (operator != holder)
        return "ERC777: revoking self as operator";  
}

pub fn _set_allowance_key(operator: &AccountHash, sender: &AccountHash, value: &U256) {

	 set_key(&allowance_key(&operator, &sender),value);

}


pub fn _call_tokens_to_send(operator: &AccountHash, from: &AccountHash, to: &AccountHash, amount: &U256, data: &Vec<u8>, operator_data: &Vec<u8>) {

	// set_key(&allowance_key(&operator, &sender),U256::one());

}

pub fn _call_tokens_received(operator: &AccountHash, from: &AccountHash, to: &AccountHash, amount: &U256, data: &Vec<u8>, operator_data: &Vec<u8>, bool require_reception_ack) {

	// set_key(&allowance_key(&operator, &sender),U256::one());

}

pub fn _before_token_transfer(operator: &AccountHash, from: &AccountHash, to: &AccountHash, amount: &U256) {

    // set_key(&allowance_key(&operator, &sender),U256::one());

}

pub fn _move(operator: &AccountHash, from: &AccountHash, to: &AccountHash, amount: &U256, user_data: &Vec<u8>, operator_data: &Vec<u8> ) {

	// set_key(&allowance_key(&operator, &sender),U256::one());
    _before_token_transfer(operator, from, to, amount);

     let from_balance: U256 = get_key::<U256>(&balance_key(&from));

     if from_balance >= amount {

         return "ERC777: transfer amount exceeds balance";
     }

     set_key(&balance_key(&from),get_key::<U256>(&balance_key(&from)).saturating_sub(amount));

     set_key(&balance_key(&to),get_key::<U256>(&balance_key(&to)).saturating_add(amount));


}


pub fn _mint(account: &AccountHash, amount: &U256, data: &Vec<u8>, operator_data: &Vec<u8>) {

	
     _mint(account, amount, data, operator_data, true);
}

pub fn _mint(account: &AccountHash, amount: &U256, data: &Vec<u8>, operator_data: &Vec<u8>, bool require_reception_ack) {

	// set_key(&allowance_key(&operator, &sender),U256::one());
    if  _exists(account) {

    	 return "ERC777: mint to the zero address";
    }

	set_key(&"total_supply",get_key::<U256>("total_supply").saturating_sub(amount));   
        
    set_key(&balance_key(&token_holder),get_key::<U256>(&balance_key(&token_holder)).saturating_sub(amount));

}

pub fn _send(from: &AccountHash, to: &AccountHash, amount: &U256, _data: &Vec<u8>, _operator_data: &Vec<u8>, bool require_reception_ack) {
          
           
            if  _exists(from) {

    	      return "ERC777: send from the zero address";
            }

            if  _exists(to) {

    	      return "ERC777: send to the zero address";
            }


            
            // set_key(&balance_key(&from_value),get_key::<U256>(&balance_key(&from_value)).saturating_sub(amount_value)); 
         
            // set_key(&balance_key(to), get_key::<U256>(&balance_key(&to)).saturating_sub(amount_value));
         

}

pub fn _burn(from: &AccountHash, amount: &U256, _data: &Vec<u8>, _operator_data: &Vec<u8>) {
        
        if  _exists(from) {

    	      return "ERC777: burn from the zero address";
        }          

}

pub fn _approve(holder: &AccountHash, spender: &AccountHash, value: &U256) {
        
        if  _exists(holder) {

    	      return "ERC777: approve from the zero address";
        }


        if  _exists(spender) {

    	      return "ERC777: approve from the zero address";
        }

        _set_allowance_key(holder, spender, value);      

}
