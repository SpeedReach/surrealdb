use crate::ctx::Context;
use crate::dbs::{Options, Transaction};
use crate::doc::CursorDoc;
use crate::err::Error;
use crate::sql::comment::shouldbespace;
use crate::sql::error::IResult;
use crate::sql::strand::{strand, Strand};
use crate::sql::value::Value;
use derive::Store;
use nom::bytes::complete::tag_no_case;
use revision::revisioned;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Store, Hash)]
#[revisioned(revision = 1)]
pub struct ThrowStatement {
	pub error: Strand,
}

impl ThrowStatement {
	/// Check if we require a writeable transaction
	pub(crate) fn writeable(&self) -> bool {
		false
	}
	/// Process this type returning a computed simple Value
	pub(crate) async fn compute(
		&self,
		_ctx: &Context<'_>,
		_opt: &Options,
		_txn: &Transaction,
		_doc: Option<&CursorDoc<'_>>,
	) -> Result<Value, Error> {
		Err(Error::Thrown(self.error.as_str().to_owned()))
	}
}

impl fmt::Display for ThrowStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "THROW {}", self.error)
	}
}

pub fn throw(i: &str) -> IResult<&str, ThrowStatement> {
	let (i, _) = tag_no_case("THROW")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, e) = strand(i)?;
	Ok((
		i,
		ThrowStatement {
			error: e,
		},
	))
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn throw_basic() {
		let sql = "THROW 'Record does not exist'";
		let res = throw(sql);
		let out = res.unwrap().1;
		assert_eq!("THROW 'Record does not exist'", format!("{}", out))
	}
}
