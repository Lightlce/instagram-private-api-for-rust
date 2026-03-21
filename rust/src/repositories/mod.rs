use crate::{http::HttpTransport, state::State};

/// Shared repository context for endpoint modules.
#[derive(Debug, Clone)]
pub struct RepositoryContext {
    pub state: State,
}

impl RepositoryContext {
    pub fn new(state: State) -> Self {
        Self { state }
    }

    pub fn default_headers(&self) -> Vec<(String, String)> {
        HttpTransport::default_headers(&self.state)
    }
}

/// Tier-1 account repository primitives.
#[derive(Debug, Clone)]
pub struct AccountRepository {
    context: RepositoryContext,
}

impl AccountRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn login_endpoint(&self) -> &'static str {
        "/api/v1/accounts/login/"
    }

    pub fn current_user_endpoint(&self) -> &'static str {
        "/api/v1/accounts/current_user/?edit=true"
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

/// Tier-1 media repository primitives.
#[derive(Debug, Clone)]
pub struct MediaRepository {
    context: RepositoryContext,
}

impl MediaRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn configure_endpoint(&self) -> &'static str {
        "/api/v1/media/configure/"
    }

    pub fn info_endpoint(&self, media_id: &str) -> String {
        format!("/api/v1/media/{media_id}/info/")
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

/// Tier-1 session repository primitives.
#[derive(Debug, Clone)]
pub struct SessionRepository {
    context: RepositoryContext,
}

impl SessionRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn sync_endpoint(&self) -> &'static str {
        "/api/v1/qe/sync/"
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

/// Collection of currently-ported repositories.
#[derive(Debug, Clone)]
pub struct Repositories {
    pub account: AccountRepository,
    pub media: MediaRepository,
    pub session: SessionRepository,
}

impl Repositories {
    pub fn new(state: State) -> Self {
        let context = RepositoryContext::new(state);
        Self {
            account: AccountRepository::new(context.clone()),
            media: MediaRepository::new(context.clone()),
            session: SessionRepository::new(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::state::State;

    use super::Repositories;

    #[test]
    fn tier1_repositories_expose_known_endpoints() {
        let repos = Repositories::new(State::from_seed("demo"));

        assert_eq!(repos.account.login_endpoint(), "/api/v1/accounts/login/");
        assert_eq!(repos.session.sync_endpoint(), "/api/v1/qe/sync/");
        assert_eq!(repos.media.info_endpoint("123"), "/api/v1/media/123/info/");
    }
}
