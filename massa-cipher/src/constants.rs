// Copyright (c) 2022 MASSA LABS <info@massa.net>

//! massa-cipher constant values.
//!
//! Read `lib.rs` module documentation for more information.

use pbkdf2::Params;

/// AES-GCM-SIV nonce size.
pub const NONCE_SIZE: usize = 12;

/// PBKDF2 salt size.
pub const SALT_SIZE: usize = 12;

/// PBKDF2 base64 salt size.
pub const B64_SALT_SIZE: usize = (SALT_SIZE / 3) * 4;

/// PBKDF2 hash parameters.
pub const HASH_PARAMS: Params = Params {
    rounds: 10_000,
    output_length: 32,
};
