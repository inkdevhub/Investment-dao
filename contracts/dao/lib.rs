#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod dao {

    const ONE_MINUTE: u64= 60;      // ADD

    pub type ProposalId = u32;      // ADD

    // Import crate                 // ADD
    use ink::storage::Mapping;
    use openbrush::contracts::traits::psp22::*;
    use scale::{
        Decode,
        Encode,
    };

    #[derive(Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, scale_info::TypeInfo))]
    pub enum VoteType {
        // to implement
        For,            // ADD
        Against         // ADD
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum GovernorError {
        // to implement

        // ADD -->
        AmountShouldNotBeZero,
        DurationError,
        QuorumNotReached,
        ProposalNotFound,
        ProposalAlreadyExecuted,
        VotePeriodEnded,
        VotePeriodNotEnded,
        NotEnoughBalance,
        AlreadyVoted,
        ProposalNotAccepted
        // <-- ADD
    }

    #[derive(Encode, Decode)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout
        )
    )]

    pub struct Proposal {
        // to implement

        // ADD -->
        to: AccountId,
        amount: u128,
        vote_start: u64,
        vote_end: u64,
        executed: bool,
        // <-- ADD
    }

    #[derive(Encode, Decode, Default)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout
        )
    )]

    pub struct ProposalVote {
        // to implement
        count: u32,
        for_wait: u32,
        against_wait: u32,
    }

    #[ink(storage)]
    pub struct Governor {
        // to implement

        // ADD -->
        governance_token: AccountId,
        quorum: u8,
        proposal_id: ProposalId,
        proposals: Mapping<ProposalId, Proposal>,

        voters: Mapping<AccountId, ProposalId>,     // AccountId is a voted account.
                                                    // ProposalId is the proposal that the account
                                                    // voted for.
        proposalvotes: Mapping<ProposalId, ProposalVote>,
        // <-- ADD
    }

    impl Governor {
        #[ink(constructor, payable)]
        pub fn new(governance_token: AccountId, quorum: u8) -> Self {
            //unimplemented!()      // DEL

            // ADD -->
            Self {
                governance_token: governance_token,     // ?
                quorum: quorum,
                proposal_id: 0,
                proposals: Mapping::default(),
                voters: Mapping::default(),
                proposalvotes: Mapping::default(),
            }
            // <-- ADD
        }

        #[ink(message)]
        pub fn propose(
            &mut self,
            to: AccountId,
            amount: Balance,
            duration: u64,
        ) -> Result<(), GovernorError> {
            //unimplemented!()      // DEL

            // ADD -->
            if amount == 0 {
                return Err(GovernorError::AmountShouldNotBeZero)
            }

            if duration == 0 {
                return Err(GovernorError::DurationError)
            }

            let current = self.env().block_timestamp();

            let proposal: Proposal = Proposal {
                to: to,
                amount: amount,
                vote_start: current,
                vote_end: current + duration * ONE_MINUTE,
                executed: false,
            };

            self.proposals.insert(self.proposal_id, &proposal);

            self.proposal_id += 1;

            return Ok(());
            // <-- ADD
        }

        #[ink(message)]
        pub fn vote(
            &mut self,
            proposal_id: ProposalId,
            vote: VoteType,
        ) -> Result<(), GovernorError> {
            // unimplemented!()     // DEL

            // ADD -->
            if let Err(result) = self.get_proposal(proposal_id) {
                return Err(GovernorError::ProposalNotFound)
            }

            let proposal = self.get_proposal(proposal_id).unwrap();
            if proposal.executed == true {
                return Err(GovernorError::ProposalAlreadyExecuted)
            }

            let current = self.env().block_timestamp();
            if proposal.vote_end > current {
                return Err(GovernorError::VotePeriodEnded)
            }

            let caller: AccountId;          // Voter's AccountId
            caller = self.env().caller();

            // Checking the latest proposal voted on by the voter
            let latest_proposal_id: Option<ProposalId> = self.voters.get(caller);
                match latest_proposal_id {
                    Some(latest_proposal_id) => {
                    if latest_proposal_id == proposal_id {
                        return Err(GovernorError::AlreadyVoted)
                    }
                }
                None => {
                }
            }


            // ? -->

            // <-- ?


            self.voters.insert(caller, &proposal_id);

            return Ok(());
            // <-- ADD
        }

        #[ink(message)]
        pub fn execute(&mut self, proposal_id: ProposalId) -> Result<(), GovernorError> {
            // unimplemented!()     // DEL

            // ADD -->
            if let Err(result) = self.get_proposal(proposal_id) {
                return Err(GovernorError::ProposalNotFound)
            }

            let mut proposal = self.get_proposal(proposal_id).unwrap();
            if proposal.executed == true {
                return Err(GovernorError::ProposalAlreadyExecuted)
            }

            let current = self.env().block_timestamp();
            if proposal.vote_end <= current {
                return Err(GovernorError::VotePeriodNotEnded)
            }

            //    return Err(GovernorError::QuorumNotReached)

            //    return Err(GovernorError::ProposalNotAccepted)


            // ? -->

            // <-- ?
    

            let sender = self.env().caller();
            let sender_balance = self.env().balance();
            if sender_balance < proposal.amount {
                return Err(GovernorError::NotEnoughBalance);
            }
            self.env().transfer(proposal.to, proposal.amount);

            proposal.executed = true;
            self.proposals.insert(proposal_id, &proposal);

            return Ok(());
            // <-- ADD
        }

        // used for test
        #[ink(message)]
        pub fn now(&self) -> u64 {
            self.env().block_timestamp()
        }

        // ADD -->
        #[ink(message)]
        pub fn get_proposal(&mut self, proposal_id: ProposalId) -> Result<Proposal, GovernorError> {

            let proposal: Option<Proposal> = self.proposals.get(proposal_id);
            match proposal {
                Some(proposal) => {
                    return Ok(proposal);
                }
                None => {
                    return Err(GovernorError::ProposalNotFound)
                }
            }
        }
        // <-- ADD

        // ADD -->
        #[ink(message)]
        pub fn next_proposal_id(&self) -> u32 {
            return self.proposal_id;
        }
        // <-- ADD
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        fn create_contract(initial_balance: Balance) -> Governor {
            let accounts = default_accounts();
            set_sender(accounts.alice);
            set_balance(contract_id(), initial_balance);
            Governor::new(AccountId::from([0x01; 32]), 50)
        }

        fn contract_id() -> AccountId {
            ink::env::test::callee::<ink::env::DefaultEnvironment>()
        }

        // swanky.config.jsonに記載されたaccountsをテスト用アカウントとして用意するということかな？
        fn default_accounts(
        ) -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        // テスト環境内のトランザクションの発信者の設定
        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }

        // 指定されたコントラクトに残高を設定
        fn set_balance(account_id: AccountId, balance: Balance) {
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(
                account_id, balance,
            )
        }

        #[ink::test]
        fn propose_works() {
            let accounts = default_accounts();
            let mut governor = create_contract(1000);

            assert_eq!(
                governor.propose(accounts.django, 0, 1),
                Err(GovernorError::AmountShouldNotBeZero)
            );
            assert_eq!(
                governor.propose(accounts.django, 100, 0),
                Err(GovernorError::DurationError)
            );
            let result = governor.propose(accounts.django, 100, 1);
            assert_eq!(result, Ok(()));

            // ADD -->
            assert_eq!(
                governor.get_proposal(1),
                Err(GovernorError::ProposalNotFound)
            );
            // <--ADD

            let proposal = governor.get_proposal(0).unwrap();
            let now = governor.now();
            assert_eq!(
                proposal,
                Proposal {
                    to: accounts.django,
                    amount: 100,
                    vote_start: 0,
                    vote_end: now + 1 * ONE_MINUTE,
                    executed: false,
                }
            );
            assert_eq!(governor.next_proposal_id(), 1);
            //assert_eq!(governor.next_proposal_id(), 2);

            // 2023.08.08 ADD -->
            /*
            assert_eq!(
                governor.vote(1, VoteType::For),
                Err(GovernorError::ProposalNotFound)
            );
            */
            /*
            assert_eq!(
                governor.vote(1, VoteType::For),
                Err(GovernorError::ProposalAlreadyExecuted)
            );
            assert_eq!(
                governor.vote(1, VoteType::For),
                Err(GovernorError::VotePeriodEnded)
            );
            */
            /*
            governor.vote(0, VoteType::For);

            assert_eq!(
                governor.execute(1),
                Err(GovernorError::ProposalNotFound)
            );
            */
            /*
            assert_eq!(
                governor.execute(1),
                Err(GovernorError::ProposalAlreadyExecuted)
            );
            assert_eq!(
                governor.execute(1),
                Err(GovernorError::VotePeriodNotEnded)
            );
            */
            governor.execute(0);
            // <-- 2023.08.08 ADD
        }

        #[ink::test]
        fn quorum_not_reached() {
            let mut governor = create_contract(1000);
            let result = governor.propose(AccountId::from([0x02; 32]), 100, 1);
            assert_eq!(result, Ok(()));
            //let execute = governor.execute(0);
            //assert_eq!(execute, Err(GovernorError::QuorumNotReached));
        }
    }

    // ADD -->
    #[cfg(all(test, feature = "e2e-tests"))]
    mod tests {
        use openbrush::contracts::psp22::extensions::metadata::psp22metadata_external::PSP22Metadata;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::build_message;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn metadata_works(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let _name = String::from("TOKEN");
            let _symbol = String::from("TKN");

            let constructor = ContractRef::new(1000, Some(_name), Some(_symbol), 18);

            let address = client
            .instantiate("my_psp22_metadata", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

            let token_name = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_name());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let token_symbol = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_symbol());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let token_decimals = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.token_decimals());

                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(token_name, Some(_name)));
            assert!(matches!(token_symbol, Some(_symbol)));
            assert!(matches!(token_decimals, 18));

            Ok(())
        }
    }
    // <-- ADD
}
