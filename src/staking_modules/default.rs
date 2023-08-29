pub struct DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    sc_ref: &'a C,
}

impl<'a, C> DefaultStakingModule<'a, C>
where
    C: crate::storage::config::ConfigModule,
    C: crate::storage::score::ScoreStorageModule,
{
    pub fn new(sc_ref: &'a C) -> Self {
        Self { sc_ref }
    }

    pub fn dummy(&self) {
        self.sc_ref.blockchain().get_block_timestamp();
    }
}
