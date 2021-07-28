
use types::{

    account::AccountHash,

    bytesrepr::{Bytes},

    U256

};

use std::os::raw::c_char;

use super::mappings::*;


/// Checks whether Account exists or not.
pub fn _exists_owner(_owner_id: AccountHash) -> bool {

     let zero_addr: AccountHash = AccountHash::from_formatted_str("account-hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap_or_default();

     let owner: AccountHash = _owner_id;

     owner == zero_addr
}

/// Checks the operator.
pub fn _is_operator_for(_operator: AccountHash, _token_holder: AccountHash) -> bool {
      
    let default_op: bool = get_key::<bool>(&default_operator_key());

    let revoke_op: bool = get_key::<bool>(&revoke_operator_key(&_operator, &_token_holder));

    let is_op_val: bool = get_key::<bool>(&is_operator_for_key(&_operator, &_token_holder));

    let val: bool = _operator == _token_holder || (default_op && !revoke_op) || is_op_val;

    set_key::<bool>(&is_operator_for_main(&_operator, &_token_holder), val); 

    return val;
}

/// Checks the authorize operator.
pub fn _authorize_operator(_operator: AccountHash, _holder: AccountHash) -> *const c_char {
   
    if ! (_operator == _holder) {

           set_key::<U256>(&logging_key(),4.into());
         
           set_key::<bool>(&is_operator_for_key(&_operator, &_operator), false);
         
           return "ERC777: authorizing self as operator".as_ptr() as *const c_char;
       
    }

     let default_operator: Vec<AccountHash> = get_key::<Vec<AccountHash>>(&default_operator_key());
    
     let mut doperator: Option<AccountHash> = None;

     for elem in &default_operator {

       if *elem == _operator {

           doperator = Some(*elem);

           break;

       }

     }

   if ! (doperator == None)  {
         remove_key(&revoke_operator_key(&_operator, &_holder));

   } else {

          set_key::<bool>(&is_operator_for_key(&_operator, &_holder), true);
     }
    set_key::<U256>(&logging_key(),6.into());


    let _revoke_op: bool = get_key::<bool>(&revoke_operator_key(&_operator, &_holder));

    let _is_op_val: bool = get_key::<bool>(&is_operator_for_key(&_operator, &_holder));

    let _val: bool = _operator == _holder || (! (doperator == None) && !_revoke_op) || _is_op_val;

    set_key::<bool>(&is_operator_for_main(&_operator, &_holder), _val);
    
    return "true".as_ptr() as *const c_char;   
}

/// Gets the allowance.
pub fn _allowance(_holder: AccountHash, _spender: AccountHash) {
   
    let val: U256 = get_key::<U256>(&allowance_key(&_holder, &_spender));
    ret(val);   
}

/// revokes operator permission
pub fn _revoke_operator(_operator: AccountHash, _holder: AccountHash) -> *const c_char {
     
    if ! (_operator == _holder) {

          set_key::<U256>(&logging_key(),8.into());
          
          set_key::<bool>(&is_operator_for_key(&_operator, &_operator), true); 

          return "ERC777: revoking self as operator".as_ptr() as *const c_char;  
    }

   let default_operator: Vec<AccountHash> = get_key::<Vec<AccountHash>>(&default_operator_key());

   let mut doperator: Option<AccountHash> = None;

     for elem in &default_operator {

       if *elem == _operator {

           doperator = Some(*elem);

           break;

       }

     }

   if ! (doperator == None)  {

          set_key::<bool>(&revoke_operator_key(&_operator, &_holder), true);
   } else {
        
         remove_key(&is_operator_for_key(&_operator, &_holder));  
        
    }

    set_key::<U256>(&logging_key(),9.into());

     let _revoke_op: bool = get_key::<bool>(&revoke_operator_key(&_operator, &_holder));

    let _is_op_val: bool = get_key::<bool>(&is_operator_for_key(&_operator, &_holder));

    let _val: bool = _operator == _holder || (! (doperator == None) && !_revoke_op) || _is_op_val;

    set_key::<bool>(&is_operator_for_main(&_operator, &_holder), _val);
   
     return "true".as_ptr() as *const c_char;
}

/// Sets the allowance key
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
/// Moves the data
pub fn _move(_operator: AccountHash, _from: AccountHash, _to: AccountHash, _amount: U256, _user_data:Bytes, _operator_data: Bytes) -> *const c_char {
	
    _before_token_transfer(_operator, _from, _to, _amount);

     let from_balance: U256 = get_key::<U256>(&balance_key(&_from));

     if from_balance >= _amount {

         return "ERC777: transfer amount exceeds balance".as_ptr() as *const c_char;
     }

     set_key(&balance_key(&_from),get_key::<U256>(&balance_key(&_from)).saturating_sub(_amount));

     set_key(&balance_key(&_to),get_key::<U256>(&balance_key(&_to)).saturating_add(_amount));
     
     return "true".as_ptr() as *const c_char;

}

/// Mints the token.
pub fn _mint(_account: AccountHash, _amount: U256, _data:Bytes, _operator_data:Bytes) -> *const c_char {

	
     _mintcheck(_account, _amount, _data, _operator_data, true)
}

pub fn _mintcheck(_account: AccountHash, _amount: U256, _data: Bytes, _operator_data: Bytes, _require_reception_ack: bool) -> *const c_char {

    if  _exists_owner(_account) {

    	 return "ERC777: mint to the zero address".as_ptr() as *const c_char;
    }

    set_key(&"total_supply",get_key::<U256>(&"total_supply").saturating_add(_amount));   
   // println!("balance : {}",&balance_key(&_account));        
 
    set_key::<U256>(&balance_key(&_account),get_key::<U256>(&balance_key(&_account)).saturating_add(_amount));

    return "true".as_ptr() as *const c_char;
}

/// Sends the token
pub fn _send(_from: AccountHash, _to: AccountHash, _amount: U256, _data: Bytes, _operator_data: Bytes, _require_reception_ack: bool) -> *const c_char {
          
           
            if  _exists_owner(_from) {

    	      return "ERC777: send from the zero address".as_ptr() as *const c_char;
            }

            if  _exists_owner(_to) {

    	      return "ERC777: send to the zero address".as_ptr() as *const c_char;
            }


            return "true".as_ptr() as *const c_char;
            // set_key(&balance_key(&from_value),get_key::<U256>(&balance_key(&from_value)).saturating_sub(amount_value)); 
         
            // set_key(&balance_key(to), get_key::<U256>(&balance_key(&to)).saturating_sub(amount_value));
         

}

/// Burns the token
pub fn _burn(_from: AccountHash, _amount: U256, _data: Bytes, _operator_data: Bytes) -> *const c_char {
        
        if  _exists_owner(_from) {

    	      return "ERC777: burn from the zero address".as_ptr() as *const c_char;
        }          

        return "true".as_ptr() as *const c_char;

}

/// Aproves the holder and spender.
pub fn _approve(_holder: AccountHash, _spender: AccountHash, _value: U256) -> *const c_char {
        
        if  _exists_owner(_holder) {

    	      return "ERC777: approve from the zero address".as_ptr() as *const c_char;
        }


        if  _exists_owner(_spender) {

            return "ERC777: approve from the zero address".as_ptr() as *const c_char;
        }

       _set_allowance_key(_holder, _spender, _value);     

       return "true".as_ptr() as *const c_char;    

}
