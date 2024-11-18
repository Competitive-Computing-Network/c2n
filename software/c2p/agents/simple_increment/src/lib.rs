use serde::{Serialize, Deserialize};
use sha2::{Sha512, Digest};

#[derive(Serialize, Deserialize)]
struct GameState {
    counter: i32,
}

#[derive(Serialize, Deserialize)]
struct AgentMove {
    next_count: i32,
}

#[no_mangle]
pub extern "C" fn process_turn(input_ptr: i32) -> i32 {
    unsafe {
        // Read lengths and hashes
        let game_state_len = *(input_ptr as *const i32);
        let game_state_hash = std::slice::from_raw_parts(
            (input_ptr as *const u8).offset(4), 
            64
        );
        let internal_state_len = *(input_ptr as *const i32).offset(17); // (4+64)/4
        let internal_state_hash = std::slice::from_raw_parts(
            (input_ptr as *const u8).offset(72), // 4+64+4
            64
        );

        // Read game state
        let game_state_bytes = std::slice::from_raw_parts(
            (input_ptr as *const u8).offset(136), // 4+64+4+64
            game_state_len as usize
        );
        
        // Verify game state hash
        let calculated_hash = Sha512::digest(game_state_bytes);
        assert!(calculated_hash[..] == game_state_hash[..], "Game state hash mismatch");

        // Read internal state
        let internal_state_bytes = std::slice::from_raw_parts(
            (input_ptr as *const u8).offset(136 + game_state_len as isize),
            internal_state_len as usize
        );
        
        // Verify internal state hash
        let calculated_internal_hash = Sha512::digest(internal_state_bytes);
        assert!(calculated_internal_hash[..] == internal_state_hash[..], "Internal state hash mismatch");

        // Process game state
        let game_state: GameState = rmp_serde::from_slice(game_state_bytes).unwrap();

        // Create agent's move
        let agent_move = AgentMove {
            next_count: game_state.counter + 1,
        };

        // Prepare response in same format
        let move_bytes = rmp_serde::to_vec(&agent_move).unwrap();
        let move_len = move_bytes.len() as i32;
        let move_hash = Sha512::digest(&move_bytes);

        // Create new internal state
        let new_internal_state = game_state.counter + 1;
        let internal_state_bytes = new_internal_state.to_ne_bytes();
        let internal_state_len = internal_state_bytes.len() as i32;
        let internal_state_hash = Sha512::digest(&internal_state_bytes);

        // Allocate and fill response
        let mut response = Vec::with_capacity(
            4 + 64 + 4 + 64 + move_bytes.len() + internal_state_bytes.len()
        );

        response.extend_from_slice(&move_len.to_ne_bytes());
        response.extend_from_slice(&move_hash);
        response.extend_from_slice(&internal_state_len.to_ne_bytes());
        response.extend_from_slice(&internal_state_hash);
        response.extend_from_slice(&move_bytes);
        response.extend_from_slice(&internal_state_bytes);

        let response_ptr = response.as_ptr() as i32;
        std::mem::forget(response);

        response_ptr
    }
}