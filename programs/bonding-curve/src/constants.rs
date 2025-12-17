// seeds
pub const GLOBAL_SEED: &[u8] = b"global";

// constants
pub const P: u64 = 1_073_000_191; // inital_virtual_token
pub const R: u64 = 30; // initial_virtual_sol
pub const TOTAL_SUPPLY: u64 = 1_000_000_000 * DECIMALS; // 1 billion tokens
pub const BONDING_CURVE_SUPPLY: u64 = 793_100_000 * DECIMALS; // total token supply allocated to bonding curve

pub const DECIMALS: u64 = 1_000_000; // scaling factor for precision for token decimals
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000; // lamports per SOL
