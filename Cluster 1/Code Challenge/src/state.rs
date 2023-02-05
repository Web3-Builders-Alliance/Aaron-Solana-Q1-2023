use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Escrow {
    pub is_initialized: bool,
    pub initializer_pubkey: Pubkey,
    pub temp_token_account_pubkey: Pubkey,
    pub initializer_token_to_receive_account_pubkey: Pubkey,
    pub expected_amount: u64,
    pub unlock_slot: u64,
    pub timeout_slot: u64
}

impl Pack for Escrow {
    //1 (bool) + 3 * 32 (Pubkey) + 1 * 8 (u64) = 105
    // adding 2 u64s for unlock & timeout slots
    const LEN: usize = 105 + 8 + 8;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Escrow::LEN];
        let (
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            expected_amount,
            unlock_slot,
            timeout_slot
        ) = array_refs![src, 1, 32, 32, 32, 8, 8 , 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Escrow {
            is_initialized,
            initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
            temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
            initializer_token_to_receive_account_pubkey: Pubkey::new_from_array(*initializer_token_to_receive_account_pubkey),
            expected_amount: u64::from_le_bytes(*expected_amount),
            unlock_slot: u64::from_le_bytes(*expected_amount),
            timeout_slot: u64::from_le_bytes(*expected_amount),

        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Escrow::LEN];
        let (
            is_initialized_dst,
            initializer_pubkey_dst,
            temp_token_account_pubkey_dst,
            initializer_token_to_receive_account_pubkey_dst,
            expected_amount_dst,
            unlock_slot_dst,
            timeout_slot_dst
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8, 8, 8];

        let Escrow {
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            expected_amount,
            unlock_slot,
            timeout_slot
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
        temp_token_account_pubkey_dst.copy_from_slice(temp_token_account_pubkey.as_ref());
        initializer_token_to_receive_account_pubkey_dst.copy_from_slice(initializer_token_to_receive_account_pubkey.as_ref());
        *expected_amount_dst = expected_amount.to_le_bytes();
        *unlock_slot_dst = unlock_slot.to_le_bytes();
        *timeout_slot_dst = timeout_slot.to_le_bytes();
        
        /*
        
        Questions: 
        - What's w/ DST naming convention
        - Why did we use *? 
        - Why to le bytes as opposed to some other serializer?


            Dean 利迪恩: Otherwise you'd have to do something like clone() or whatever because you'd be passing a memory pointer, not the contents of it
        [6:44 AM]Dean 利迪恩: but dereference is better in this case because reasons

        
        */
        

    }
}

impl Sealed for Escrow {}
    
impl IsInitialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}