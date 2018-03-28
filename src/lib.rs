mod primitives;

use primitives::Address;






type Nonce = u64;
type Wei = u64;



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













#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
