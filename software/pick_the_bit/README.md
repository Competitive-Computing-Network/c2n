# Pick the Bit and Competitive Computing Platform - Towards a New Benchmark for AGI System Performance

---

### 1. The Pick the Bit Game

#### 1.1 Game Overview

Pick the Bit (PtB) is a turn-based, multi-agent (minimum of 2 agents but theoretically an unlimited number of agents) game where agents compete by guessing a binary value—either 0 or 1—each round. The goal is to avoid picking the bit chosen by the majority of agents. Agents that pick the majority bit lose health (when their health goes to 0 or below, the agent 'dies' and is removed from the game), and the game continues until only one agent remains.

#### 1.2 Game Mechanics

1. **Health Dynamics:**

   - Each agent starts with a fixed amount of health points (HP).
   - Agents that guess the majority bit lose health points equal to a predetermined loss value.
   - Agents that guess the minority bit retain their health.
   - Health loss scales asymptotically in later rounds, increasing the stakes over time. The reason for this is because earlier rounds of the game are more random and a loss should not incur as much health loss.

2. **Random Noise Agents:**

   - At a minimum, one random agent with infinite health is always present to enable tie breaks when there are only two agents left.
   - Additional random agents in addition to the single random agent can be added from the beginning to increase random noise and maintain unpredictability.&#x20;
   - These random agents choose their bits pseudorandomly based on a cryptographically secure PRNG with a securely selected seed value.

3. **Hidden Information:**

   - The health levels of other agents and the number of agents choosing each bit are hidden, forcing agents to infer patterns and make strategic guesses.
   - The only four things that an agent receives as inputs each round are:
     - The current round number.
     - The majority and minority bits from the previous round.
     - The agent's own current health level.
     - The amount of health that will be lost for a loss of the next round (also, the health loss schedule will be passed to the agent at the beginning of the game at a minimum).

4. **Incentivizing Monetary Rewards:**

   - Each round, agents that survive collect tokens, representing an equal share of the health points lost by the defeated agents.
   - The total tokens accrued by an agent are not revealed to any of the agents at all (including the agent that is assigned the tokens), and do not give any advantage in the game.&#x20;
   - The tokens an agent ends up with at the end of the game can be redeemed for monetary rewards by the team that owns the agent at the end of the game.
   - A percentage of the prize pool is reserved for the game winner, ensuring that strategic play and survival remain paramount.

5. **Game Complexity:**

   - PtB rewards non-random play by favoring agents that detect and exploit patterns in opponents' choices. Random strategies are penalized over time due to predictable health loss.
   - Under the token system, pure random play by an agent will strongly tend towards monetary loss since a percentage of the prize pool is pre-allocated to the winning agent.

---

### 2. Running PtB on C2P

#### 2.1 The Competitive Computing Platform (C2P)

The Competitive Computing Platform (C2P) is an isolated, resource-constrained environment for executing AI-generated agents. It enforces standardization across competitions, ensuring fairness and reproducibility.

#### 2.2 Agent Constraints

1. **WASM WASI Modules:**

   - All agents are submitted as WebAssembly (WASM) WASI modules, ensuring portability and security.

2. **Resource Limits:**

   - Memory: Limited to 4 GiB.
   - Fuel: Execution is capped using Wasmtime's fuel feature to ensure computational fairness.
   - No Networking Access: Agents are entirely sandboxed, removing external dependencies or external learning.

3. **Game State Communication:**

   - Agents receive game state updates via shared memory and submit their moves back through the same mechanism. No external communication is permitted, ensuring that all strategies are self-contained.

#### 2.3 C2P Architecture

1. **Broker and Hosts:**

   - The game broker orchestrates competitions, communicating game state updates to agent hosts and logging outcomes.

2. **Single-Node Execution:**

   - For simplicity, C2P competitions can run on a single node with all components (broker, Kafka instance, WASM modules) co-located.

3. **Turn-Based Execution:**

   - Each turn, agents receive the game state and submit their moves asynchronously. The broker processes all moves, calculates health adjustments, and updates the game state for the next round.

#### 2.4 Benchmarking Independence

   - PtB on C2P can benchmark any AGI system, independent of its architecture. The only requirement is that the AGI system generates a WASM WASI agent for the PtB game.
   - This architectural independence ensures that C2P provides a level playing field for all AGI systems, allowing researchers and developers to focus on algorithmic sophistication rather than hardware or language-specific implementations.

---

### 3. PtB and C2P as a Benchmark for AGI Performance

#### 3.1 Benchmarking AGI Through PtB

Pick the Bit is designed to test core AGI capabilities:

1. **Strategic Adaptation:**

   - AGI systems must adapt to the shifting meta-game, learning and optimizing strategies with limited feedback.

2. **Pattern Recognition:**

   - Detecting and responding to subtle patterns in agent behavior and game state is critical for survival.

3. **Robustness Under Constraints:**

   - The WASM WASI sandbox ensures that agent performance is tied solely to its algorithmic sophistication, not hardware advantages.

#### 3.2 C2P as a Universal Standard

1. **Decoupling from Hardware:**

   - By requiring agents to run on commodity hardware with standardized constraints, C2P removes externalities, enabling direct comparisons between AGI systems.

2. **Interoperability:**

   - WASM WASI ensures agents can be developed in any language that compiles to WASM, making C2P accessible to a wide range of researchers and organizations.

3. **Transparent Competitions:**

   - C2P logs all game state updates and agent moves, providing a fully auditable record of each competition.

#### 3.3 Meta-Learning and AGI Evaluation

1. **Dynamic Agent Generation:**
   - PtB encourages the use of meta-learning systems that dynamically generate agents tailored to the game environment.
   - By iteratively refining agents through competitions, AGI systems can demonstrate their ability to generalize, adapt, and innovate.

---

### 4. Conclusion

Pick the Bit (PtB) and the Competitive Computing Platform (C2P) together represent a new frontier in AGI benchmarking. PtB's dynamic and evolving meta-game challenges agents to excel in adaptability, pattern recognition, and strategic thinking, while C2P provides a standardized, resource-constrained environment for fair competition. By isolating agent performance from hardware advantages and enabling reproducible evaluations, PtB and C2P offer a universal platform for AGI research and benchmarking, pushing the boundaries of what intelligent systems can achieve. Through these competitions, the AI community can foster innovation, collaboration, and progress toward truly general intelligence.

