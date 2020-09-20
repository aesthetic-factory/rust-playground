mod error;
pub use error::TransationError;

use serde_derive::*;

#[derive(Deserialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, String> {
    let s = match std::fs::read_to_string(fname) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    Ok(t)
}

fn get_transactions_b(fname: &str) -> Result<Vec<Transaction>, String> {
    std::fs::read_to_string(fname)
        .map_err(|e| e.to_string())
        .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.to_string()))
}

pub fn get_transactions_c(fname: &str) -> Result<Vec<Transaction>, TransationError> {
    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
}

pub fn get_first_transaction_for(fname:&str, uname:&str)->Result<Transaction, failure::Error>{
    let trans = get_transactions_c(fname)?;
    for t in trans {
        if t.from == uname {
            return Ok(t);
        }
    }
    Err(TransationError::Mess("Could not find name").into())
}