import type BN from 'bn.js';
import type {ReturnNumber} from '@727-ventures/typechain-types';

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
	againstVotes: number,
	forVotes: number
}

export type Proposal = {
	to: AccountId,
	amount: ReturnNumber,
	voteStart: number,
	voteEnd: number,
	executed: boolean
}

