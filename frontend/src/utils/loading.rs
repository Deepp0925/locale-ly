#[derive(Debug, Clone)]
pub enum LoadingState<T> {
    /// No state
    None,
    /// loading state
    Loading,
    /// loaded state
    Loaded(T),
    /// error state
    Error(String),
}
