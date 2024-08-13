mod game;
mod ui;
mod view;

use cursive::traits::With;

fn main() {
    let mut siv = cursive::default();
    // Set a theme (nord) (I copy and pasted this from the git repo and changed some colors)
    siv.set_theme(cursive::theme::Theme {
        shadow: false,
        borders: cursive::theme::BorderStyle::Simple,
        palette: cursive::theme::Palette::retro().with(|palette| {
            {
                // First, override some colors from the base palette.
                use cursive::style::Color::TerminalDefault;
                use cursive::style::PaletteColor::*;

                palette[Background] = TerminalDefault;
                palette[View] = cursive::theme::Color::Rgb(46, 52, 64);
                palette[Primary] = cursive::theme::Color::Rgb(213, 219, 230);
                palette[TitlePrimary] = cursive::theme::Color::Rgb(126, 158, 189);
                palette[Secondary] = cursive::theme::Color::Rgb(126, 158, 189);
                palette[Highlight] = cursive::theme::Color::Rgb(190, 96, 105);
            }

            {
                // Then override some styles.
                use cursive::style::Effect::*;
                use cursive::style::PaletteStyle::*;
                use cursive::style::Style;
                palette[Highlight] = Style::from(palette[Highlight]).combine(Bold);
                palette[EditableTextCursor] = Style::secondary().combine(Reverse).combine(Underline)
            }
        }),
    });
    // show the main menu
    ui::show_menu_main(&mut siv);
    // Set the refresh rate to 30 FPS and run
    siv.set_autorefresh(true);
    //siv.set_fps(30);
    siv.run();
}
