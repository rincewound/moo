use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Block,
};

use crate::app::ApplicationState;

pub fn render_mode_header(
    frame: &mut ratatui::Frame,
    dest: ratatui::layout::Rect,
    mode: &str,
    app_state: &crate::app::ApplicationState,
) {
    // renders a box on top with the current mode's name,
    // the active buffer names and the current line number

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(frame.area());

    // as output: Select the first four buffers starting at the active one
    // and show their name and modified flag. If there are more buffers, add
    // an ellipsis, if fewer, just show as many as available

    const MAX_BUFFERS_TO_SHOW: usize = 6;

    let num_to_skip = if app_state.buffers.len() > MAX_BUFFERS_TO_SHOW {
        // if more than 4 buffers, make sure to always display
        // at least 4 buffers.
        if app_state.current_buffer > app_state.buffers.len() - MAX_BUFFERS_TO_SHOW {
            app_state.buffers.len() - MAX_BUFFERS_TO_SHOW
        } else {
            app_state.current_buffer
        }
    } else {
        0
    };

    let buffer_names: Vec<String> = app_state
        .buffers
        .iter()
        .skip(num_to_skip)
        .take(MAX_BUFFERS_TO_SHOW)
        .map(|buffer| {
            let output_string = format!(
                "{}{}|",
                if buffer.modified { "● " } else { "" },
                buffer.name
            );
            output_string
        })
        .collect();

    let mut pos = 1;
    for (id, buffer_name) in buffer_names.iter().enumerate() {
        let len = buffer_name.chars().count() as u16;
        let mut the_widget = ratatui::widgets::Paragraph::new(buffer_name.clone())
            .alignment(ratatui::layout::Alignment::Left);
        if id == app_state.current_buffer {
            the_widget = the_widget
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
        }

        frame.render_widget(
            the_widget,
            ratatui::layout::Rect::new(pos, 1 as u16, len, 1),
        );
        pos += len + 1;
    }

    if buffer_names.len() > 4 {
        frame.render_widget(
            ratatui::widgets::Paragraph::new("...").alignment(ratatui::layout::Alignment::Left),
            ratatui::layout::Rect::new(pos, 1, 3, 1),
        );
    }

    // show current mode name:
    let output_string = format!("{}", mode);
    let len = output_string.len() as u16;
    frame.render_widget(
        ratatui::widgets::Paragraph::new(output_string)
            .alignment(ratatui::layout::Alignment::Right),
        ratatui::layout::Rect::new(dest.width - (len + 1), 1, len, 1),
    );

    frame.render_widget(
        ratatui::widgets::Paragraph::new("")
            .alignment(ratatui::layout::Alignment::Right)
            .block(Block::bordered()),
        dest,
    );
}

pub fn rotate_buffer(app_state: &mut ApplicationState, direction: i32) {
    let next_buffer_id = app_state.current_buffer as i32 + direction;
    app_state.current_buffer = (next_buffer_id % app_state.buffers.len() as i32) as usize;
}
