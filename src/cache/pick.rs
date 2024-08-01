mod state;
mod ui;

use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    hint::unreachable_unchecked,
    io::stdout,
    rc::Rc,
};

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use itertools::Itertools;
use parking_lot::Mutex;
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};
use state::{HistoryEntry, State};

use crate::{
    cache::Cache,
    template::{Category, Template},
    CACHE,
};

use super::{Folder, Item};

pub fn pick_template() -> anyhow::Result<Option<Template>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let state = Rc::new(Mutex::new(State {
        // matching_templates: templates.iter().map(|t| (t.clone(), vec![])).collect(),
        search_term: String::new(),
        list_state: {
            let mut state = ListState::default();
            state.select(Some(0));
            state
        },
        current_folder: CACHE.root.clone(),
        history: Vec::new(),
    }));

    let selected = loop {
        terminal.draw(ui::ui(state.clone()))?;
        let (should_quit, selected) = handle_events(&state)?;

        if should_quit {
            break selected;
        }
    };

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(selected)
}

fn handle_events(state: &Mutex<State>) -> anyhow::Result<(bool, Option<Template>)> {
    let mut state = state.lock();

    // state.matching_templates = templates
    //     .iter()
    //     .filter_map(|t| {
    //         SkimMatcherV2::default()
    //             .fuzzy_indices(t.name(), state.search_term.as_str())
    //             .map(|(score, indices)| (t, score, indices))
    //     })
    //     .sorted_by(
    //         |(a, score_a, _), (b, score_b, _)| match score_b.cmp(score_a) {
    //             Ordering::Equal => a.cmp(b),
    //             ordering => ordering,
    //         },
    //     )
    //     .map(|(t, _, indices)| (t.clone(), indices))
    //     .collect();

    if let Event::Key(key) = event::read()? {
        Ok(state.handle_key_event(key))
    } else {
        Ok((false, None))
    }
}
