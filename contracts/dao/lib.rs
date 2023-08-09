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
        ProposalNotAccepted,
        ProposalVoteNotFound,
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
        vote_start: u64,
        vote_end: u64,
        executed: bool,
        amount: u128,
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
        count: u8,              // number of votes
        for_weight: u128,       // weight total for for
        against_weight: u128,   // weight total for against
    }

    #[ink(storage)]
    pub struct Governor {
        // to implement

        // ADD -->
        governance_token: AccountId,
        quorum: u8,
        proposal_id: ProposalId,

        // Mapping
        proposals: Mapping<ProposalId, Proposal>,
        proposal_votes: Mapping<ProposalId, ProposalVote>,
        // <-- ADD
    }

    impl Governor {
        #[ink(constructor, payable)]
        pub fn new(governance_token: AccountId, quorum: u8) -> Self {
            //unimplemented!()      // DEL

            // ADD -->
            Self {
                governance_token: governance_token,
                quorum: quorum,
                proposal_id: 0,
                proposals: Mapping::default(),
                proposal_votes: Mapping::default(),
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

            // <--

            // I have not been able to implement a check that the amount does not exceed 
            // the total supply of governance tokens.

            // -->

            if duration == 0 {
                return Err(GovernorError::DurationError)
            }

            let current = self.env().block_timestamp();
            let proposal: Proposal = Proposal {
                to: to,
                vote_start: current,
                vote_end: current + duration * ONE_MINUTE,
                executed: false,
                amount: amount,
            };
            self.proposals.insert(self.proposal_id, &proposal);

            let proposal_vote: ProposalVote = ProposalVote {
                count: 0,
                for_weight: 0,
                against_weight: 0,
            };
            self.proposal_votes.insert(self.proposal_id, &proposal_vote);

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
            let total_supply: u128;
            let amount: u128;
            let weight: u128;

            if let Err(result) = self.get_proposal(proposal_id) {
                return Err(GovernorError::ProposalNotFound)
            }

            let proposal = self.get_proposal(proposal_id).unwrap();
            if proposal.executed == true {
                return Err(GovernorError::ProposalAlreadyExecuted)
            }

            let current = self.env().block_timestamp();
            if proposal.vote_end <= current {
                return Err(GovernorError::VotePeriodEnded)
            }

            // -->

            // I haven't been able to implement the voted check.
            // (GovernorError::AlreadyVoted)

            // <--

            let caller: AccountId;      // Voter's AccountId
            caller = self.env().caller();

            // -->

            // I haven't been able to implement governonce tokens.
            // Therefore, it was not possible to obtain the amount of
            // governance tokens owned by the caller.

            // <--

            // DEBUG -->
            total_supply = 1000;
            amount = 600;
            weight = amount * 100 / total_supply;
            // <-- DEBUG

            if let Err(result) = self.get_proposal_vote(proposal_id) {
                return Err(GovernorError::ProposalVoteNotFound)
            }
            let mut proposal_vote = self.get_proposal_vote(proposal_id).unwrap();
            proposal_vote.count += 1;

            if vote == VoteType::For {
                proposal_vote.for_weight += weight;
            }
            else {
                proposal_vote.against_weight += weight;
            }

            self.proposal_votes.insert(proposal_id, &proposal_vote);

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

            if let Err(result) = self.get_proposal_vote(proposal_id) {
                return Err(GovernorError::ProposalVoteNotFound)
            }
            let proposal_vote = self.get_proposal_vote(proposal_id).unwrap();
            if self.quorum > proposal_vote.count {
                return Err(GovernorError::QuorumNotReached)
            }

            if proposal_vote.for_weight <= proposal_vote.against_weight {
                return Err(GovernorError::ProposalNotAccepted)
            }

            // -->

            // I added this error handling because I couldn't implement 
            // automatic execution of execute().
            let current = self.env().block_timestamp();
            if proposal.vote_end <= current {
                return Err(GovernorError::VotePeriodNotEnded)
            }

            // <--

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
        pub fn get_proposal_vote(&mut self, proposal_id: ProposalId) -> Result<ProposalVote, GovernorError> {

            let proposal_vote: Option<ProposalVote> = self.proposal_votes.get(proposal_id);
            match proposal_vote {
                Some(proposal_vote) => {
                    return Ok(proposal_vote);
                }
                None => {
                    return Err(GovernorError::ProposalVoteNotFound)
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

        fn default_accounts(
        ) -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }

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
        }

        // ADD -->
        #[ink::test]
        fn vote_works() {
            let accounts = default_accounts();
            let mut governor = create_contract(1000);
            let result = governor.propose(accounts.django, 100, 1);

            assert_eq!(
                governor.vote(1, VoteType::For),
                Err(GovernorError::ProposalNotFound)
            );
           
            let mut proposal = governor.get_proposal(0).unwrap();
            proposal.executed = true;
            governor.proposals.insert(0, &proposal);
            assert_eq!(
                governor.vote(0, VoteType::For),
                Err(GovernorError::ProposalAlreadyExecuted)
            );

            proposal.executed = false;
            proposal.vote_end = 0;
            governor.proposals.insert(0, &proposal);
            assert_eq!(
                governor.vote(0, VoteType::For),
                Err(GovernorError::VotePeriodEnded)
            );

            proposal.vote_end = 1;
            governor.proposals.insert(0, &proposal);
            governor.vote(0, VoteType::For);
            let proposal_vote = governor.get_proposal_vote(0).unwrap();
            assert_eq!(
                proposal_vote,
                ProposalVote {
                    count: 1,
                    for_weight: 60,
                    against_weight: 0,
                }
            );

            assert_eq!(
                governor.get_proposal_vote(1),
                Err(GovernorError::ProposalVoteNotFound)
            );
        }
        // <-- ADD

        // ADD -->
        #[ink::test]
        fn execute_works() {
            let accounts = default_accounts();
            let mut governor = create_contract(1000);
            let result = governor.propose(accounts.django, 100, 1);

            assert_eq!(
                governor.execute(1),
                Err(GovernorError::ProposalNotFound)
            );
           
            let mut proposal = governor.get_proposal(0).unwrap();
            proposal.executed = true;
            governor.proposals.insert(0, &proposal);
            assert_eq!(
                governor.execute(0),
                Err(GovernorError::ProposalAlreadyExecuted)
            );

            proposal.executed = false;
            proposal.vote_end = 1;
            governor.proposals.insert(0, &proposal);
            assert_eq!(
                governor.execute(0),
                Err(GovernorError::QuorumNotReached)
            );

            proposal.vote_end = 1;
            governor.proposals.insert(0, &proposal);
            governor.vote(0, VoteType::For);
            let mut proposal_vote = governor.get_proposal_vote(0).unwrap();
            proposal_vote.count = 50;

            proposal_vote.for_weight = 0;
            proposal_vote.against_weight = 50;
            governor.proposal_votes.insert(0, &proposal_vote);
            assert_eq!(
                governor.execute(0),
                Err(GovernorError::ProposalNotAccepted)
            );

            proposal_vote.for_weight = 50;
            proposal_vote.against_weight = 0;
            governor.proposal_votes.insert(0, &proposal_vote);
            governor.proposal_votes.insert(0, &proposal_vote);
            let result = governor.execute(0);
            assert_eq!(result, Ok(()));
        }
        // <-- ADD

        #[ink::test]
        fn quorum_not_reached() {
            let mut governor = create_contract(1000);
            let result = governor.propose(AccountId::from([0x02; 32]), 100, 1);
            assert_eq!(result, Ok(()));
            let execute = governor.execute(0);
            assert_eq!(execute, Err(GovernorError::QuorumNotReached));
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
