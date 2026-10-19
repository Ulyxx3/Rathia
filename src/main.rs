// src/main.rs
mod map;
mod audio;
mod game;
mod ui;
mod app;

use std::io::stdout;
use std::time::Duration;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurer le hook de panique pour restaurer le terminal si nécessaire
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture, Show);
        original_hook(panic_info);
    }));

    // Initialisation du terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    // Boucle de jeu haute fréquence (~120 FPS pour un rythme ultra réactif)
    while app.running {
        // Rendu de l'écran courant
        app.render(&mut terminal)?;

        // Événements Crossterm (poll 8ms pour ne pas saturer le CPU tout en étant fluide)
        if event::poll(Duration::from_millis(8))? {
            match event::read()? {
                Event::Key(key) => {
                    app.handle_key_event(key);
                }
                Event::Mouse(mouse) => {
                    app.handle_mouse_event(mouse);
                }
                _ => {}
            }
        }

        // Mise à jour de la logique de jeu (audio sync, hits, health)
        app.update();
    }

    // Nettoyage et restauration du terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        Show
    )?;
    terminal.show_cursor()?;

    println!("Merci d'avoir joué à Rathia !");
    Ok(())
}
