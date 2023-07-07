/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/dao';
import type BN from 'bn.js';
import type { ApiPromise } from '@polkadot/api';



export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;

	constructor(
		nativeContract : ContractPromise,
		apiPromise: ApiPromise,
	) {
		this.__nativeContract = nativeContract;
		this.__apiPromise = apiPromise;
	}
	/**
	 * propose
	 *
	 * @param { ArgumentTypes.AccountId } to,
	 * @param { (string | number | BN) } amount,
	 * @param { (number | string | BN) } duration,
	*/
	"propose" (
		to: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		duration: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "propose", [to, amount, duration], __options);
	}

	/**
	 * vote
	 *
	 * @param { (number | string | BN) } proposalId,
	 * @param { ArgumentTypes.VoteType } vote,
	*/
	"vote" (
		proposalId: (number | string | BN),
		vote: ArgumentTypes.VoteType,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "vote", [proposalId, vote], __options);
	}

	/**
	 * execute
	 *
	 * @param { (number | string | BN) } proposalId,
	*/
	"execute" (
		proposalId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "execute", [proposalId], __options);
	}

	/**
	 * getProposalVote
	 *
	 * @param { (number | string | BN) } proposalId,
	*/
	"getProposalVote" (
		proposalId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getProposalVote", [proposalId], __options);
	}

	/**
	 * getProposal
	 *
	 * @param { (number | string | BN) } proposalId,
	*/
	"getProposal" (
		proposalId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getProposal", [proposalId], __options);
	}

	/**
	 * hasVoted
	 *
	 * @param { (number | string | BN) } proposalId,
	 * @param { ArgumentTypes.AccountId } accountId,
	*/
	"hasVoted" (
		proposalId: (number | string | BN),
		accountId: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "hasVoted", [proposalId, accountId], __options);
	}

	/**
	 * now
	 *
	*/
	"now" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "now", [], __options);
	}

}