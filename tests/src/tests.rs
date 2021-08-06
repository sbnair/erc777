use casper_types::{
    account::AccountHash 
};
use crate::erc777::{token_cfg, Sender, Token};

#[test]
fn test_erc777_deploy() {
    let t = Token::deployed();
    assert_eq!(t.name(), token_cfg::NAME);
    assert_eq!(t.symbol(), token_cfg::SYMBOL);
        
    println!("Account of Ali: {}", t.ali);
    println!("Account of Bob: {}", t.bob);
    println!("Account of Joe: {}", t.joe);
//    assert_eq!(t.decimals(), token_cfg::DECIMALS);
    assert_eq!(t.total_supply(), token_cfg::total_supply());  // Token supply is set to 5 and so the default balance of Ali and Bob accounts are set to 5.
 
}

#[test]
fn test_erc777_transfer() {
    let amount = 1.into();
    let mut t = Token::deployed();
    t.transfer(t.bob, amount, Sender(t.ali));
  
    println!("Balance: {}", t.balance_of(t.ali));
    assert_eq!(t.balance_of(t.ali), 4.into());
    assert_eq!(t.balance_of(t.bob), 6.into());
}


#[test]
fn test_erc777_operator() {
    let mut t = Token::deployed();
    
    println!("is_operator_for: {}", t.is_operator_for(t.ali,t.ali));

    assert_eq!(t.is_operator_for(t.ali,t.ali), true); // Ali is the default operator 

    assert_eq!(t.is_operator_for(t.bob,t.bob), false); // bob is not the default operator 
}

#[test]
fn approve_and_transferfrom_invalidtoken()
{
    let mut t = Token::deployed();
    t.mint_token(t.bob, 1.into(), Sender(t.ali));
    t.mint_token(t.bob, 2.into(), Sender(t.ali));
    println!("Mint token: {}", t.bob);
    println!("Balance Token: {}", t.balance_of(t.bob));
    
    assert_eq!(t.balance_of(t.bob), 8.into());                 
  
    // Approving invalid token
   t.approve(t.bob, 3.into(), Sender(t.ali));                 
  //  assert_ne!(t.owner_of(3.into()), t.bob);                   
    println!("Mint token1: {}",t.joe);
    // TransferFrom invalid token
   t.transfer_from(t.ali, t.joe, 3.into() ,Sender(t.bob));
    assert_eq!(t.balance_of(t.joe), 3.into());                  
    assert_eq!(t.balance_of(t.ali), 2.into());                  
}

#[test]
fn test_erc777_transfer_too_much() {
    let amount = 3.into();

    let mut t = Token::deployed();

    println!("Before Balances of Ali {}", t.balance_of(t.ali));

    println!("Before Balances of Bob {}", t.balance_of(t.bob));
 
    t.transfer(t.bob, amount, Sender(t.ali));

    assert_eq!(t.balance_of(t.ali), 2.into());

    assert_eq!(t.balance_of(t.bob), 8.into()); 

    println!("Balances of Ali {}", t.balance_of(t.ali));

    println!("Balances of Bob {}", t.balance_of(t.bob)); 
}

#[test]
fn test_erc777_approve() {
    let amount = 1.into();
    let mut t = Token::deployed();
    t.approve(t.bob, amount, Sender(t.ali));

    assert_eq!(t.allowance(t.ali, t.bob), amount);
   
}

#[test]
fn test_erc777_transfer_from() {
    let allowance = 10.into();

    let amount = 3.into();

    let mut t = Token::deployed();

    t.approve(t.bob, allowance, Sender(t.ali));

    t.transfer_from(t.ali, t.joe, amount, Sender(t.bob));

    assert_eq!(t.balance_of(t.ali), 2.into());

    assert_eq!(t.balance_of(t.bob), 5.into());

    assert_eq!(t.balance_of(t.joe), 3.into());

}

#[test]
/// Burn function which will display balance and total supply before burning and also the same parameters are displayed after burning the passed amount. 
fn test_erc777_burn() {
    let amount = 3.into();
    
    let mut t = Token::deployed();
    
    println!("Before Balance of {}", t.balance_of(t.ali));
    
    println!("Before Token Supply of {}", t.total_supply());

    t.burn_token(amount, t.bob, Sender(t.ali));

    println!("After Balance of {}", t.balance_of(t.bob));

    println!("After Token Supply of {}", t.total_supply());
    
    assert_eq!(t.balance_of(t.bob), 2.into());
    
    assert_eq!(t.total_supply(), 2.into());
  }

