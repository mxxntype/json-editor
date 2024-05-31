use crate::app::{ActiveScreen, EditingMode, JsonEditor};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;

pub fn ui(f: &mut Frame, app: &JsonEditor) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Create New Json",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);
    let mut list_items = Vec::<ListItem>::new();

    for key in app.pairs.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let list = List::new(list_items);

    f.render_widget(list, chunks[1]);
    let current_navigation_text = vec![
        // The first half of the text.
        match app.current_screen {
            ActiveScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            ActiveScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            ActiveScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .clone(),
        // A white divider bar to separate the two sections.
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing.
        {
            match &app.editing_mode {
                Some(EditingMode::Key) => {
                    Span::styled("Editing Json Key", Style::default().fg(Color::Green))
                }
                Some(EditingMode::Value) => {
                    Span::styled("Editing Json Value", Style::default().fg(Color::LightGreen))
                }
                None => Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray)),
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            ActiveScreen::Main | ActiveScreen::Exiting => Span::styled(
                "(q) to quit / (e) to make new pair",
                Style::default().fg(Color::Red),
            ),
            ActiveScreen::Editing => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);

    if let Some(editing) = &app.editing_mode {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, f.size());
        f.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut key_block = Block::default().title("Key").borders(Borders::ALL);
        let mut value_block = Block::default().title("Value").borders(Borders::ALL);
        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            EditingMode::Key => key_block = key_block.style(active_style),
            EditingMode::Value => value_block = value_block.style(active_style),
        };

        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
        f.render_widget(key_text, popup_chunks[0]);

        let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
        f.render_widget(value_text, popup_chunks[1]);
    }

    if app.current_screen == ActiveScreen::Exiting {
        f.render_widget(Clear, f.size()); // Clears the entire screen and anything already drawn.
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(Color::Red),
        );

        // NOTE: The `trim: false` will stop the text from being cut off when over the edge of the block.
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, f.size());
        f.render_widget(exit_paragraph, area);
    }
}

/// Helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, container: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces.
    let padding_y = (100 - percent_y) / 2;
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(padding_y),
            Constraint::Percentage(percent_y),
            Constraint::Percentage(padding_y),
        ])
        .split(container);

    // Cut the middle vertical piece into three width-wise pieces.
    let padding_x = (100 - percent_x) / 2;
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(padding_x),
            Constraint::Percentage(percent_x),
            Constraint::Percentage(padding_x),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk.
}
