use crate::repositories::Repositories;

/// Workflow service for publish flows.
#[derive(Debug, Clone)]
pub struct PublishService {
    repositories: Repositories,
}

impl PublishService {
    pub fn new(repositories: Repositories) -> Self {
        Self { repositories }
    }

    pub fn create_photo_publish_plan(&self) -> Vec<String> {
        vec![
            self.repositories.session.sync_endpoint().to_string(),
            self.repositories.media.configure_endpoint().to_string(),
        ]
    }
}

/// Workflow service for app simulation flows.
#[derive(Debug, Clone)]
pub struct SimulateService {
    repositories: Repositories,
}

impl SimulateService {
    pub fn new(repositories: Repositories) -> Self {
        Self { repositories }
    }

    pub fn warmup_sequence(&self) -> Vec<String> {
        vec![
            self.repositories
                .account
                .current_user_endpoint()
                .to_string(),
            self.repositories.session.sync_endpoint().to_string(),
        ]
    }
}

/// Workflow service for story orchestration.
#[derive(Debug, Clone)]
pub struct StoryService {
    repositories: Repositories,
}

impl StoryService {
    pub fn new(repositories: Repositories) -> Self {
        Self { repositories }
    }

    pub fn preflight_endpoints(&self) -> Vec<String> {
        vec![
            self.repositories.session.sync_endpoint().to_string(),
            self.repositories
                .account
                .current_user_endpoint()
                .to_string(),
        ]
    }
}

/// Workflow service for account/media search.
#[derive(Debug, Clone)]
pub struct SearchService {
    repositories: Repositories,
}

impl SearchService {
    pub fn new(repositories: Repositories) -> Self {
        Self { repositories }
    }

    pub fn bootstrap_endpoints(&self) -> Vec<String> {
        vec![self
            .repositories
            .account
            .current_user_endpoint()
            .to_string()]
    }
}

/// Workflow service for insights collection.
#[derive(Debug, Clone)]
pub struct InsightsService {
    repositories: Repositories,
}

impl InsightsService {
    pub fn new(repositories: Repositories) -> Self {
        Self { repositories }
    }

    pub fn bootstrap_endpoints(&self, media_id: &str) -> Vec<String> {
        vec![self.repositories.media.info_endpoint(media_id)]
    }
}

/// Service registry for orchestration flows.
#[derive(Debug, Clone)]
pub struct Services {
    pub publish: PublishService,
    pub simulate: SimulateService,
    pub story: StoryService,
    pub search: SearchService,
    pub insights: InsightsService,
}

impl Services {
    pub fn new(repositories: Repositories) -> Self {
        Self {
            publish: PublishService::new(repositories.clone()),
            simulate: SimulateService::new(repositories.clone()),
            story: StoryService::new(repositories.clone()),
            search: SearchService::new(repositories.clone()),
            insights: InsightsService::new(repositories),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{repositories::Repositories, state::shared_from_seed};

    use super::Services;

    #[test]
    fn services_expose_bootstrap_sequences() {
        let services = Services::new(Repositories::new(shared_from_seed("demo")));

        assert_eq!(services.publish.create_photo_publish_plan().len(), 2);
        assert_eq!(services.search.bootstrap_endpoints().len(), 1);
        assert_eq!(services.insights.bootstrap_endpoints("100").len(), 1);
    }
}
