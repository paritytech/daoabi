use ethabi::{Function, Token, Uint};
use Error;

pub struct Vote {
	function: Function,
}

impl Vote {
	pub fn new(function: Function) -> Self {
		Vote {
			function: function,
		}
	}

	pub fn encode(&self, proposal_id: Uint, supports_proposal: bool) -> Vec<u8> {
		self.function.encode_call(vec![Token::Uint(proposal_id), Token::Bool(supports_proposal)]).expect("")
	}

	pub fn decode(&self, data: Vec<u8>) -> Result<Uint, Error> {
		let tokens = try!(self.function.decode_output(data));
		let result = tokens.into_iter()
			.next()
			.and_then(Token::to_uint)
			.unwrap();
		Ok(result)
	}
}
