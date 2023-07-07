# Coding Assignment 1

## Investment DAO

### Instructions

- Use this template and continue work in your own repository
- Implement the two smart contracts:
  - **PSP22 + PSP22Metadata**
  - **DAO**
- In the dao directory run `cargo test` and the two tests should pass

### What should be used

- Swanky & Swanky node
- ink! v4.2.1
- PSP22 from Openbrush

### Description

In this assignment you will create an investment DAO.

Users will be able to submit a funding proposals and asks governance token holders to vote on it. A funding proposal will have a voting period, a fund amount as well as fund recipient.

At the end of the voting period and if the vote reach the defined quorum, the fund recipient defined in the proposal will get the fund transferred.

### Types

`Proposal` 
* defines the fund recipient `to`
* the start & end of the voting period `vote_start` `vote_end`, 
* a boolean to know if it has been already `executed` or not
* requested  `amount`.

`ProposalVote` defines the proportion (in %) of `for_votes` and `against_vote`

`VoteType` an enum with two fields to cast a vote: Against and For

`ProposalId` identifiers for Proposals

### Callable functions

**new** (constructor)

```rust
#[ink(constructor, payable)]
pub fn new(governance_token: AccountId, quorum: u8) -> Self { ...
```

The constructor is `payable` in order to fund the contract with a certain amount of native token that will be transferred to the recipients of funding proposals (of course proposals amount should not exceed balance of the contract).

`governance_token` the PSP22 token `accountId` of the governance token

`quorum` Minimum number of cast voted required for a proposal to be successful. The quorum is usually somewhere in the range of 1-10% of total token supply.

**propose**

```rust
pub fn propose(&mut self, to: AccountId, amount: Balance, duration: u64) -> Result<(), DaoError> { ...
```

This function is used to submit a new proposal. Any user can call this function.

`to` the recipient `AccountId`

`amount` the amount of funds (in Native tokens) requested

`duration` the duration (in minutes) of the open voting period for the proposal. Note at it should start directly after this function call.

In the body of the function:

- Ensure the `amount` is not 0 (or return `DaoError::AmountShouldNotBeZero`)
- Ensure the `duration` is not 0 (or return `DaoError::DurationError`)
- The vote star value should be the actual block timestamp
- The Proposal should be added to the `proposals` Mapping (please increase `Id` by one before insert)

**vote**

```rust
pub fn vote(&mut self, proposal_id: ProposalId, vote: VoteType) -> Result<(), DaoError> { ...
```

This function is called to cast the vote of the caller. Any user can call this function.

`proposal_id` the id of the proposal to vote on

`vote` is either `Against` or `For`

In the body of the function:

- Ensure the proposal exist (or return `DaoError::ProposalNotFound`)     
- Ensure the proposal has not been already executed  (or return `DaoError::ProposalAlreadyExecuted`)     
- Ensure the voting period is still open (or return `DaoError::VotePeriodEnded`)      
- Ensure the caller has not already voted (or return `DaoError::AlreadyVoted`)      
- Add the caller is the `votes` Mapping     
- Check the `weight` of the caller of the governance token (the proportion of caller balance in relation to total supply)      
- Add the `weight` value to `against_votes` or `for_votes` based on `vote`     
- Insert proposal in `proposal_votes` Mapping     

**execute**

```rust
pub fn execute(&mut self, proposal_id: ProposalId) -> Result<(), DaoError> { ...
```

This function is called when the voting period of a proposal has ended to transfer the funds to the recipient (if the quorum has been reached). Anyone can call this function.

`proposal_id` the id of the proposal to vote on

In the body of the function:

- Ensure the proposal exist (or return `DaoError::ProposalNotFound`)     
- Ensure the proposal has not been already executed  (or return `DaoError::ProposalAlreadyExecuted`)     
- Ensure the sum of `For` & `Against` vote reach quorum (or return `DaoError::QuorumNotReached`)     
- Ensure there is more `For` votes than `Against` votes (or return `DaoError::ProposalNotAccepted`)    
- Save that proposal has been executed     
- transfer `amount` to the recipient     

### **Contract storage**

`proposals` a Mapping to identify proposals: between `ProposalId` and Proposals      
`proposal_votes` a Mapping between Proposals and `ProposalVotes`     
`votes` a Mapping to ensure an account has already voted, where the mapping key is a tupple of `(ProposalId, AccountId)` and mapping value is just `()`    
`next_proposal_id` to track next proposal id     
`quorum` Quorum required for a proposal to be successful. As percentage of total supply of governance tokens     
`governance_token`  address of governance token contract     

### Submission criteria
* Both contracts should be implemented in the folders defined by the workspace
* All contract types and functions should be implemented as described above
* Unit test should be expanded to cover all the functions of the contracts. Run the unit test with `cargo test`
* Existing 2 unit tests for dao contract can be extended but should not be deleted and should pass
* There is a Github CI test that should be passing. You can check it in the Actions tab of your repository

Due to big number of participants the submissions will NOT be considered if any of the above criteria is not met.