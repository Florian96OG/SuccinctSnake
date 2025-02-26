use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct GameEvent {
    event_type: String,  // "food" or "special_food"
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
    
    let proof = vec![1, 2, 3]; // Mock proof
    let result = (score, proof);
    serde_wasm_bindgen::to_value(&result).unwrap()
}
