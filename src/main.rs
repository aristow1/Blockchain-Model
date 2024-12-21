
mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("First block data".to_string());
    blockchain.add_block("Second block data".to_string());

    for block in blockchain.chain.iter() {
        println!("{:?}", block);
    }

    println!("Is blockchain valid? {}", blockchain.is_valid());
}
