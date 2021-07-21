use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, AsymmetricType, CLTyped, PublicKey,
    RuntimeArgs, U256, U512,

    bytesrepr::{Bytes},
};

pub mod token_cfg {
    use super::*;
    pub const NAME: &str = "ERC777";
    pub const SYMBOL: &str = "ERC";
    pub const DECIMALS: u8 = 18;
    pub fn total_supply() -> U256 {
        1_000.into()
    }
    pub fn total_granularity() -> U256 {
        1_000.into()
    }
    pub fn token_default_operators() -> Vec<AccountHash> {
      //  Vec(AccountHash::new([42; 32])).into()
        let val: Vec<AccountHash> = Vec::new();
        return val; 
     }
    pub fn token_message() -> String {
       return "special_value".to_string(); 
    }  
 
}

pub struct Sender(pub AccountHash);

pub struct Token {
    context: TestContext,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
}

impl Token {
    pub fn deployed() -> Token {
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();
        let alic = ali.clone();
        let bobc = bob.clone(); 
      //   const MY_ACCOUNT: [u8; 32] = [7u8; 32];
        let mut context = TestContextBuilder::new()
            .with_public_key(ali, U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob, U512::from(500_000_000_000_000_000u64))
           //  .with_account(MY_ACCOUNT, U512::from(128_000_000))
            .build();
        let session_code = Code::from("../../target/wasm32-unknown-unknown/release/contract.wasm");
        let session_args = runtime_args! {
            "token_name" => token_cfg::NAME,
            "token_symbol" => token_cfg::SYMBOL,
          //  "token_decimals" => token_cfg::DECIMALS,
          //  "token_total_supply" => token_cfg::total_supply(),
          //  "token_granularity" =>  token_cfg::total_granularity(),
          //  "token_default_operators" => token_cfg::token_default_operators(),
          //  "message" => token_cfg::token_message()
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(alic.to_account_hash())
            .with_authorization_keys(&[alic.to_account_hash()])
            .build();
        context.run(session);
        Token {
            context,
            ali: alic.to_account_hash(),
            bob: bobc.to_account_hash(),
            joe: joe.to_account_hash(),
        }
    }

    fn contract_hash(&self) -> Hash {
        self.context
            .query(self.ali, &[format!("{}_hash", token_cfg::NAME)])
            .unwrap_or_else(|_| panic!("{} contract not found", token_cfg::NAME))
            .into_t()
            .unwrap_or_else(|_| panic!("{} has wrong type", token_cfg::NAME))
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.ali, &[token_cfg::NAME.to_string(), name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.contract_hash(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    pub fn name(&self) -> String {
        self.query_contract("name").unwrap()
    }

    pub fn symbol(&self) -> String {
        self.query_contract("symbol").unwrap()
    }

    pub fn decimals(&self) -> u8 {
        self.query_contract("decimals").unwrap()
    }

    pub fn balance_of(&self, account: AccountHash) -> U256 {
        let key = format!("balance_{}", account);
       println!("{}",key); 
       self.query_contract(&key).unwrap_or_default()
    }

    pub fn allowance(&self, owner: AccountHash, spender: AccountHash) -> U256 {
        let key = format!("allowances_{}_{}", owner, spender);
        self.query_contract(&key).unwrap_or_default()
    }

    pub fn transfer(&mut self, recipient: AccountHash, amount: U256, sender: Sender) {
        self.call(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount
            },
        );
    }

    pub fn approve(&mut self, spender: AccountHash, amount: U256, sender: Sender) {
         
       self.call(
            sender,
            "approve",
            runtime_args! {
                "spender" => spender,
                "value" => amount
            },
        );
    }

    pub fn mint_token(&mut self, token_holder: AccountHash, amount: U256, sender: Sender) {
       let _data:Bytes = vec![0x41u8, 0x41u8, 0x42u8].into(); 
       
      let _operator_data:Bytes = vec![0x59u8, 0x59u8, 0x59u8].into();      
          
       self.call(
            sender,
            "mint",
            runtime_args! {
                "token_holder" => token_holder,
                "amount" => amount,
                "data" => _data.clone(),
                "operator_data" => _operator_data.clone(),   
            },
        );
    }


    

    pub fn transfer_from(
        &mut self,
        owner: AccountHash,
        recipient: AccountHash,
        amount: U256,
        sender: Sender,
    ) {
        self.call(
            sender,
            "transfer_from",
            runtime_args! {
                "holder" => owner,
                "recipient" => recipient,
                "amount" => amount
            },
        );
    }
}
