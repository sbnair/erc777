
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


pub fn _exists(_token_id: U256) -> bool {

	let zero_addr: AccountHash = AccountHash::from_formatted_str("account-hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap_or_default();

	let owner: AccountHash = get_key(&owner_key(_token_id));

	owner != zero_addr
  
}



pub fn _exists_owner(_owner_id: AccountHash) -> bool {

     let zero_addr: AccountHash = AccountHash::from_formatted_str("account-hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap_or_default();

     let owner: AccountHash = _owner_id;

     owner != zero_addr
   


}

pub fn _is_operator_for(_operator: AccountHash, _token_holder: AccountHash) -> bool {
     if _operator == _token_holder {
                ret(true);
     }
     get_key::<U256>(&allowance_key(&_operator, &_token_holder)) == U256::one()
     
}

pub fn _authorize_operator(_operator: AccountHash, _holder: AccountHash) {
   
    if (_operator != _holder) {
        ret("ERC777: authorizing self as operator"); 
    }
}

pub fn _allowance(_holder: AccountHash, _spender: AccountHash) {
   
    let val: U256 = get_key::<U256>(&allowance_key(&_holder, &_spender));
    ret(val);   
}

pub fn _revoke_operator(_operator: AccountHash, _holder: AccountHash) {
     
    if (_operator != _holder) {
        ret("ERC777: revoking self as operator");  
    }
}

pub fn _set_allowance_key(_operator: AccountHash, _sender: AccountHash, _value: U256) {

	 set_key(&allowance_key(&_operator, &_sender),_value);

}

#[allow(unused)]
pub fn _call_tokens_to_send(_operator: AccountHash, _from: AccountHash, _to: AccountHash, _amount: U256, _data: Bytes, _operator_data: Bytes) {

	// set_key(&allowance_key(&operator, &sender),U256::one());

}

#[allow(unused)]
pub fn _call_tokens_received(_operator: AccountHash, _from: AccountHash, _to: AccountHash, _amount: U256, _data: Bytes, _operator_data: Bytes, _require_reception_ack: bool) {

	// set_key(&allowance_key(&_operator, &_sender),U256::one());

}

#[allow(unused)]
pub fn _before_token_transfer(_operator: AccountHash, _from: AccountHash, _to: AccountHash, _amount: U256) {

    // set_key(&allowance_key(&operator, &sender),U256::one());

}

pub fn _move(_operator: AccountHash, _from: AccountHash, _to: AccountHash, _amount: U256, _user_data:Bytes, _operator_data: Bytes) {

	// set_key(&allowance_key(&operator, &sender),U256::one());
    _before_token_transfer(_operator, _from, _to, _amount);

     let from_balance: U256 = get_key::<U256>(&balance_key(&_from));

     if from_balance >= _amount {

         ret("ERC777: transfer amount exceeds balance");
     }

     set_key(&balance_key(&_from),get_key::<U256>(&balance_key(&_from)).saturating_sub(_amount));

     set_key(&balance_key(&_to),get_key::<U256>(&balance_key(&_to)).saturating_add(_amount));


}


pub fn _mint(_account: AccountHash, _amount: U256, _data:Bytes, _operator_data:Bytes) {

	
     _mintcheck(_account, _amount, _data, _operator_data, true);
}

pub fn _mintcheck(_account: AccountHash, _amount: U256, _data: Bytes, _operator_data: Bytes, _require_reception_ack: bool) {

	// set_key(&allowance_key(&operator, &sender),U256::one());
    if  _exists_owner(_account) {

    	 ret("ERC777: mint to the zero address");
    }

	set_key(&"total_supply",get_key::<U256>("total_supply").saturating_sub(_amount));   
        
    set_key(&balance_key(&_account),get_key::<U256>(&balance_key(&_account)).saturating_sub(_amount));

}

pub fn _send(_from: AccountHash, _to: AccountHash, _amount: U256, _data: Bytes, _operator_data: Bytes, _require_reception_ack: bool) {
          
           
            if  _exists_owner(_from) {

    	      ret("ERC777: send from the zero address");
            }

            if  _exists_owner(_to) {

    	      ret("ERC777: send to the zero address");
            }


            
            // set_key(&balance_key(&from_value),get_key::<U256>(&balance_key(&from_value)).saturating_sub(amount_value)); 
         
            // set_key(&balance_key(to), get_key::<U256>(&balance_key(&to)).saturating_sub(amount_value));
         

}

pub fn _burn(_from: AccountHash, _amount: U256, _data: Bytes, _operator_data: Bytes) {
        
        if  _exists_owner(_from) {

    	      ret("ERC777: burn from the zero address");
        }          

}

pub fn _approve(_holder: AccountHash, _spender: AccountHash, _value: U256) {
        
        if  _exists_owner(_holder) {

    	      ret("ERC777: approve from the zero address");
        }


        if  _exists_owner(_spender) {

    	      ret("ERC777: approve from the zero address");
        }

        _set_allowance_key(_holder, _spender, _value);      

}
