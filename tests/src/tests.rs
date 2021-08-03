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
    assert_eq!(t.total_supply(), token_cfg::total_supply());
 
}

#[test]
fn test_erc777_transfer() {
    let amount = 10.into();
    let mut t = Token::deployed();
    t.transfer(t.bob, amount, Sender(t.ali));
  
    println!("Balance: {}", t.balance_of(t.ali));
    assert_eq!(t.balance_of(t.ali), 0.into());
    assert_eq!(t.balance_of(t.bob), 15.into());
}


#[test]
fn test_erc777_operator() {
    let mut t = Token::deployed();
    
    println!("is_operator_for: {}", t.is_operator_for(t.ali,t.ali));

    assert_eq!(t.is_operator_for(t.ali,t.ali), true);  
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
    assert_eq!(t.balance_of(t.joe), 0.into());                  
    assert_eq!(t.balance_of(t.ali), 5.into());                  
}

#[test]
fn test_erc777_transfer_too_much() {
    let amount = 6.into();

    let mut t = Token::deployed();

    println!("Before Balances of Ali {}", t.balance_of(t.ali));

    println!("Before Balances of Bob {}", t.balance_of(t.bob));
 
    t.transfer(t.bob, amount, Sender(t.ali));

    assert_eq!(t.balance_of(t.ali), 0.into());

    assert_eq!(t.balance_of(t.bob), 11.into()); 

    println!("Balances of Ali {}", t.balance_of(t.ali));

    println!("Balances of Bob {}", t.balance_of(t.bob)); 
}

#[test]
fn test_erc777_approve() {
    let amount = 10.into();
    let mut t = Token::deployed();
    t.approve(t.bob, amount, Sender(t.ali));

    assert_eq!(t.allowance(t.ali, t.bob),amount);
   
    println!("Allownce {}", t.allowance(t.ali, t.bob));
}

#[test]
fn test_erc777_transfer_from() {
    let allowance = 10.into();
    let amount = 3.into();
    let mut t = Token::deployed();
    t.approve(t.bob, allowance, Sender(t.ali));
    t.transfer_from(t.ali, t.joe, amount, Sender(t.bob));
    assert_eq!(t.balance_of(t.ali), 5.into());
    assert_eq!(t.balance_of(t.bob), 5.into());
    assert_eq!(t.balance_of(t.joe), 0.into());
    assert_eq!(t.allowance(t.ali, t.bob), 10.into());
}

#[test]
/// Burn function which will display balance and total supply before burning and also the same parameters are displayed after burning the passed amount. 
fn test_erc777_burn() {
    let amount = 6.into();
    
    let mut t = Token::deployed();
    
    println!("Before Balance of {}", t.balance_of(t.ali));
    
    println!("Before Token Supply of {}", t.total_supply());

    t.burn_token(amount, t.ali, Sender(t.ali));

    println!("After Balance of {}", t.balance_of(t.ali));

    println!("After Token Supply of {}", t.total_supply());
    
    assert_eq!(t.balance_of(t.ali), 0.into());
    
    assert_eq!(t.total_supply(), 0.into());


    println!("Allowance {}", t.allowance(t.ali, t.ali)); 
  }

#[test]
fn test_erc777_transfer_from_too_much() {

    let amount = token_cfg::total_supply().checked_add(1.into()).unwrap();
 
    let mut t = Token::deployed();
    
    println!("Before Balance of {}", t.balance_of(t.ali));
  
   println!("Before Balance of {}", t.balance_of(t.bob));

   println!("Before Balance of {}", t.balance_of(t.joe));

   t.transfer_from(t.ali, t.joe, amount, Sender(t.bob));

   assert_eq!(t.balance_of(t.ali), 0.into());

   assert_eq!(t.balance_of(t.bob), 5.into());

   assert_eq!(t.balance_of(t.joe), 6.into());  

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

  
   assert_eq!(t.is_operator_for(t.bob,t.bob), true); // Bob as a default operator works

   t.revoke_operator(t.ali, Sender(t.ali)); // Removes ali as a default operator

   assert_eq!(t.is_operator_for(t.ali,t.bob), false); // ali is not a default operator.
}




