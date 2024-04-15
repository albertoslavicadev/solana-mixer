use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    program_pack::{Pack, IsInitialized, Sealed},
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    hash::{hash, Hash},
};

entrypoint!(process_instruction);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Escrow {
    is_initialized: bool,
    depositor_pubkey: Pubkey,
    secret_hash: Hash,
}

impl Sealed for Escrow {}

impl IsInitialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Escrow {
    const LEN: usize = 1 + 32 + 32; // bool + Pubkey + Hash

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0] = self.is_initialized as u8;
        dst[1..33].copy_from_slice(self.depositor_pubkey.as_ref());
        dst[33..65].copy_from_slice(self.secret_hash.as_ref());
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let is_initialized = src[0] != 0;
        let depositor_pubkey = Pubkey::new_from_array([
            src[1], src[2], src[3], src[4], src[5], src[6], src[7], src[8],
            src[9], src[10], src[11], src[12], src[13], src[14], src[15], src[16],
            src[17], src[18], src[19], src[20], src[21], src[22], src[23], src[24],
            src[25], src[26], src[27], src[28], src[29], src[30], src[31], src[32],
        ]);
        let secret_hash = Hash::new_from_array([
            src[33], src[34], src[35], src[36], src[37], src[38], src[39], src[40],
            src[41], src[42], src[43], src[44], src[45], src[46], src[47], src[48],
            src[49], src[50], src[51], src[52], src[53], src[54], src[55], src[56],
            src[57], src[58], src[59], src[60], src[61], src[62], src[63], src[64],
        ]);

        Ok(Escrow {
            is_initialized,
            depositor_pubkey,
            secret_hash,
        })
    }
}

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let initializer = next_account_info(accounts_iter)?;
    let escrow_account = next_account_info(accounts_iter)?;

    match input[0] {
        0 => {
            let _rent = &Rent::from_account_info(next_account_info(accounts_iter)?)?;
            let lamports_to_deposit = 1_000_000; // Only 1 sol deposit is possible

            if **escrow_account.lamports.borrow() > 0 || Escrow::unpack_unchecked(&escrow_account.data.borrow())?.is_initialized {
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            **escrow_account.lamports.borrow_mut() = lamports_to_deposit;
            **initializer.lamports.borrow_mut() -= lamports_to_deposit;
            solana_program::program::invoke(
                &system_instruction::transfer(initializer.key, escrow_account.key, lamports_to_deposit),
                &[initializer.clone(), escrow_account.clone()],
            )?;

            let secret_hash = hash(&input[1..input.len()]);
            let escrow_state = Escrow {
                is_initialized: true,
                depositor_pubkey: *initializer.key,
                secret_hash,
            };
            Escrow::pack(escrow_state, &mut escrow_account.data.borrow_mut())?;
        },
        1 => {
            let escrow_state = Escrow::unpack(&escrow_account.data.borrow())?;
            if escrow_state.depositor_pubkey != *initializer.key {
                return Err(ProgramError::IncorrectProgramId);
            }

            let provided_hash = hash(&input[1..input.len()]);
            if provided_hash != escrow_state.secret_hash {
                return Err(ProgramError::InvalidArgument);
            }

            **initializer.lamports.borrow_mut() += **escrow_account.lamports.borrow();
            **escrow_account.lamports.borrow_mut() = 0;
            Escrow::pack(Escrow::default(), &mut escrow_account.data.borrow_mut())?;
        },
        _ => return Err(ProgramError::InvalidInstructionData),
    }

    Ok(())
}
