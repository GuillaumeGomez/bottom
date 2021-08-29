use std::collections::HashMap;

use crossterm::event::{KeyEvent, MouseEvent};
use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders},
    Frame,
};

use crate::{
    app::{
        data_farmer::DataCollection, data_harvester::temperature::TemperatureType,
        event::EventResult, sort_text_table::SortableColumn,
    },
    canvas::Painter,
    data_conversion::convert_temp_row,
};

use super::{
    text_table::TextTableData, AppScrollWidgetState, CanvasTableWidthState, Component,
    SortableTextTable, Widget,
};

pub struct TempWidgetState {
    pub scroll_state: AppScrollWidgetState,
    pub table_width_state: CanvasTableWidthState,
}

impl TempWidgetState {
    pub fn init() -> Self {
        TempWidgetState {
            scroll_state: AppScrollWidgetState::default(),
            table_width_state: CanvasTableWidthState::default(),
        }
    }
}

#[derive(Default)]
pub struct TempState {
    pub widget_states: HashMap<u64, TempWidgetState>,
}

impl TempState {
    pub fn init(widget_states: HashMap<u64, TempWidgetState>) -> Self {
        TempState { widget_states }
    }

    pub fn get_mut_widget_state(&mut self, widget_id: u64) -> Option<&mut TempWidgetState> {
        self.widget_states.get_mut(&widget_id)
    }

    pub fn get_widget_state(&self, widget_id: u64) -> Option<&TempWidgetState> {
        self.widget_states.get(&widget_id)
    }
}

/// A table displaying disk data.  Essentially a wrapper around a [`TextTable`].
pub struct TempTable {
    table: SortableTextTable,
    bounds: Rect,
    display_data: TextTableData,
    temp_type: TemperatureType,
}

impl Default for TempTable {
    fn default() -> Self {
        let table = SortableTextTable::new(vec![
            SortableColumn::new_flex("Sensor".into(), None, false, 0.8),
            SortableColumn::new_hard("Temp".into(), None, false, Some(5)),
        ])
        .default_ltr(false);

        Self {
            table,
            bounds: Rect::default(),
            display_data: Default::default(),
            temp_type: TemperatureType::default(),
        }
    }
}

impl TempTable {
    /// Sets the [`TemperatureType`] for the [`TempTable`].
    pub fn set_temp_type(mut self, temp_type: TemperatureType) -> Self {
        self.temp_type = temp_type;
        self
    }
}

impl Component for TempTable {
    fn handle_key_event(&mut self, event: KeyEvent) -> EventResult {
        self.table.handle_key_event(event)
    }

    fn handle_mouse_event(&mut self, event: MouseEvent) -> EventResult {
        self.table.handle_mouse_event(event)
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, new_bounds: Rect) {
        self.bounds = new_bounds;
    }
}

impl Widget for TempTable {
    fn get_pretty_name(&self) -> &'static str {
        "Temperature"
    }

    fn draw<B: Backend>(
        &mut self, painter: &Painter, f: &mut Frame<'_, B>, area: Rect, selected: bool,
    ) {
        let block = Block::default()
            .border_style(if selected {
                painter.colours.highlighted_border_style
            } else {
                painter.colours.border_style
            })
            .borders(Borders::ALL); // TODO: Also do the scrolling indicator!

        self.table
            .table
            .draw_tui_table(painter, f, &self.display_data, block, area, selected);
    }

    fn update_data(&mut self, data_collection: &DataCollection) {
        self.display_data = convert_temp_row(data_collection, &self.temp_type);
    }
}