use wasmtime::*;
use wasmtime_wasi::WasiCtx;
use sha2::{Sha512, Digest};
use serde::{Serialize, Deserialize};
use std::error::Error;

pub trait GameMessage: Serialize + for<'a> Deserialize<'a> + Clone {}
impl<T: Serialize + for<'a> Deserialize<'a> + Clone> GameMessage for T {}

pub struct AgentHost<'a, S: GameMessage, M: GameMessage> {
    engine: &'a mut Store<WasiCtx>,
    instance: Instance,
    memory: Memory,
    game_callback: Box<dyn Fn(M) -> S>
}

impl<'a, S: GameMessage, M: GameMessage> AgentHost<'a, S, M> {
    pub fn new(
        wasm_bytes: &[u8],
        game_callback: impl Fn(M) -> S + 'static,
        store: &'a mut Store<WasiCtx>,
        linker: &Linker<WasiCtx>,
    ) -> Result<Self, Box<dyn Error>> {
        let module = Module::new(store.engine(), wasm_bytes)?;
        let instance = linker.instantiate(&mut *store, &module)?;
        let memory = instance.get_memory(&mut *store, "memory")
            .ok_or("Failed to get WASM memory")?;

        Ok(Self {
            engine: store,
            instance,
            memory,
            game_callback: Box::new(game_callback)
        })
    }

    pub fn process_turn(&mut self, game_state: S, internal_state: &[u8]) 
        -> Result<(M, Vec<u8>), Box<dyn Error>> 
    {
        let state_bytes = rmp_serde::to_vec(&game_state)?;
        let state_hash = Sha512::digest(&state_bytes);
        
        let mut input_data = Vec::new();
        input_data.extend_from_slice(&(state_bytes.len() as i32).to_ne_bytes());
        input_data.extend_from_slice(&state_hash);
        input_data.extend_from_slice(&(internal_state.len() as i32).to_ne_bytes());
        input_data.extend_from_slice(&Sha512::digest(internal_state));
        input_data.extend_from_slice(&state_bytes);
        input_data.extend_from_slice(internal_state);

        let input_ptr = self.write_to_memory(&input_data)?;
        let process_turn = self.instance.get_typed_func::<i32, i32>(&mut *self.engine, "process_turn")?;
        let result_ptr = process_turn.call(&mut *self.engine, input_ptr)?;

        let (agent_move, new_internal_state) = self.read_agent_response(result_ptr)?;
        
        let _new_game_state = (self.game_callback)(agent_move.clone());

        Ok((agent_move, new_internal_state))
    }

    fn write_to_memory(&mut self, data: &[u8]) -> Result<i32, Box<dyn Error>> {
        let ptr = self.memory.data_size(&mut *self.engine) as i32;
        self.memory.grow(&mut *self.engine, (data.len() / 65536 + 1) as u64)?;
        
        let memory_slice = unsafe {
            std::slice::from_raw_parts_mut(
                self.memory.data_mut(&mut *self.engine).as_mut_ptr().add(ptr as usize),
                data.len()
            )
        };
        memory_slice.copy_from_slice(data);
        
        Ok(ptr)
    }

    fn read_agent_response(&mut self, ptr: i32) -> Result<(M, Vec<u8>), Box<dyn Error>> {
        let memory_slice = self.memory.data(&mut *self.engine);
        
        let move_len = i32::from_ne_bytes(memory_slice[ptr as usize..ptr as usize + 4]
            .try_into()?);
        let move_hash_offset = ptr as usize + 4;
        let internal_state_len = i32::from_ne_bytes(
            memory_slice[move_hash_offset + 64..move_hash_offset + 68].try_into()?
        );
        
        let move_data_offset = move_hash_offset + 68 + 64;
        let move_data = &memory_slice[move_data_offset..move_data_offset + move_len as usize];
        
        let calculated_hash = Sha512::digest(move_data);
        let stored_hash = &memory_slice[move_hash_offset..move_hash_offset + 64];
        if calculated_hash.as_slice() != stored_hash {
            return Err("Move hash verification failed".into());
        }
        
        let internal_state_offset = move_data_offset + move_len as usize;
        let internal_state = memory_slice[internal_state_offset..
            internal_state_offset + internal_state_len as usize].to_vec();
        
        let agent_move: M = rmp_serde::from_slice(move_data)?;
        
        Ok((agent_move, internal_state))
    }
}