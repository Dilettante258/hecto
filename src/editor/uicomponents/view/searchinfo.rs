
use crate::editor::Line;
use crate::prelude::*;
use super::Location;
pub struct SearchInfo {
    pub prev_location: Location,
    pub prev_scroll_offset: Position,
    pub query: Option<Line>
}