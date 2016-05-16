use ethabi::{Function, Token, Uint};
use {Proposal, Error};
use util::to_bytes32;

pub struct Proposals {
	function: Function,
}

impl Proposals {
	pub fn new(function: Function) -> Self {
		Proposals {
			function: function,
		}
	}

	pub fn encode(&self, id: Uint) -> Vec<u8> {
		self.function.encode_call(vec![Token::Uint(id)]).expect("")
	}

	pub fn decode(&self, data: Vec<u8>) -> Result<Proposal, Error> {
		let tokens = try!(self.function.decode_output(data));
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
