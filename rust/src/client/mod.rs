use crate::{repositories::Repositories, services::Services, state::State};

/// High-level API client surface (`IgApiClient` equivalent).
#[derive(Debug, Clone)]
pub struct IgApiClient {
    pub state: State,
    pub repositories: Repositories,
    pub services: Services,
}

impl IgApiClient {
    pub fn from_seed(seed: impl Into<String>) -> Self {
        let state = State::from_seed(seed);
        let repositories = Repositories::new(state.clone());
        let services = Services::new(repositories.clone());

        Self {
            state,
            repositories,
            services,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IgApiClient;

    #[test]
    fn client_bootstraps_repositories_and_services() {
        let client = IgApiClient::from_seed("demo");

        assert_eq!(
            client.repositories.account.login_endpoint(),
            "/api/v1/accounts/login/"
        );
        assert_eq!(client.services.publish.create_photo_publish_plan().len(), 2);
    }
}
