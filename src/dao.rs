use std::ptr;
use ethabi::{Contract, Interface, Uint, Token};
use super::{Error, Proposal};

fn to_bytes32(data: Vec<u8>) -> [u8; 32] {
	if data.len() != 32 {
		panic!("expected bytes32");
	}
	let mut result = [0u8; 32];
	unsafe {
		ptr::copy(data.as_ptr(), result.as_mut_ptr(), 32);
	}
	result
}

pub struct DAO {
	contract: Contract,
}

impl DAO {
	pub fn new() -> Self {
		DAO {
			contract: Contract::new(Interface::load(include_bytes!("./dao.json")).expect("Invalid abi.")),
		}
	}

	pub fn proposal(&self, id: Uint) -> Vec<u8> {
		let func = self.contract.function("proposals".to_owned()).expect("");
		let params = vec![Token::Uint(id)];
		func.encode_call(params).expect("expected param is Uint: params = vec![Token::Uint]: qed")
	}

	pub fn proposal_output(&self, data: Vec<u8>) -> Result<Proposal, Error> {
		let func = self.contract.function("proposals".to_owned()).expect("");
		let tokens = try!(func.decode_output(data));
		let mut i = tokens.into_iter();

		let proposal = Proposal {
			recipient: i.next().and_then(Token::to_address).unwrap(),
			amount: i.next().and_then(Token::to_uint).unwrap(),
			description: i.next().and_then(Token::to_string).unwrap(),
			voting_deadline: i.next().and_then(Token::to_uint).unwrap(),
			open: i.next().and_then(Token::to_bool).unwrap(),
			proposal_passed: i.next().and_then(Token::to_bool).unwrap(),
			proposal_hash: i.next().and_then(Token::to_fixed_bytes).map(to_bytes32).unwrap(),
			proposal_deposit: i.next().and_then(Token::to_uint).unwrap(),
			new_curator: i.next().and_then(Token::to_bool).unwrap(),
			yea: i.next().and_then(Token::to_uint).unwrap(),
			nay: i.next().and_then(Token::to_uint).unwrap(),
			creator: i.next().and_then(Token::to_address).unwrap(),
		};

		Ok(proposal)
	}
}

#[cfg(test)]
mod test {
	extern crate rustc_serialize;
	use self::rustc_serialize::hex::FromHex;
	use super::DAO;

	#[test]
	fn test_proposal() {
		let expected = "013cf08b1111111111111111111111111111111111111111111111111111111111111111".from_hex().unwrap();
		let dao = DAO::new();
		let proposal_call = dao.proposal([0x11u8; 32]);
		assert_eq!(proposal_call, expected);
	}
}
