#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    PreLoading,
    Loading,
    Running(RunState)
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum RunState {
    Menu,
    InGame(bool)
}
