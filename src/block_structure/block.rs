use super::{
    block_header::BlockHeader, 
    transaction::Transaction,
    error_block::ErrorBlock,
};

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>
}

impl Block {

    pub fn new(header: BlockHeader) -> Self {
        Block { 
            header, 
            transactions: vec![] 
        }
    }

    pub fn proof_of_inclusion(&self) -> bool {
        //self.block_header.proof_of_inclusion(&self.transactions)
        todo!()
    }

    pub fn agregar_transaccion(self, transaction: Transaction) {
        todo!()
    }
}