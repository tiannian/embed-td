use serde::Serialize;
use tm_abci::ApplicationXX;

/// ABCI Application of tendermint
pub trait App: ApplicationXX {
    type AppState: Serialize;

    fn app_state(&self) -> Self::AppState;
}

impl App for () {
    type AppState = ();

    fn app_state(&self) -> Self::AppState {}
}
