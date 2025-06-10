use anyhow::Error;
use gpui::{prelude::*, App, AppContext, Entity, Global};

use crate::assistant::Prompt;
use crate::storage::{Storage, StorageKey};

use super::error_state::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SettingsTabType {
    General,
    Assistant,
    Hotkeys,
    About,
}

#[derive(Debug)]
pub struct SettingsState {
    pub active_tab: SettingsTabType,
    pub error: Option<Error>,
    pub prompts: Vec<Prompt>,
}

impl ErrorState for SettingsState {
    fn get_error(&self) -> Option<&Error> {
        self.error.as_ref()
    }
}

#[derive(Clone)]
pub struct SettingsStateController {
    pub state: Entity<SettingsState>,
}

impl Global for SettingsStateController {}

impl ErrorStateController for SettingsStateController {
    fn set_error(&self, cx: &mut App, error: Option<Error>) {
        self.state.update(cx, |state, cx| {
            state.error = error;
            cx.notify();
        });
    }
}

impl SettingsStateController {
    pub fn init(cx: &mut App) {
        let storage = cx.global_mut::<Storage>();

        storage.subscribe(|key, value, cx| match key {
            StorageKey::Prompts => {
                let prompts = serde_json::from_str::<Vec<Prompt>>(&value)
                    .ok()
                    .unwrap_or(vec![]);

                set_prompts(cx, prompts);
            }
            _ => {}
        });

        let prompts = storage
            .get(StorageKey::Prompts)
            .and_then(|value| serde_json::from_str::<Vec<Prompt>>(&value).ok())
            .unwrap_or(vec![]);

        let state: Entity<SettingsState> = cx.new(|_cx| SettingsState {
            active_tab: SettingsTabType::Assistant,
            error: None,
            prompts,
        });

        let settings_state = SettingsStateController { state };

        cx.set_global(settings_state);
    }

    pub fn update(f: impl FnOnce(&mut Self, &mut App), cx: &mut App) {
        if !cx.has_global::<Self>() {
            return;
        }

        cx.update_global::<Self, _>(|this, cx| {
            f(this, cx);
        })
    }

    // pub fn update_async(f: impl FnOnce(&mut Self, &mut App), cx: &mut AsyncApp) {
    //     let _ = cx.update_global::<Self, _>(|this, cx| {
    //         f(this, cx);
    //     });
    // }

    pub fn set_active_tab(&self, cx: &mut App, tab: SettingsTabType) {
        self.state.update(cx, |state, cx| {
            state.active_tab = tab;
            cx.notify();
        });
    }

    pub fn set_prompts(&self, cx: &mut App, prompts: Vec<Prompt>) {
        self.state.update(cx, |state, cx| {
            state.prompts = prompts;
            cx.notify();
        });
    }
}

pub fn set_active_tab(cx: &mut App, tab: SettingsTabType) {
    SettingsStateController::update(|this, cx| this.set_active_tab(cx, tab), cx);
}

pub fn set_error(cx: &mut App, error: Option<Error>) {
    SettingsStateController::update(|this, cx| this.set_error(cx, error), cx);
}

pub fn set_prompts(cx: &mut App, prompts: Vec<Prompt>) {
    SettingsStateController::update(|this, cx| this.set_prompts(cx, prompts), cx);
}
