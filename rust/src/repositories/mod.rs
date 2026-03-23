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

/// User repository primitives.
#[derive(Debug, Clone)]
pub struct UserRepository {
    context: RepositoryContext,
}

impl UserRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn info_endpoint(&self, id: &str) -> String {
        format!("/api/v1/users/{id}/info/")
    }

    pub fn username_info_endpoint(&self, username: &str) -> String {
        format!("/api/v1/users/{username}/usernameinfo/")
    }

    pub fn search_endpoint(&self) -> &'static str {
        "/api/v1/users/search/"
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

/// Friendship repository primitives.
#[derive(Debug, Clone)]
pub struct FriendshipRepository {
    context: RepositoryContext,
}

impl FriendshipRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn show_endpoint(&self, id: &str) -> String {
        format!("/api/v1/friendships/show/{id}/")
    }

    pub fn show_many_endpoint(&self) -> &'static str {
        "/api/v1/friendships/show_many/"
    }

    pub fn action_endpoint(&self, action: &str, id: &str) -> String {
        format!("/api/v1/friendships/{action}/{id}/")
    }

    pub fn followers_endpoint(&self, id: &str) -> String {
        format!("/api/v1/friendships/{id}/followers/")
    }

    pub fn following_endpoint(&self, id: &str) -> String {
        format!("/api/v1/friendships/{id}/following/")
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

/// Direct repository primitives.
#[derive(Debug, Clone)]
pub struct DirectRepository {
    context: RepositoryContext,
}

impl DirectRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn create_group_thread_endpoint(&self) -> &'static str {
        "/api/v1/direct_v2/create_group_thread/"
    }

    pub fn ranked_recipients_endpoint(&self) -> &'static str {
        "/api/v1/direct_v2/ranked_recipients/"
    }

    pub fn get_presence_endpoint(&self) -> &'static str {
        "/api/v1/direct_v2/get_presence/"
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

/// Direct thread repository primitives.
#[derive(Debug, Clone)]
pub struct DirectThreadRepository {
    context: RepositoryContext,
}

impl DirectThreadRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn thread_endpoint(&self, thread_id: &str) -> String {
        format!("/api/v1/direct_v2/threads/{thread_id}/")
    }

    pub fn approve_endpoint(&self, thread_id: &str) -> String {
        format!("/api/v1/direct_v2/threads/{thread_id}/approve/")
    }

    pub fn decline_endpoint(&self, thread_id: &str) -> String {
        format!("/api/v1/direct_v2/threads/{thread_id}/decline/")
    }

    pub fn broadcast_endpoint(&self, item: &str) -> String {
        format!("/api/v1/direct_v2/threads/broadcast/{item}/")
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

/// Discover repository primitives.
#[derive(Debug, Clone)]
pub struct DiscoverRepository {
    context: RepositoryContext,
}

impl DiscoverRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn chaining_endpoint(&self) -> &'static str {
        "/api/v1/discover/chaining/"
    }

    pub fn topical_explore_endpoint(&self) -> &'static str {
        "/api/v1/discover/topical_explore/"
    }

    pub fn mark_su_seen_endpoint(&self) -> &'static str {
        "/api/v1/discover/mark_su_seen/"
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

/// Tag repository primitives.
#[derive(Debug, Clone)]
pub struct TagRepository {
    context: RepositoryContext,
}

impl TagRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn search_endpoint(&self) -> &'static str {
        "/api/v1/tags/search/"
    }

    pub fn sections_endpoint(&self, tag: &str) -> String {
        format!("/api/v1/tags/{tag}/sections/")
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

/// Location repository primitives.
#[derive(Debug, Clone)]
pub struct LocationRepository {
    context: RepositoryContext,
}

impl LocationRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn info_endpoint(&self, id: &str) -> String {
        format!("/api/v1/locations/{id}/info/")
    }

    pub fn story_endpoint(&self, id: &str) -> String {
        format!("/api/v1/locations/{id}/story/")
    }

    pub fn search_endpoint(&self) -> &'static str {
        "/api/v1/location_search/"
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

/// Upload repository primitives.
#[derive(Debug, Clone)]
pub struct UploadRepository {
    context: RepositoryContext,
}

impl UploadRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn photo_rupload_endpoint(&self, name: &str) -> String {
        format!("/rupload_igphoto/{name}")
    }

    pub fn video_rupload_endpoint(&self, name: &str) -> String {
        format!("/rupload_igvideo/{name}")
    }

    pub fn headers(&self) -> Vec<(String, String)> {
        self.context.default_headers()
    }
}

/// Status repository primitives.
#[derive(Debug, Clone)]
pub struct StatusRepository {
    context: RepositoryContext,
}

impl StatusRepository {
    pub fn new(context: RepositoryContext) -> Self {
        Self { context }
    }

    pub fn get_viewable_statuses_endpoint(&self) -> &'static str {
        "/api/v1/status/get_viewable_statuses/"
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
    pub user: UserRepository,
    pub friendship: FriendshipRepository,
    pub direct: DirectRepository,
    pub direct_thread: DirectThreadRepository,
    pub discover: DiscoverRepository,
    pub tag: TagRepository,
    pub location: LocationRepository,
    pub upload: UploadRepository,
    pub status: StatusRepository,
    pub advanced: AdvancedRepositories,
}

impl Repositories {
    pub fn new(state: SharedState) -> Self {
        let context = RepositoryContext::new(state);
        Self {
            account: AccountRepository::new(context.clone()),
            media: MediaRepository::new(context.clone()),
            session: SessionRepository::new(context.clone()),
            user: UserRepository::new(context.clone()),
            friendship: FriendshipRepository::new(context.clone()),
            direct: DirectRepository::new(context.clone()),
            direct_thread: DirectThreadRepository::new(context.clone()),
            discover: DiscoverRepository::new(context.clone()),
            tag: TagRepository::new(context.clone()),
            location: LocationRepository::new(context.clone()),
            upload: UploadRepository::new(context.clone()),
            status: StatusRepository::new(context.clone()),
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
        assert_eq!(repos.user.info_endpoint("42"), "/api/v1/users/42/info/");
        assert_eq!(
            repos.friendship.action_endpoint("follow", "42"),
            "/api/v1/friendships/follow/42/"
        );
        assert_eq!(
            repos.direct.create_group_thread_endpoint(),
            "/api/v1/direct_v2/create_group_thread/"
        );
        assert_eq!(
            repos.direct_thread.approve_endpoint("123"),
            "/api/v1/direct_v2/threads/123/approve/"
        );
        assert_eq!(
            repos.discover.topical_explore_endpoint(),
            "/api/v1/discover/topical_explore/"
        );
        assert_eq!(repos.tag.search_endpoint(), "/api/v1/tags/search/");
        assert_eq!(
            repos.location.search_endpoint(),
            "/api/v1/location_search/"
        );
        assert_eq!(
            repos.upload.photo_rupload_endpoint("upload-name"),
            "/rupload_igphoto/upload-name"
        );
        assert_eq!(
            repos.status.get_viewable_statuses_endpoint(),
            "/api/v1/status/get_viewable_statuses/"
        );
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
