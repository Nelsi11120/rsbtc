pub mod crypto;
pub mod sha256;
pub mod types;
pub mod util;

use uint::construct_uint;
// construct an unsigned 256-bit integer
// consisting of 4*64-bit words
construct_uint! {
    pub struct U256(4);
}
