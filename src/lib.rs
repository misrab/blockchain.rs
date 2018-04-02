mod primitives;


use primitives::{Address, Hash};


use std::collections::HashMap;




type Nonce = u64;
type Wei = u64;


// the global state of accounts at 
// any point in time
struct worldState {
  accounts: HashMap<Address, Account>,
}



struct Account {
  nonce: Nonce,
  balance: Wei,
  //storageRoot: ,
  //codeHash: ,
}


struct Transaction {
  nonce: Nonce,
  gasPrice: Wei,
  gasLimit: Wei,
  to: Address,
  value: Address,
  //v, r, s: ,
  //init || data: Option?

}


struct BlockHeader {
  //parentHash: ,
  //ommersHash: ,
  //beneficiary: ,
  //stateRoot: ,
  //transactionsRoot: ,
  //receiptsRoots: ,
  //logsBloom: ,
  //difficulty: ,
  //number: ,
  //gasLimit: ,
  //gasUsed: ,
  //timestamp: ,
  //extraData: ,
  //mixHash: ,
  //nonce: ,
}
// also B_U and B_T (block ommers, block transactions)



// transaction generator
// creates random transactions
// for testing purposes
// this generator is able to create transactions on anyone's behalf
struct TransactionGenerator {
  interval: u32,
}

impl TransactionGenerator {
  //fn new() -> TransactionGenerator {
  //  TransactionGenerator{
  //    interval: 200,
  //  }
  //}

  fn moo() {
    println!("moo!");
  }

  /*
  fn generate_transaction() -> Transaction {
      // select two random accounts
      let sender =
      let receiver =

      // get the nonce from the sender's account
      // in world state
      let nonce = worldState[sender].nonce + 1;

      // random gasValue, gasLimit, value within reason
      // todo

      let t = Transaction {
    };


    t
  }*/


}









#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }



    #[test]
    fn simulate() {
      let tg = TransactionGenerator{
        interval: 200,
      };



    }



}
