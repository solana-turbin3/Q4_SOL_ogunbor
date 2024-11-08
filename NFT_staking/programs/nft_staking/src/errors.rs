use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Incorrect mint")]
    IncorrectMint,
    #[msg("Incorrect collection")]
    IncorrectCollection,
    #[msg("Collection not verified")]
    CollectionNotVerified,
    #[msg("Max stake reached")]
    MaxStakeReached,
    #[msg("Unstake ealier than required")]
    StakingPeriodNotElapsed,
}
