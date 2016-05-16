extern crate ethabi;

mod dao;
mod error;
pub mod function;
mod proposal;
mod util;

pub use dao::DAO;
pub use error::Error;
pub use proposal::Proposal;

#[cfg(test)]
mod tests {
	use ethabi::{Interface, Contract};

    #[test]
    fn it_works() {
		let json_abi = include_bytes!("./dao.json");
		let face = Interface::load(json_abi).unwrap();
		let _ = Contract::new(face);
    }
}
