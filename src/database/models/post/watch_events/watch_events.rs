use diesel::prelude::*;
use diesel::MysqlConnection;
use crate::database::schema::watch_events::dsl::{self, watch_events as table};
use crate::lib::Consequence;
use super::{WatchEventEntity, WatchEventKind};
use crate::lib::consequence::WatchEventError;


// Graph adjacency matrix
// Void, Submit, Accept, Refuse, Progress, Over
const WATCH_EVENT_AM: [[bool; 6]; 6] = [
    [false, true, false, false, false, false], // Void
    [false, false, true, true, false, false], // Submit
    [false, false, false, false, true, true], // Accept
    [false, false, false, false, false, false], // Refuse
    [false, false, false, false, true, true], // Progress
    [false, false, false, false, false, false], // Over
];

impl WatchEventEntity {

    pub fn by_post_id(conn: &MysqlConnection, post_id: &u32) -> Consequence<Vec<WatchEventEntity>> {
        table.filter(dsl::post_id.eq(post_id)).load(conn).map(Ok)?
    }

    pub fn validate_transition(from: Option<&WatchEventKind>, to: &WatchEventKind) -> Consequence<()> {
        let row = from.map(|wek| wek.into()).unwrap_or(0);
        let column: u8 = to.into();
        if !WATCH_EVENT_AM[row as usize][column as usize] {
            Err(WatchEventError::InvalidWatchTransition)?
        }
        Ok(())
    }

    pub fn sort(events: &mut Vec<Self>) {
        events.sort_unstable_by(|a, b| a.event.cmp(&b.event));
    }

    pub fn last(events: &mut Vec<Self>) -> Option<Self> {
        Self::sort(events);
        events[..].last().cloned()
    }

}
