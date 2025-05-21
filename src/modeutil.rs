use ratatui::layout::{Constraint, Direction, Layout};

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

    let num_to_skip = if app_state.buffers.len() > 4 {
        // if more than 4 buffers, make sure to always display
        // at least 4 buffers.
        if app_state.current_buffer > app_state.buffers.len() - 4 {
            app_state.buffers.len() - 4
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
        .take(4)
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
            ratatui::layout::Rect::new(pos, 4, 3, 1),
        );
    }

    // show current mode name:
    let output_string = format!("{}", mode);
    let len = output_string.len() as u16;
    frame.render_widget(
        ratatui::widgets::Paragraph::new(output_string)
            .alignment(ratatui::layout::Alignment::Right),
        ratatui::layout::Rect::new(dest.width - len, 0, len, 1),
    );
}
