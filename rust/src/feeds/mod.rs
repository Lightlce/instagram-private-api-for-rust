use serde::{Deserialize, Serialize};

/// Serializable pagination state for feed continuation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PaginationState {
    pub next_max_id: Option<String>,
    pub pages_fetched: usize,
    pub more_available: bool,
}

/// A page of feed data plus updated pagination state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeedPage<T> {
    pub items: Vec<T>,
    pub state: PaginationState,
}

/// Pull-based feed abstraction with retry-aware pagination semantics.
pub trait Feed {
    type Item;

    fn state(&self) -> &PaginationState;
    fn state_mut(&mut self) -> &mut PaginationState;
    fn request_page(&mut self) -> FeedPage<Self::Item>;

    fn items(&mut self) -> Vec<Self::Item> {
        self.request_page().items
    }

    fn is_finished(&self) -> bool {
        !self.state().more_available
    }
}

/// In-memory feed implementation used by tests and early migration wiring.
#[derive(Debug, Clone)]
pub struct StaticFeed<T> {
    pages: Vec<Vec<T>>,
    state: PaginationState,
}

impl<T: Clone> StaticFeed<T> {
    pub fn new(pages: Vec<Vec<T>>) -> Self {
        let more_available = !pages.is_empty();
        Self {
            pages,
            state: PaginationState {
                more_available,
                ..PaginationState::default()
            },
        }
    }
}

impl<T: Clone> Feed for StaticFeed<T> {
    type Item = T;

    fn state(&self) -> &PaginationState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut PaginationState {
        &mut self.state
    }

    fn request_page(&mut self) -> FeedPage<Self::Item> {
        let index = self.state.pages_fetched;
        let items = self.pages.get(index).cloned().unwrap_or_default();

        self.state.pages_fetched += 1;
        self.state.more_available = self.state.pages_fetched < self.pages.len();
        self.state.next_max_id = self
            .state
            .more_available
            .then(|| format!("page_{}", self.state.pages_fetched));

        FeedPage {
            items,
            state: self.state.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Feed, StaticFeed};

    #[test]
    fn static_feed_iterates_pages_and_tracks_state() {
        let mut feed = StaticFeed::new(vec![vec![1, 2], vec![3]]);

        assert_eq!(feed.items(), vec![1, 2]);
        assert!(!feed.is_finished());

        assert_eq!(feed.items(), vec![3]);
        assert!(feed.is_finished());
    }
}
