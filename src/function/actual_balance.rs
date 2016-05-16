use ethabi::{Function, Token, Uint};
use Error;

pub struct ActualBalance {
	function: Function,
}

impl ActualBalance {
	pub fn new(function: Function) -> Self {
		ActualBalance {
			function: function,
		}
	}

	pub fn encode(&self) -> Vec<u8> {
		self.function.encode_call(vec![]).expect("")
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