#[test]
fn test_erc777_transfer_from_too_much() {

    let amount = token_cfg::total_supply().checked_add(1.into()).unwrap();
 
    let mut t = Token::deployed();
    
    println!("Before Balance of {}", t.balance_of(t.ali));
  
   println!("Before Balance of {}", t.balance_of(t.bob));

   println!("Before Balance of {}", t.balance_of(t.joe));

   t.transfer_from(t.ali, t.joe, amount, Sender(t.bob));

   assert_eq!(t.balance_of(t.ali), 5.into());

   assert_eq!(t.balance_of(t.bob), 5.into());

   assert_eq!(t.balance_of(t.joe), 0.into());  

   println!("After Balance of {}", t.balance_of(t.ali));
  
   println!("After Balance of {}", t.balance_of(t.bob));

   println!("After Balance of {}", t.balance_of(t.joe));
}

#[test]
fn test_erc777_authorize_operator() {
    let mut t = Token::deployed();
    t.authorize_operator(t.ali, Sender(t.ali));
    
     assert_eq!(t.is_operator_for(t.ali,t.ali), true);

}

#[test]
fn test_erc777_revoke_operator() {
    let mut t = Token::deployed();
    t.revoke_operator(t.ali, Sender(t.ali));

    assert_eq!(t.is_operator_for(t.ali,t.ali), true);

}

#[test]
fn test_erc777_default_operators() {
    let t = Token::deployed();
    let val: Vec<AccountHash> = t.default_operators();

    println!("Default Operators: {}", val[0]);
    assert_eq!(val[0], t.ali);

}


#[test]
fn test_erc777_auth_revoke_operators() {
    let mut t = Token::deployed();
 
   
   assert_eq!(t.is_operator_for(t.ali,t.ali), true); // Default operator is ali
   assert_eq!(t.is_operator_for(t.ali,t.bob), false); // bob is not a default operator

   t.authorize_operator(t.bob, Sender(t.bob)); // Authorized Bob as the default operator

  
   assert_eq!(t.is_operator_for(t.bob,t.bob), false); // Bob as a default operator works

   t.revoke_operator(t.ali, Sender(t.ali)); // Removes ali as a default operator

   assert_eq!(t.is_operator_for(t.ali,t.bob), false); // ali is not a default operator.
}


#[test]
fn test_erc777_auth_revoke_burn_send_operators() {
    let mut t = Token::deployed();
    
    let amount = 2.into();

   assert_eq!(t.is_operator_for(t.ali,t.ali), true); // Default operator is ali
   assert_eq!(t.is_operator_for(t.ali,t.bob), false); // bob is not a default operator

   t.authorize_operator(t.bob, Sender(t.ali)); // Authorized Bob as the default operator by Ali
   

   assert_eq!(t.is_operator_for(t.bob,t.ali), true); // Bob is authorized as default operator

   t.operator_send(t.bob, amount, Sender(t.ali));  // Send works as bob is an operator

   println!("Authorize (send) Balance of {}", t.balance_of(t.ali));

   println!("Authorize (send) Balance of {}", t.balance_of(t.bob));


   t.operator_burn(t.bob, amount, Sender(t.ali)); // Burn works as bob is an operator

   println!("Authorize (burn) Balance of {}", t.balance_of(t.ali));

   println!("Authorize (burn) Balance of {}", t.balance_of(t.bob));

   t.revoke_operator(t.bob, Sender(t.ali)); // Revokes bob as a default operator

   assert_eq!(t.is_operator_for(t.bob,t.ali), false); // bob is not a default operator.


   t.operator_send(t.bob, amount, Sender(t.ali)); // Send function doesn't work as bob is not authorized now.

   println!("Revoke (send) Balance of {}", t.balance_of(t.ali));

   println!("Revoke (send) Balance of {}", t.balance_of(t.bob));


   t.operator_burn(t.bob, amount, Sender(t.ali)); // Burn function doesn't work as bob is not authorized now.

   println!("Revoke (burn) Balance of {}", t.balance_of(t.ali));

   println!("Revoke (burn) Balance of {}", t.balance_of(t.bob));
}




