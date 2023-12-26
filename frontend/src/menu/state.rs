use std::str::FromStr;

use leptos::{Memo, SignalGet, SignalSetter};
use leptos_router::{create_query_signal, use_query, Params, ParamsError};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum MenuState {
    #[default]
    /// Menu is closed
    Closed,
    /// Menu is open
    Open,
}

pub const MENU_STATE: &str = "menu_state";

impl ToString for MenuState {
    fn to_string(&self) -> String {
        match self {
            Self::Closed => "closed".to_owned(),
            Self::Open => "open".to_owned(),
        }
    }
}

impl FromStr for MenuState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "closed" => Ok(Self::Closed),
            "open" => Ok(Self::Open),
            _ => Ok(Self::Closed),
        }
    }
}

impl Params for MenuState {
    fn from_map(map: &leptos_router::ParamsMap) -> Result<Self, ParamsError> {
        let state = match map.get(MENU_STATE) {
            Some(state) => MenuState::from_str(state).unwrap(),
            None => MenuState::default(),
        };
        Ok(state)
    }
}

pub fn open_menu() {
    let (_, set_state) = create_query_signal::<MenuState>(MENU_STATE);
    set_state.set(Some(MenuState::Open));
}

pub fn close_menu() {
    let (_, set_state) = create_query_signal::<MenuState>(MENU_STATE);
    set_state.set(Some(MenuState::Closed));
}

pub fn is_menu_open() -> bool {
    let menu = use_query::<MenuState>();
    if let Ok(state) = menu.get() {
        return state == MenuState::Open;
    }

    false
}

pub fn use_menu_state() -> MenuState {
    let menu = use_query::<MenuState>();
    if let Ok(state) = menu.get() {
        return state;
    }

    MenuState::Closed
}

pub fn use_menu_state_signal() -> (Memo<Option<MenuState>>, SignalSetter<Option<MenuState>>) {
    let (state, set_state) = create_query_signal::<MenuState>(MENU_STATE);
    (state, set_state)
}

pub fn use_menu_read_signal() -> Memo<Option<MenuState>> {
    let (state, _) = create_query_signal::<MenuState>(MENU_STATE);
    state
}
