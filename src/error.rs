use ethabi::Error as ABIError;

#[derive(Debug)]
pub enum Error {
	ABI(ABIError),
}

impl From<ABIError> for Error {
	fn from(e: ABIError) -> Self {
		Error::ABI(e)
	}
}
