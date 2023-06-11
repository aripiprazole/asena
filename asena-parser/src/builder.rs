use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    vec::IntoIter,
};

use super::event::Event;

#[derive(Clone)]
pub struct EventBuilder {
    events: Vec<Event>,
}

impl EventBuilder {
    pub fn new(events: Vec<Event>) -> Self {
        Self { events }
    }

    pub fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tab_index = 0;
        for event in self.events.iter() {
            write!(f, "{:w$}", "", w = tab_index)?;
            writeln!(f, "{event:?}")?;
            if let Event::Open(..) = event {
                tab_index += 2;
            }
            if let Event::Close = event {
                tab_index -= 2;
            }
        }
        Ok(())
    }
}

impl IntoIterator for EventBuilder {
    type Item = Event;

    type IntoIter = IntoIter<Event>;

    fn into_iter(self) -> Self::IntoIter {
        self.events.into_iter()
    }
}

impl DerefMut for EventBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.events
    }
}

impl Deref for EventBuilder {
    type Target = Vec<Event>;

    fn deref(&self) -> &Self::Target {
        &self.events
    }
}

impl Debug for EventBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.render(f)
    }
}

impl Display for EventBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.render(f)
    }
}
