use ethabi::{Contract, Interface, Function};
use function::{Proposals, NumberOfProposals, ActualBalance, Vote};

/// DAO contract.
pub struct DAO {
	contract: Contract,
}

impl DAO {
	/// Creates new abi instance.
	pub fn new() -> Self {
		DAO {
			contract: Contract::new(Interface::load(include_bytes!("./dao.json")).expect("Invalid abi.")),
		}
	}

	/// Private function used to encode params expected to be found in abi.json with params known to be valid.
	fn function(&self, name: &str) -> Function {
		self.contract.function(name.to_owned()).expect("")
	}

	/// Proposals to spend the DAO's ether or to choose a new Curator.
	pub fn proposals(&self) -> Proposals {
		Proposals::new(self.function("proposals"))
	}

	/// Returns total number of proposals ever created.
	pub fn number_of_proposals(&self) -> NumberOfProposals {
		NumberOfProposals::new(self.function("numberOfProposals"))
	}

	/// Actual balance available (not locked in proposals).
	pub fn actual_balance(&self) -> ActualBalance {
		ActualBalance::new(self.function("actualBalance"))
	}

	/// Vote on proposal. Returns the vote ID.
	pub fn vote(&self) -> Vote {
		Vote::new(self.function("vote"))
	}
}

#[cfg(test)]
mod test {
	extern crate rustc_serialize;
	use self::rustc_serialize::hex::FromHex;
	use ethabi::Uint;
	use ethabi::token::TokenFromHex;
	use super::DAO;

	#[test]
	fn test_proposal() {
		let id: Uint = "0000000000000000000000000000000000000000000000000000000000000002".token_from_hex().unwrap();
		let expected = "013cf08b0000000000000000000000000000000000000000000000000000000000000002".from_hex().unwrap();
		let dao = DAO::new();
		let call = dao.proposals().encode(id);
		assert_eq!(call, expected);
	}

	#[test]
	fn test_number_of_proposals() {
		let expected = "8d7af473".from_hex().unwrap();
		let dao = DAO::new();
		let call = dao.number_of_proposals().encode();
		assert_eq!(call, expected);
	}

	#[test]
	fn test_number_of_proposals_output() {
		let expected: Uint = "0000000000000000000000000000000000000000000000000000000000000021".token_from_hex().unwrap();
		let dao = DAO::new();
		let decoded = dao.number_of_proposals().decode("0000000000000000000000000000000000000000000000000000000000000021".from_hex().unwrap()).unwrap();
		assert_eq!(decoded, expected);
	}

	#[test]
	fn test_actual_balance() {
		let expected = "39d1f908".from_hex().unwrap();
		let dao = DAO::new();
		let call = dao.actual_balance().encode();
		assert_eq!(call, expected);
	}

	#[test]
	fn test_actual_balance_output() {
		let expected: Uint = "0000000000000000000000000000000000000000000000000000000000000021".token_from_hex().unwrap();
		let dao = DAO::new();
		let decoded = dao.actual_balance().decode("0000000000000000000000000000000000000000000000000000000000000021".from_hex().unwrap()).unwrap();
		assert_eq!(decoded, expected);
	}

	#[test]
	fn test_vote() {
		let id: Uint = "0000000000000000000000000000000000000000000000000000000000000002".token_from_hex().unwrap();
		let vote = true;
		let expected = "c9d27afe00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000001".from_hex().unwrap();
		let dao = DAO::new();
		let call = dao.vote().encode(id, vote);
		assert_eq!(call, expected);
	}

	#[test]
	fn test_vote_output() {
		let expected: Uint = "0000000000000000000000000000000000000000000000000000000000000021".token_from_hex().unwrap();
		let dao = DAO::new();
		let decoded = dao.vote().decode("0000000000000000000000000000000000000000000000000000000000000021".from_hex().unwrap()).unwrap();
		assert_eq!(decoded, expected);
	}
}
