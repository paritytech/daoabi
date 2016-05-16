use ethabi::{Address, Uint};

/// A proposal.
/// 
/// With `newCurator == false` represents a transaction to be issued by this DAO
/// A proposal with `newCurator == true` represents a DAO split.
pub struct Proposal {
	/// The address where the `amount` will go to if the proposal is accepted
	/// or if `newCurator` is true, the proposed Curator of
	/// the new DAO).
	pub recipient: Address,
	/// The amount to transfer to `recipient` if the proposal is accepted.
	pub amount: Uint,
	/// A plain text description of the proposal/
	pub description: String,
	/// A unix timestamp, denoting the end of the voting period
	pub voting_deadline: Uint,
	/// True if the proposal's votes have yet to be counted, otherwise False
	pub open: bool,
	/// True if quorum has been reached, the votes have been counted, and
	/// the majority said yes.
	pub proposal_passed: bool,
	/// A hash to check validity of a proposal
	pub proposal_hash: [u8; 32],
	/// Deposit in wei the creator added when submitting their proposal. It
	/// is taken from the msg.value of a newProposal call.
	pub proposal_deposit: Uint,
	/// True if this proposal is to assign a new Curator.
	pub new_curator: bool,
	/// Number of Tokens in favor of the proposal
	pub yea: Uint,
	/// Number of Tokens opposed to the proposal
	pub nay: Uint,
	/// Address of the shareholder who created the proposal
	pub creator: Address,
}
