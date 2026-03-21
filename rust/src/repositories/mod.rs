use crate::{
    http::HttpTransport,
    state::SharedState,
};

/// Shared repository context for endpoint modules.
#[derive(Debug, Clone)]
pub struct RepositoryContext {
    pub state: SharedState,
}

impl RepositoryContext {
    pub fn new(state: SharedState) -> Self {
        Self { state }
    }

    pub fn default_headers(&self) -> Vec<(String, String)> {
        let state = self.state.read().expect("shared state read lock");
        HttpTransport::default_headers(&state)
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

/// Advanced repository primitives.
#[derive(Debug, Clone)]
pub struct LiveRepository {
    context: RepositoryContext,
}

impl LiveRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn create_broadcast_endpoint(&self) -> &'static str {
        "/api/v1/live/create/"
    }

    pub fn heartbeat_endpoint(&self, broadcast_id: &str) -> String {
        format!("/api/v1/live/{broadcast_id}/heartbeat_and_get_viewer_count/")
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

#[derive(Debug, Clone)]
pub struct MusicRepository {
    context: RepositoryContext,
}

impl MusicRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn genres_endpoint(&self) -> &'static str {
        "/api/v1/music/genres/"
    }

    pub fn moods_endpoint(&self) -> &'static str {
        "/api/v1/music/moods/"
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

#[derive(Debug, Clone)]
pub struct IgtvRepository {
    context: RepositoryContext,
}

impl IgtvRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn search_endpoint(&self) -> &'static str {
        "/api/v1/igtv/search/"
    }

    pub fn channel_endpoint(&self, user_id: &str) -> String {
        format!("/api/v1/igtv/channel/{user_id}/")
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

#[derive(Debug, Clone)]
pub struct HighlightsRepository {
    context: RepositoryContext,
}

impl HighlightsRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn tray_endpoint(&self, user_id: &str) -> String {
        format!("/api/v1/highlights/{user_id}/highlights_tray/")
    }

    pub fn reels_endpoint(&self) -> &'static str {
        "/api/v1/highlights/reels_media/"
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

#[derive(Debug, Clone)]
pub struct AdsRepository {
    context: RepositoryContext,
}

impl AdsRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn interests_endpoint(&self) -> &'static str {
        "/api/v1/ads/interests/"
    }

    pub fn account_endpoint(&self) -> &'static str {
        "/api/v1/ads/account/"
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

/// Group of advanced repositories.
#[derive(Debug, Clone)]
pub struct AdvancedRepositories {
    pub live: LiveRepository,
    pub music: MusicRepository,
    pub igtv: IgtvRepository,
    pub highlights: HighlightsRepository,
    pub ads: AdsRepository,
}

impl AdvancedRepositories {
    pub fn new(context: RepositoryContext) -> Self {
        Self {
            live: LiveRepository::new(context.clone()),
            music: MusicRepository::new(context.clone()),
            igtv: IgtvRepository::new(context.clone()),
            highlights: HighlightsRepository::new(context.clone()),
            ads: AdsRepository::new(context),
        }
    }
}

/// Collection of currently-ported repositories.
#[derive(Debug, Clone)]
pub struct Repositories {
    pub account: AccountRepository,
    pub media: MediaRepository,
    pub session: SessionRepository,
    pub advanced: AdvancedRepositories,
}

impl Repositories {
    pub fn new(state: SharedState) -> Self {
        let context = RepositoryContext::new(state);
        Self {
            account: AccountRepository::new(context.clone()),
            media: MediaRepository::new(context.clone()),
            session: SessionRepository::new(context.clone()),
            advanced: AdvancedRepositories::new(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::state::shared_from_seed;

    use super::Repositories;

    #[test]
    fn repositories_expose_known_endpoints() {
        let repos = Repositories::new(shared_from_seed("demo"));

        assert_eq!(repos.account.login_endpoint(), "/api/v1/accounts/login/");
        assert_eq!(repos.session.sync_endpoint(), "/api/v1/qe/sync/");
        assert_eq!(repos.media.info_endpoint("123"), "/api/v1/media/123/info/");
        assert_eq!(
            repos.advanced.live.create_broadcast_endpoint(),
            "/api/v1/live/create/"
        );
        assert_eq!(
            repos.advanced.music.genres_endpoint(),
            "/api/v1/music/genres/"
        );
        assert_eq!(
            repos.advanced.igtv.search_endpoint(),
            "/api/v1/igtv/search/"
        );
        assert_eq!(
            repos.advanced.highlights.tray_endpoint("42"),
            "/api/v1/highlights/42/highlights_tray/"
        );
        assert_eq!(
            repos.advanced.ads.account_endpoint(),
            "/api/v1/ads/account/"
        );
    }
}
