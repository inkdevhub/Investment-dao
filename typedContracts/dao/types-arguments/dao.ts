import type BN from 'bn.js';

export type AccountId = string | number[]

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export enum GovernorError {
	amountShouldNotBeZero = 'AmountShouldNotBeZero',
	durationError = 'DurationError',
	proposalNotFound = 'ProposalNotFound',
	proposalAlreadyExecuted = 'ProposalAlreadyExecuted',
	votePeriodEnded = 'VotePeriodEnded',
	alreadyVoted = 'AlreadyVoted',
	votePeriodNotEnded = 'VotePeriodNotEnded',
	quorumNotReached = 'QuorumNotReached',
	transferError = 'TransferError',
	proposalNotAccepted = 'ProposalNotAccepted'
}

export enum VoteType {
	against = 'Against',
	for = 'For'
}

export type ProposalVote = {
	againstVotes: (number | string | BN),
	forVotes: (number | string | BN)
}

export type Proposal = {
	to: AccountId,
	amount: (string | number | BN),
	voteStart: (number | string | BN),
	voteEnd: (number | string | BN),
	executed: boolean
}

