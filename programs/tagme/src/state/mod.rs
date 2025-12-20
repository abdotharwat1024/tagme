mod ownership_history;
mod prod_reg_acc;
mod program_state;
mod status;
mod user;

pub use ownership_history::*;
pub use prod_reg_acc::*;
pub use program_state::*;
pub use status::*;
pub use user::*;

pub const PRODUCT_HISTORY_SEED: &[u8] = b"history";
pub const USER_ACC_SEED: &[u8] = b"user_account";
pub const PRODUCT_ACC_SEED: &[u8] = b"product";
pub const MAX_HISTORY: usize = 5;
