#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

mod keys;
mod utils;
mod ERC820Registry;
mod ERC777TokensRecipient;
mod ERC777TokensSender;
mod ERC777;

//  use casper_contract::{
//    contract_api::{runtime, storage},
// };
// use casper_types::{Key, URef};


pub mod token {
    use pwasm_std::String;
    use pwasm_abi::types::*;
    use crate::keys::*;
    use crate::utils::*;

    pub struct ERC777Contract;

    impl ERC777Contract {
        fn require_multiple(&mut self, amount: &U256) {
            require(amount % self.granularity() == U256::zero(),
                    "Amount is not a multiple of granularity");
        }

        fn require_sufficient_funds(&mut self, address: &Address, amount: &U256) {
            require(read_balance_of(address) >= *amount, "Not enough funds");
        }

        fn is_operator_for(&mut self, operator: &Address, token_holder: &Address) -> bool {
            if operator == token_holder {
                return true;
            }
            U256::from_big_endian(&pwasm_ethereum::read(&operator_map_key(operator, token_holder))) == U256::one()
        }

        fn do_send(&mut self, operator: &Address, from: &Address, to: &Address, amount: &U256, data: &Vec<u8>, operatorData: &Vec<u8>) {
            self.require_multiple(amount);
            self.require_sufficient_funds(from, amount);
            require(to != &H160::zero(), "Cannot send to 0x0");

            let mut registry = ERC820RegistryClient::new(Address::from([0x82, 0x0b, 0x58, 0x6C, 0x8C, 0x28, 0x12, 0x53, 0x66, 0xC9, 0x98, 0x64, 0x1B, 0x09, 0xDC, 0xbE, 0x7d, 0x4c, 0xBF, 0x06]));

            let sender_hook = registry.getInterfaceImplementer(*from, ERC777TokensSender_key().into());

            // Call ERC777 sender hook if present
            if sender_hook != Address::zero() {
                let mut sender = ERC777TokensSenderClient::new(sender_hook);
                sender.tokensToSend(
                    *operator,
                    *from,
                    *to,
                    *amount,
                    data.clone(),
                    operatorData.clone());
            }

            pwasm_ethereum::write(&balance_key(from),
                                  &read_balance_of(from)
                                      .saturating_sub(*amount).into());

            pwasm_ethereum::write(&balance_key(to),
                                  &read_balance_of(to)
                                      .saturating_add(*amount).into());

            let recipient_hook = registry.getInterfaceImplementer(*to, ERC777TokensRecipient_key().into());

            // Call ERC777 recipient hook if present
            if recipient_hook != Address::zero() {
                let mut recipient = ERC777TokensRecipientClient::new(recipient_hook);
                recipient.tokensReceived(
                    *operator,
                    *from,
                    *to,
                    *amount,
                    data.clone(),
                    operatorData.clone());
            }

            self.Sent(*operator,
                      *from,
                      *to,
                      *amount,
                      data.clone(),
                      operatorData.clone());
            if erc20_compatible() {
                self.Transfer(*from, *to, *amount);
            }
        }

        fn do_burn(&mut self, operator: &Address, token_holder: &Address, amount: &U256, data: &Vec<u8>, operatorData: &Vec<u8>) {
            self.require_multiple(amount);
            self.require_sufficient_funds(token_holder, amount);

            pwasm_ethereum::write(&balance_key(token_holder),
                                  &read_balance_of(&token_holder)
                                      .saturating_sub(*amount).into());

            pwasm_ethereum::write(&total_supply_key(),
                                  &self.totalSupply()
                                      .saturating_sub(*amount).into());

            self.Burned(*operator,
                        *token_holder,
                        *amount,
                        data.clone(),
                        operatorData.clone());

            if erc20_compatible() {
                self.Transfer(*token_holder, Address::zero(), *amount);
            }
        }
    }

    use crate::ERC777::*;
    use crate::ERC777TokensRecipient::*;
    use crate::ERC777TokensSender::*;
    use crate::ERC820Registry::*;

    impl ERC777Interface for ERC777Contract {
        fn constructor(&mut self, name: String, symbol: String, granularity: U256) {
            pwasm_ethereum::write(&owner_key(), &H256::from(pwasm_ethereum::sender()).into());
            write_string(&name_key(), &name);
            write_string(&symbol_key(), &symbol);
            pwasm_ethereum::write(&granularity_key(), &granularity.into());
        }

