#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod proxy {
    use brush::contracts::{
        ownable::*, psp22::extensions::metadata::*, psp22::extensions::mintable::*,
    };
    use ink_lang::codegen::EmitEvent;
    use ink_lang::codegen::Env;
    use ink_prelude::string::ToString;
    use ink_storage::traits::SpreadAllocate;

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        amount: Balance,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage, PSP22MetadataStorage, OwnableStorage)]
    pub struct FulaToken {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
        #[OwnableStorageField]
        ownable: OwnableData,
    }

    impl FulaToken {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut FulaToken| {
                instance._init_with_owner(instance.env().caller());
                instance.initialize(total_supply).unwrap()
            })
        }

        #[ink(message)]
        #[brush::modifiers(only_owner)]
        pub fn initialize(&mut self, total_supply: Balance) -> Result<(), OwnableError> {
            let metadata = PSP22MetadataData {
                name: Some("Fula Token".to_string()),
                symbol: Some("FULA".to_string()),
                decimals: 18, // 18 decimals for Ethereum compatibility
                _reserved: None,
            };
            self.metadata = metadata;
            self._mint(self.owner(), total_supply).expect("Should mint");
            Ok(())
        }
    }

    impl Ownable for FulaToken {}
    impl PSP22 for FulaToken {}
    impl PSP22Metadata for FulaToken {}
    impl PSP22Mintable for FulaToken {}
    impl PSP22Internal for FulaToken {
        fn _emit_transfer_event(
            &self,
            from: Option<AccountId>,
            to: Option<AccountId>,
            amount: Balance,
        ) {
            self.env().emit_event(Transfer { from, to, amount })
        }
    }
}
