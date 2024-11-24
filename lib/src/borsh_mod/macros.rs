#[macro_export]
macro_rules! borsh_instruction {
    ($discriminator_name:ident, $struct_name:ident) => {
        impl $crate::Discriminator for $struct_name {
            fn discriminator() -> u8 {
                $discriminator_name::$struct_name as u8
            }
        }

        impl $struct_name {
            pub fn try_to_bytes(
                &self,
            ) -> Result<Vec<u8>, solana_program::program_error::ProgramError> {
                let instruction_vec = borsh::to_vec(self)?;
                Ok([
                    [$discriminator_name::$struct_name as u8].to_vec(),
                    instruction_vec,
                ]
                .concat())
            }
        }

        impl $struct_name {
            pub fn try_from_bytes(
                data: &[u8],
            ) -> Result<Self, solana_program::program_error::ProgramError> {
                Self::try_from_slice(data).or(Err(
                    solana_program::program_error::ProgramError::InvalidInstructionData,
                ))
            }
        }
    };
}
