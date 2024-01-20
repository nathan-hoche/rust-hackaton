#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod client {
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    
    #[derive(scale::Decode, scale::Encode, Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Role {
        SuperAdmin,
        Admin,
        Modo,
        User
    }

    impl Default for Role {
        fn default() -> Self {
            Role::User
        }
    }

    impl Clone for Role {
        fn clone(&self) -> Self {
            match self {
                Role::SuperAdmin => Role::SuperAdmin,
                Role::Admin => Role::Admin,
                Role::Modo => Role::Modo,
                Role::User => Role::User,
            }
        }   
    }

    impl PartialEq for Role {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Role::SuperAdmin => {
                    match other {
                        Role::SuperAdmin => true,
                        _ => false
                    }
                },
                Role::Admin => {
                    match other {
                        Role::Admin => true,
                        _ => false
                    }
                },
                Role::Modo => {
                    match other {
                        Role::Modo => true,
                        _ => false
                    }
                },
                Role::User => {
                    match other {
                        Role::User => true,
                        _ => false
                    }
                },
            }
        }
    }

    #[ink(storage)]
    pub struct Client {
        /// Stores a single `bool` value on the storage.
        role: Role,
    }

    impl Client {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: Role) -> Self {
            Self {role: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn set(&mut self, role: Role) {
            if self.role == Role::SuperAdmin {
                self.role = role.clone();
            } else if self.role == Role::Admin {
                if role == Role::Modo || role == Role::User {
                    self.role = role.clone();
                }
            }
        }


        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_role(&self) -> Role {
            self.role.clone()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let cli = Client::default();
            assert_eq!(cli.get_role(), Role::User);
        }

        /// We test to set role from SuperAdmin
        /// SuperAdmin can set role to Admin, Modo, User
        /// Admin can set role to Modo, User
        
        #[ink::test]
        fn set_role_from_superadmin() {
            let mut cli1: Client = Client::new(Role::SuperAdmin);
            // let mut cli2 = Client::new(Default::default());
            cli1.set(Role::Admin);
            assert_eq!(cli1.get_role(), Role::Admin);
        }

        #[ink::test]
        fn set_role_from_admin() {
            let mut cli1: Client = Client::new(Role::Admin);
            // let mut cli2 = Client::new(Default::default());
            cli1.set(Role::Modo);
            assert_eq!(cli1.get_role(), Role::Modo);
        }

    }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = InkShowRef::default();

            // When
            let contract_account_id = client
                .instantiate("ink_show", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<InkShowRef>(contract_account_id.clone())
                .call(|ink_show| ink_show.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = InkShowRef::new(false);
            let contract_account_id = client
                .instantiate("ink_show", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<InkShowRef>(contract_account_id.clone())
                .call(|ink_show| ink_show.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<InkShowRef>(contract_account_id.clone())
                .call(|ink_show| ink_show.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<InkShowRef>(contract_account_id.clone())
                .call(|ink_show| ink_show.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
