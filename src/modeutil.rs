use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
};

use ratatui::{style::Stylize, text::Line};

use crate::app::ApplicationState;

pub fn render_mode_header(
    frame: &mut ratatui::Frame,
    dest: ratatui::layout::Rect,
    mode: &str,
    app_state: &crate::app::ApplicationState,
) {
    // renders a box on top with the current mode's name,
    // the active buffer names and the current line number

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

pub fn render(
    mode_name: &str,
    frame: &mut ratatui::Frame,
    app_state: &crate::app::ApplicationState,
) {
    // ToDo: This should be generalized a bit for all modes!
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(frame.area());

    let buffer = &app_state.buffers[app_state.current_buffer];

    // show buffer name + modified flag:
    render_mode_header(frame, layout[0], mode_name, app_state);

    for (id, line) in buffer
        .buffer
        .lines
        .iter()
        .skip(buffer.scroll_offset)
        .enumerate()
    {
        let line_width = if line.len() < frame.area().width as usize - 1 {
            line.len() as u16
        } else {
            frame.area().width as u16 - 1
        };

        let line_as_string = line.iter().map(|c| c.to_string()).collect::<String>();

        frame.render_widget(
            ratatui::widgets::Paragraph::new(line_as_string)
                .alignment(ratatui::layout::Alignment::Left),
            ratatui::layout::Rect::new(0, 3 + id as u16, line_width, 1),
        );

        // render cursor:
        let effective_line = id + buffer.scroll_offset;
        if buffer.cursor_line == effective_line {
            // get character under cursor
            let char = line.iter().nth(buffer.cursor_position);
            let cursor_char = if let Some(c) = char { c.clone() } else { '_' };

            let mut cursor = cursor_char.to_string().rapid_blink();
            if char.is_some() {
                cursor = cursor.underlined();
            }

            let the_cusor = Line::from(vec![cursor]);
            if buffer.cursor_position < frame.area().width as usize {
                frame.render_widget(
                    ratatui::widgets::Paragraph::new(the_cusor)
                        .alignment(ratatui::layout::Alignment::Left),
                    ratatui::layout::Rect::new(
                        buffer.cursor_position as u16,
                        (buffer.cursor_line - buffer.scroll_offset + 3) as u16,
                        1,
                        1,
                    ),
                );
            }
        }
    }
}
