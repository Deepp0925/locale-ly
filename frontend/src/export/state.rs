use std::str::FromStr;

use leptos::{Memo, SignalGet, SignalSetter};
use leptos_router::{create_query_signal, use_query, Params, ParamsError};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum ExportState {
    #[default]
    /// Export is closed
    Closed,
    /// Export is open
    Open,
}

pub const EXPORT_STATE: &str = "export_state";

impl ToString for ExportState {
    fn to_string(&self) -> String {
        match self {
            Self::Closed => "closed".to_owned(),
            Self::Open => "open".to_owned(),
        }
    }
}

impl FromStr for ExportState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "closed" => Ok(Self::Closed),
            "open" => Ok(Self::Open),
            _ => Ok(Self::Closed),
        }
    }
}

impl Params for ExportState {
    fn from_map(map: &leptos_router::ParamsMap) -> Result<Self, ParamsError> {
        let state = match map.get(EXPORT_STATE) {
            Some(state) => ExportState::from_str(state).unwrap(),
            None => ExportState::default(),
        };
        Ok(state)
    }
}

pub fn open_export() {
    let (_, set_state) = create_query_signal::<ExportState>(EXPORT_STATE);
    set_state.set(Some(ExportState::Open));
}

pub fn close_export() {
    let (_, set_state) = create_query_signal::<ExportState>(EXPORT_STATE);
    set_state.set(Some(ExportState::Closed));
}

pub fn is_export_open() -> bool {
    let menu = use_query::<ExportState>();
    if let Ok(state) = menu.get() {
        return state == ExportState::Open;
    }

    false
}

pub fn use_export_state() -> ExportState {
    let menu = use_query::<ExportState>();
    if let Ok(state) = menu.get() {
        return state;
    }

    ExportState::Closed
}

pub fn use_export_state_signal() -> (Memo<Option<ExportState>>, SignalSetter<Option<ExportState>>) {
    let (state, set_state) = create_query_signal::<ExportState>(EXPORT_STATE);
    (state, set_state)
}

pub fn use_export_read_signal() -> Memo<Option<ExportState>> {
    let (state, _) = create_query_signal::<ExportState>(EXPORT_STATE);
    state
}
