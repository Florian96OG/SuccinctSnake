use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use sp1_sdk::{ProverClient, SP1Stdin};

#[derive(Serialize, Deserialize)]
struct GameEvent {
    event_type: String,
    timestamp: u64,
}

#[wasm_bindgen]
pub fn prove_score(events: JsValue) -> JsValue {
    let game_events: Vec<GameEvent> = serde_wasm_bindgen::from_value(events).unwrap();
    let score = game_events.iter().fold(0, |acc, event| {
        match event.event_type.as_str() {
            "food" => acc + 1,
            "special_food" => acc + 5,
            _ => acc,
        }
    });
    let proof = vec![1, 2, 3]; // Mock proof for WASM
    let result = (score, proof);
    serde_wasm_bindgen::to_value(&result).unwrap()
}

// SP1 proving function for ELF
#[cfg(not(target_arch = "wasm32"))]
pub fn prove_score_with_sp1(events: Vec<GameEvent>) -> (u32, Vec<u8>) {
    let score = events.iter().fold(0, |acc, event| {
        match event.event_type.as_str() {
            "food" => acc + 1,
            "special_food" => acc + 5,
            _ => acc,
        }
    });
    
    let mut stdin = SP1Stdin::new();
    stdin.write(&events);
    stdin.write(&score);
    
    let client = ProverClient::new();
    let (proof, _) = client.execute("target/release/score_proof", stdin).run().unwrap();
    (score, proof)
}

// Main function for ELF binary
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Example usage for testing
    let events = vec![
        GameEvent { event_type: "food".to_string(), timestamp: 1630000000 },
        GameEvent { event_type: "special_food".to_string(), timestamp: 1630000001 },
    ];
    let (score, proof) = prove_score_with_sp1(events);
    println!("Score: {}, Proof: {:?}", score, proof);
}
