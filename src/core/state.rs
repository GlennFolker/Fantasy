#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum FState {
    PreLoading,
    Loading,
    Menu,
    InGame
}
