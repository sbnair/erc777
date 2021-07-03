use crate::*;
    use crate::ERC777::ERC777Interface;
    use pwasm_std::String;
    use pwasm_abi::types::*;
    use pwasm_test::ext_reset;
    use pwasm_std::keccak;

    static TEST_NAME: &'static str = "TestToken";
    static TEST_SYMBOL: &'static str = "TTK";
    static TEST_GRANULARITY: u64 = 100000000000000;

    fn test_owner_address() -> Address {
        Address::from([0xea, 0x67, 0x4f, 0xdd, 0xe7, 0x14, 0xfd, 0x97, 0x9d, 0xe3, 0xed, 0xf0, 0xf5, 0x6a, 0xa9, 0x71, 0x6b, 0x89, 0x8e, 0xc8])
    }

    fn init_test_contract() -> token::ERC777Contract {
        let mut contract = token::ERC777Contract {};
        // Here we're creating an External context using ExternalBuilder and set the `sender` to the `owner_address`
        ext_reset(|e| e.sender(test_owner_address()));

        let name = String::from(TEST_NAME);
        let symbol = String::from(TEST_SYMBOL);
        contract.constructor(name.clone(), symbol, U256::from(TEST_GRANULARITY));
        contract
    }

    #[test]
    fn should_set_and_retrieve_the_correct_token_name() {
        let mut contract = init_test_contract();
        assert_eq!(contract.name(), TEST_NAME);
    }

    #[test]
    fn should_set_and_retrieve_the_correct_token_symbol() {
        let mut contract = init_test_contract();
        assert_eq!(contract.symbol(), TEST_SYMBOL);
    }

    #[test]
    fn initial_total_supply_should_be_zero() {
        let mut contract = init_test_contract();
        assert_eq!(contract.totalSupply(), U256::zero());
    }

    #[test]
    fn should_set_and_retrieve_granularity() {
        let mut contract = init_test_contract();
        assert_eq!(contract.granularity(), U256::from(TEST_GRANULARITY));
    }

    #[test]
    fn should_authorize_operator() {
        let mut contract = init_test_contract();
        let operator = Address::from_low_u64_le(1u64);
        assert_eq!(contract.isOperatorFor(test_owner_address(), test_owner_address()), true);
        assert_eq!(contract.isOperatorFor(operator, test_owner_address()), false);
        contract.authorizeOperator(operator);
        assert_eq!(contract.isOperatorFor(operator, test_owner_address()), true);
        contract.revokeOperator(operator);
        assert_eq!(contract.isOperatorFor(operator, test_owner_address()), false);
    }

    compiletime_keccak::compiletime_keccak!(hashed_string);

    #[test]
    fn compare_compile_time_to_runtime_keccak() {
        let hash = keccak(b"hashed_string");
        assert_eq!(hashed_string(), hash);
    }