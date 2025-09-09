use std::io;
use vortex_hash::hash;

fn main() {
    println!("=== VortexHASH CLI ===");
    println!("Quantum-resistant hash function demonstration");
    println!();
    println!("How it works:");
    println!("1. Absorb phase: Data is XORed into the sponge state");
    println!("2. Squeeze phase: Permutation generates the final hash");
    println!();
    println!("Sponge Construction Diagram:");
    println!("Data -> [XOR] -> Sponge State -> [Permute] -> Hash Output");
    println!("         ↑");
    println!("         Rate (32 bytes per block)");
    println!();
    println!("Enter data to hash (or 'exit' to quit):");

    let stdin = io::stdin();
    for line in stdin.lines() {
        let input = line.unwrap().trim().to_string();
        if input == "exit" {
            break;
        }
        if input.is_empty() {
            continue;
        }
        let hash_result = hash(input.as_bytes());
        println!("Input: {}", input);
        println!("Hash: {:?}", hash_result);
        println!();
        println!("Enter next data (or 'exit' to quit):");
    }
    println!("Goodbye!");
}