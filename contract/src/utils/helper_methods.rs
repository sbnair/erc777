
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

/// Checks the authorize operator.
pub fn _authorize_operator(_operator: AccountHash, _holder: AccountHash) -> *const c_char {
     
    if _operator == _holder {

           set_key::<bool>(&is_operator_for_key(&_operator, &_operator), false);
         
           return "ERC777: authorizing self as operator".as_ptr() as *const c_char;
       
    }

     let mut default_operator: Vec<AccountHash> = get_key::<Vec<AccountHash>>(&default_operator_key());
    
     let mut doperator: Option<AccountHash> = None;

     for elem in &default_operator {

       if *elem == _operator {

           doperator = Some(*elem);

           break;

       }

     }
 
   if  doperator == None {

         default_operator.push(_operator);

         doperator = Some(_operator);

         set_key::<Vec<AccountHash>>(&default_operator_key(), default_operator);    
   }  

   if ! (doperator == None)  {
         remove_key(&revoke_operator_key(&_operator, &_holder));

   } else {

        set_key::<bool>(&is_operator_for_key(&_operator, &_holder), true);
     }

  
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
     
    if  _operator == _holder {

          set_key::<bool>(&is_operator_for_key(&_operator, &_operator), true); 

          return "ERC777: revoking self as operator".as_ptr() as *const c_char;  
    }

   let mut default_operator: Vec<AccountHash> = get_key::<Vec<AccountHash>>(&default_operator_key());

   let mut doperator: Option<AccountHash> = None;

     for elem in &default_operator {

       if *elem == _operator {

           doperator = Some(*elem);

           break;

       }

     }

   if ! (doperator == None)  {

         let index = default_operator.iter().position(|x| *x == doperator.unwrap()).unwrap();

         default_operator.remove(index); 
          
         set_key::<Vec<AccountHash>>(&default_operator_key(), default_operator);
  
         set_key::<bool>(&revoke_operator_key(&_operator, &_holder), true);

   } else {
        
         remove_key(&is_operator_for_key(&_operator, &_holder));  
        
    }

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

    // Logic to be written after 1820 implementation
}

#[allow(unused)]
pub fn _call_tokens_received(_operator: AccountHash, _from: AccountHash, _to: AccountHash, _amount: U256, _data: Bytes, _operator_data: Bytes, _require_reception_ack: bool) {

   // Logic to be written after 1820 implementation
}

#[allow(unused)]
pub fn _before_token_transfer(_operator: AccountHash, _from: AccountHash, _to: AccountHash, _amount: U256) {

  // Logic to be written after 1820 implementation

}
/// Moves the data
pub fn _move(_operator: AccountHash, _from: AccountHash, _to: AccountHash, _amount: U256, _user_data:Bytes, _operator_data: Bytes) -> *const c_char {
	
    _before_token_transfer(_operator, _from, _to, _amount);

     let from_balance: U256 = get_key::<U256>(&balance_key(&_from));

     if from_balance < _amount {

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

            let _move_data: Bytes = _data.clone();

            let _received_data: Bytes = _data.clone();

            let _move_operator_data: Bytes = _operator_data.clone();

            let _received_operator_data: Bytes = _operator_data.clone(); 

            _call_tokens_to_send(_from, _from, _to, _amount, _data, _operator_data); 

            _move(_from, _from, _to, _amount, _move_data, _move_operator_data);

            _call_tokens_received(_from, _from, _to, _amount, _received_data, _received_operator_data, _require_reception_ack);   


            return "true".as_ptr() as *const c_char;    

}

/// Burns the token
pub fn _burn(_from: AccountHash, _amount: U256, _data: Bytes, _operator_data: Bytes) -> *const c_char {
        
        if  _exists_owner(_from) {

    	      return "ERC777: burn from the zero address".as_ptr() as *const c_char;
        }
	
	let _zero_addr: AccountHash = AccountHash::from_formatted_str("account-hash-0000000000000000000000000000000000000000000000000000000000000000").unwrap_or_default();
	
	_call_tokens_to_send(_from, _from, _zero_addr, _amount, _data, _operator_data);
	
	_before_token_transfer(_from, _from, _zero_addr, _amount); 
	
	let _from_balance: U256 = get_key::<U256>(&balance_key(&_from));
	
	if _from_balance < _amount {
		
	    return "ERC777: burn amount exceeds balance".as_ptr() as *const c_char;
	}	

	set_key(&balance_key(&_from),get_key::<U256>(&balance_key(&_from)).saturating_sub(_amount));
	
	set_key(&"total_supply",get_key::<U256>(&"total_supply").saturating_sub(_amount));	
	 
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

      set_key::<U256>(&allowance_key(&_holder, &_spender), _value);

       return "true".as_ptr() as *const c_char;    

}
