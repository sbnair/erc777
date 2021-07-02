use crate::erc777::{token_cfg, Sender, Token};
use casper_engine_test_support::{Code, Error, SessionBuilder, TestContextBuilder, Value};
    use casper_types::{RuntimeArgs, runtime_args, U512};


#[test]
#[should_panic]
fn test_erc777_basic() {
let amount = token_cfg::total_supply().checked_add(1.into()).unwrap();
    let mut t = Token::deployed();
        assert_eq!(t.name(), token_cfg::NAME);
    assert_eq!(t.symbol(), token_cfg::SYMBOL);
    assert_eq!(t.decimals(), token_cfg::DECIMALS);
    assert_eq!(t.balance_of(t.ali), token_cfg::total_supply());
    assert_eq!(t.balance_of(t.bob), 0.into());
    assert_eq!(t.allowance(t.ali, t.ali), 0.into());
    assert_eq!(t.allowance(t.ali, t.bob), 0.into());
    assert_eq!(t.allowance(t.bob, t.ali), 0.into());
    assert_eq!(t.allowance(t.bob, t.bob), 0.into());   
  //  t.transfer_from(t.ali, t.joe, amount, Sender(t.bob));
//  let mut t = Token::deployed();
//  t.name();
}
  //  fn should_store_hello_world() {
    //    let mut context = TestContextBuilder::new()
      //      .with_account(MY_ACCOUNT, U512::from(128_000_000))
        //    .build();

        // The test framework checks for compiled Wasm files in '<current working dir>/wasm'.  Paths
        // relative to the current working dir (e.g. 'wasm/contract.wasm') can also be used, as can
        // absolute paths.
     

//#[test]
//fn test_erc777_deploy() {
   //  let amount = token_cfg::total_supply().checked_add(1.into()).unwrap();
   //  let mut t = Token::deployed();
 //    t.name();
  //   assert_eq!("ERC777",token_cfg::NAME);
    // assert_eq!(t.name(), token_cfg::NAME);
   // assert_eq!(t.symbol(), token_cfg::SYMBOL);
//    assert_eq!(t.decimals(), token_cfg::DECIMALS);
//    assert_eq!(t.balance_of(t.ali), token_cfg::total_supply());
 //   assert_eq!(t.balance_of(t.bob), 0.into());
 //   assert_eq!(t.allowance(t.ali, t.ali), 0.into());
 //   assert_eq!(t.allowance(t.ali, t.bob), 0.into());
 //   assert_eq!(t.allowance(t.bob, t.ali), 0.into());
 //   assert_eq!(t.allowance(t.bob, t.bob), 0.into());
//}

//#[test]
//fn test_erc777_transfer() {
  //  let amount = 10.into();
  //  let mut t = Token::deployed();
  //  t.transfer(t.bob, amount, Sender(t.ali));
   // assert_eq!(t.balance_of(t.ali), token_cfg::total_supply() - amount);
  //  assert_eq!(t.balance_of(t.bob), amount);
//}


//#[test]
//fn test_erc777_approve() {
  //  let amount = 10.into();
  //  let mut t = Token::deployed();
  //  t.approve(t.bob, amount, Sender(t.ali));
  //  assert_eq!(t.balance_of(t.ali), token_cfg::total_supply());
  //  assert_eq!(t.balance_of(t.bob), 0.into());
  //  assert_eq!(t.allowance(t.ali, t.bob), amount);
  //  assert_eq!(t.allowance(t.bob, t.ali), 0.into());
//}

#[test]
#[should_panic]
fn test_erc777_transfer_from() {
    let allowance = 10.into();
    let amount = 3.into();
    let mut t = Token::deployed();
    t.approve(t.bob, allowance, Sender(t.ali));
    t.transfer_from(t.ali, t.joe, amount, Sender(t.bob));
  //  assert_eq!(t.balance_of(t.ali), token_cfg::total_supply() - amount);
  //  assert_eq!(t.balance_of(t.bob), 0.into());
 //   assert_eq!(t.balance_of(t.joe), amount);
  //  assert_eq!(t.allowance(t.ali, t.bob), allowance - amount);
}

#[test]
#[should_panic]
fn test_erc777_transfer_from_too_much() {
    let amount = token_cfg::total_supply().checked_add(1.into()).unwrap();
    let mut t = Token::deployed();
    t.transfer_from(t.ali, t.joe, amount, Sender(t.bob));
}