        fn mint(&mut self, tokenHolder: Address, amount: U256, operatorData: Vec<u8>) {
            require_owner();
            self.require_multiple(&amount);
            
            pwasm_ethereum::write(&total_supply_key(),
                                  &self.totalSupply()
                                      .saturating_add(amount).into());

            pwasm_ethereum::write(&balance_key(&tokenHolder),
                                  &read_balance_of(&tokenHolder)
                                      .saturating_add(amount).into());

            self.Minted(pwasm_ethereum::sender(), tokenHolder, amount, operatorData);
            if erc20_compatible() {
                self.Transfer(Address::zero(), tokenHolder, amount);
            }
        }

        fn name(&mut self) -> String {
            read_string(&name_key())
        }

        fn symbol(&mut self) -> String {
            read_string(&symbol_key())
        }

        fn totalSupply(&mut self) -> U256 {
            U256::from_big_endian(&pwasm_ethereum::read(&total_supply_key()))
        }

        fn balanceOf(&mut self, owner: Address) -> U256 {
            read_balance_of(&owner)
        }

        fn granularity(&mut self) -> U256 {
            U256::from_big_endian(&pwasm_ethereum::read(&granularity_key()))
        }

        fn defaultOperators(&mut self) -> Vec<Address> {
            Vec::new()
        }

        fn authorizeOperator(&mut self, operator: Address) {
            let sender = pwasm_ethereum::sender();
            require(operator != sender, "Cannot authorize yourself as an operator");
            pwasm_ethereum::write(&operator_map_key(&operator, &sender),
                                  &U256::one().into());
            self.AuthorizedOperator(operator, sender);
        }

        fn revokeOperator(&mut self, operator: Address) {
            let sender = pwasm_ethereum::sender();
            require(operator != sender, "Cannot revoke yourself as an operator");
            pwasm_ethereum::write(&operator_map_key(&operator, &sender),
                                  &U256::zero().into());
            self.RevokedOperator(operator, sender);
        }

        fn isOperatorFor(&mut self, operator: Address, tokenHolder: Address) -> bool {
            self.is_operator_for(&operator, &tokenHolder)
        }

        fn send(&mut self, to: Address, amount: U256, data: Vec<u8>) {
            let from = pwasm_ethereum::sender();
            self.do_send(&from, &from, &to, &amount, &data, &Vec::new());
        }

        fn operatorSend(&mut self, from: Address, to: Address, amount: U256, data: Vec<u8>, operatorData: Vec<u8>)
        {
            let operator = pwasm_ethereum::sender();
            require(self.is_operator_for(&operator, &from), "Not an operator");
            self.do_send(&operator, &from, &to, &amount, &data, &operatorData);
        }

        fn burn(&mut self, amount: U256, data: Vec<u8>) {
            let sender = pwasm_ethereum::sender();
            self.do_burn(&sender, &sender, &amount, &data, &Vec::new());
        }

        fn operatorBurn(&mut self, from: Address, amount: U256, data: Vec<u8>, operatorData: Vec<u8>) {
            let operator = pwasm_ethereum::sender();
            require(self.is_operator_for(&operator, &from), "Not an operator");
            self.do_burn(&operator, &from, &amount, &data, &operatorData);
        }

        fn disableERC20(&mut self) {
            require_owner();
            pwasm_ethereum::write(&erc20_compatibility_key(), &U256::zero().into());
        }
        fn enableERC20(&mut self) {
            require_owner();
            pwasm_ethereum::write(&erc20_compatibility_key(), &U256::one().into());
        }
    }
}

// Declares the dispatch and dispatch_ctor methods
use pwasm_abi::eth::EndpointInterface;

/// Will be described in the next step
#[no_mangle]
pub fn deploy() {
    let mut endpoint = crate::ERC777::ERC777Endpoint::new(token::ERC777Contract {});
    endpoint.dispatch_ctor(&pwasm_ethereum::input());
}

/// The call function is the main function of the *deployed* contract
#[no_mangle]
pub fn call() {
    let mut endpoint = crate::ERC777::ERC777Endpoint::new(token::ERC777Contract {});
    pwasm_ethereum::ret(&endpoint.dispatch(&pwasm_ethereum::input()));
}



