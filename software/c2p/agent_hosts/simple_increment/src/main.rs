use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use agent_host::AgentHost;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
struct GameState {
    counter: i32,
}

#[derive(Serialize, Deserialize, Clone)]
struct AgentMove {
    next_count: i32,
}

fn game_engine(agent_move: AgentMove) -> GameState {
    GameState { counter: agent_move.next_count }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get path to WASM file using CARGO_MANIFEST_DIR
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let wasm_path = PathBuf::from(manifest_dir)
        .join("../../target/wasm32-wasip1/release/simple_increment.wasm");

    // Debug output to verify path
    println!("Loading WASM from: {}", wasm_path.display());
    
    let wasm_bytes = std::fs::read(&wasm_path)
        .map_err(|e| format!("Failed to read WASM at {}: {}", wasm_path.display(), e))?;

    // WASI setup
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    // Create host
    let mut host = AgentHost::new(
        &wasm_bytes,
        game_engine,
        &mut store,
        &linker
    )?;

    // Game loop
    let mut game_state = GameState { counter: 0 };
    let mut internal_state = Vec::new();

    for i in 0..100 {
        let (agent_move, new_internal_state) = host.process_turn(game_state, &internal_state)?;
        println!("Iteration {}: counter = {}", i, agent_move.next_count);
        
        game_state = game_engine(agent_move);
        internal_state = new_internal_state;
    }

    println!("Final counter: {}", game_state.counter);
    Ok(())
}
