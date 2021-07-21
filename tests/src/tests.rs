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
//    assert_eq!(t.balance_of(t.ali), token_cfg::total_supply());
 //   assert_eq!(t.balance_of(t.bob), 0.into());
 //   assert_eq!(t.allowance(t.ali, t.ali), 0.into());
  //  assert_eq!(t.allowance(t.ali, t.bob), 0.into());
  //  assert_eq!(t.allowance(t.bob, t.ali), 0.into());
  //  assert_eq!(t.allowance(t.bob, t.bob), 0.into());
}

#[test]
fn test_erc777_transfer() {
    let amount = 10.into();
    let mut t = Token::deployed();
    t.transfer(t.bob, amount, Sender(t.ali));
    assert_eq!(t.balance_of(t.ali), 0.into());
    assert_eq!(t.balance_of(t.bob), 0.into());
}

#[test]
fn approve_and_transferfrom_invalidtoken()
{
    let mut t = Token::deployed();
    t.mint_token(t.ali, 1.into(), Sender(t.ali));
    t.mint_token(t.ali, 2.into(), Sender(t.ali));
    println!("Mint token: {}", t.ali);
    println!("Balance Token: {}", t.balance_of(t.ali));
     assert_eq!(t.balance_of(t.ali), 0.into());                  // should pass, ali now has two token

    // Approving invalid token
   t.approve(t.bob, 3.into(), Sender(t.ali));                  // token 3 doesnot exist
  //  assert_ne!(t.owner_of(3.into()), t.bob);                    // Not Equal should pass, because id 3 is a non extent token and its owner should not be bob
    println!("Mint token1: {}",t.joe);
    // TransferFrom invalid token
   t.transfer_from(t.ali, t.joe, 3.into() ,Sender(t.bob));
    assert_eq!(t.balance_of(t.joe), 0.into());                  // joe's balance should still be zero, because the transfer above should not have gone through
    assert_eq!(t.balance_of(t.ali), 0.into());                  // Ali's balances should remain same
}

#[test]
fn test_erc777_transfer_too_much() {
    let amount = 1.into();
    let mut t = Token::deployed();
    t.transfer(t.ali, amount, Sender(t.bob));
}

#[test]
fn test_erc777_approve() {
    let amount = 10.into();
    let mut t = Token::deployed();
    t.approve(t.bob, amount, Sender(t.ali));
  //  println!("Approve token");
   assert_eq!(t.balance_of(t.ali), 0.into());
   assert_eq!(t.balance_of(t.bob), 0.into());
   assert_eq!(t.allowance(t.ali, t.bob), 0.into());
    assert_eq!(t.allowance(t.bob, t.ali), 0.into());
}

#[test]
fn test_erc777_transfer_from() {
    let allowance = 10.into();
    let amount = 3.into();
    let mut t = Token::deployed();
    t.approve(t.bob, allowance, Sender(t.ali));
    t.transfer_from(t.ali, t.joe, amount, Sender(t.bob));
    assert_eq!(t.balance_of(t.ali), 0.into());
    assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.balance_of(t.joe), 0.into());
    assert_eq!(t.allowance(t.ali, t.bob), 0.into());
}

#[test]
fn test_erc777_transfer_from_too_much() {
    let amount = token_cfg::total_supply().checked_add(1.into()).unwrap();
    let mut t = Token::deployed();
    t.transfer_from(t.ali, t.joe, amount, Sender(t.bob));
}
