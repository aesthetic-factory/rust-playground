
extern crate  d1_filework;
use d1_filework::*;
use failure::Error;

fn main()-> Result<(), Error> {
    println!("Hello, world!");
    let trans =
    get_transactions_c("test_data/transaction.json").expect("Could not load transaction");
    for t in trans {
        println!("{:?}", t);
    }
    let t = get_first_transaction_for("test_data/transaction.json", "May");
    match t{
        Ok(v)=>  println!("Found transcation {:?}", v),
        Err(e)=> println!("Error {}, Backtrace: {}", e, e.backtrace()),
    }
   
    Ok(())
}

