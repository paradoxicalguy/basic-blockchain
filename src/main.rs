mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut my_chain = Blockchain::new();

    my_chain.add_block("First transaction".to_string());
    my_chain.add_block("Second transaction".to_string());
    my_chain.add_block("Third transaction".to_string());

    println!("\n✅ Blockchain valid? {}", my_chain.is_chain_valid());

    println!("\n🧱 Full chain:\n");
    my_chain.print_chain();
}
