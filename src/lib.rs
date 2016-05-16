extern crate ethabi;

mod dao;
mod error;
mod proposal;

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
