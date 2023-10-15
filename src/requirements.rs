multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait RequirementsModule {
    fn require_token_matches(&self, token_1: &TokenIdentifier, token_2: &TokenIdentifier) {
        require!(token_1 == token_2, "Tokens must match");
    }
}
