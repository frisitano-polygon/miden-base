// PUBLIC CONSTANTS
// ================================================================================================

// BOOKKEEPING
// ------------------------------------------------------------------------------------------------

/// The memory address at which the transaction vault root is stored.
pub const TX_VAULT_ROOT_PTR: u64 = 0;

/// The memory address at which the number of executed notes is stored.
pub const NUM_EXECUTED_NOTES_PTR: u64 = 1;

/// The memory address at which the number of created notes is stored.
pub const NUM_CREATED_NOTES_PTR: u64 = 2;

/// A pointer to the memory address at which a pointer to the current consumed note is stored.
pub const CURRENT_CONSUMED_NOTE_PTR: u64 = 3;

// GLOBAL INPUTS
// ------------------------------------------------------------------------------------------------

/// The memory address at which the global inputs section begins.
pub const GLOBAL_INPUTS_SECTION_OFFSET: u64 = 10;

/// The memory address at which the latest known block hash is stored.
pub const BLK_HASH_PTR: u64 = 10;

/// The memory address at which the account id is stored.
pub const ACCT_ID_PTR: u64 = 11;

/// The memory address at which the initial account hash is stored.
pub const INIT_ACCT_HASH_PTR: u64 = 12;

/// The memory address at which the global nullifier commitment is stored.
pub const NULLIFIER_COM_PTR: u64 = 13;

/// The memory address at which the initial nonce is stored.
pub const INIT_NONCE_PTR: u64 = 14;

// ACCOUNT DATA
// ------------------------------------------------------------------------------------------------

/// The memory address at which the account data section begins
pub const ACCT_DATA_SECTION_OFFSET: u64 = 100;

/// The memory address at which the account id and nonce is stored.
/// The account id is stored in the first element.
/// The account nonce is stored in the fourth element.
pub const ACCT_ID_AND_NONCE_PTR: u64 = 100;

/// The memory address at which the account vault root is stored.
pub const ACCT_VAULT_ROOT_PTR: u64 = 101;

/// The memory address at which the account storage root is stored.
pub const ACCT_STORAGE_ROOT_PTR: u64 = 102;

/// The memory address at which the account code root is stored.
pub const ACCT_CODE_ROOT_PTR: u64 = 103;

// NOTES DATA
// ------------------------------------------------------------------------------------------------

/// The maximum number of assets that can be stored in a single note.
pub const MAX_ASSETS_PER_NOTE: u64 = 256;

/// The size of the memory segment allocated to each note
pub const NOTE_MEM_SIZE: u64 = 1024;

// CONSUMED NOTES DATA
// ------------------------------------------------------------------------------------------------

/// The memory address at which the consumed note section begins.
pub const CONSUMED_NOTE_SECTION_OFFSET: u64 = 1000;

/// The memory address at which the number of consumed notes is stored.
pub const CONSUMED_NOTE_NUM_PTR: u64 = 1000;

/// The offsets at which data of a consumed note is stored relative to the start of its data segment.
pub const CONSUMED_NOTE_HASH_OFFSET: u64 = 0;
pub const CONSUMED_NOTE_SERIAL_NUM_OFFSET: u64 = 1;
pub const CONSUMED_NOTE_SCRIPT_ROOT_OFFSET: u64 = 2;
pub const CONSUMED_NOTE_INPUTS_HASH_OFFSET: u64 = 3;
pub const CONSUMED_NOTE_VAULT_ROOT_OFFSET: u64 = 4;
pub const CONSUMED_NOTE_NUM_ASSETS_OFFSET: u64 = 5;
pub const CONSUMED_NOTE_ASSETS_OFFSET: u64 = 6;

/// The maximum number of consumed notes that can be processed in a single transaction.
pub const MAX_NUM_CONSUMED_NOTES: u64 = 1023;

// CREATED NOTES DATA
// ------------------------------------------------------------------------------------------------

/// The memory address at which the created notes section begins.
pub const CREATED_NOTE_SECTION_OFFSET: u64 = 10000;

/// The offsets at which data of a created note is stored relative to the start of its data segment.
pub const CREATED_NOTE_HASH_OFFET: u64 = 0;
pub const CREATED_NOTE_METADATA_OFFSET: u64 = 1;
pub const CREATED_NOTE_RECIPIENT_OFFSET: u64 = 2;
pub const CREATED_NOTE_VAULT_HASH_OFFSET: u64 = 3;
pub const CREATED_NOTE_NUM_ASSETS_OFFSET: u64 = 4;
pub const CREATED_NOTE_ASSETS_OFFSET: u64 = 5;
