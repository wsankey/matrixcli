use crossterm::{
    cursor,
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crossterm::cursor::MoveTo;
use rand::{Rng, thread_rng};
use std::{io::stdout, thread, time::Duration};
use rand::distributions::Alphanumeric;


// Define a struct to represent a text stream
struct TextStream {
    characters: Vec<char>,
    position: usize, // Vertical position
    x: u16,          // Horizontal position (column)
    speed: usize,    // Speed of the stream
    color: Color, // Add this field
}


// Main function - Entry point of the program
fn main() -> crossterm::Result<()> {
    setup_terminal()?;
    main_loop()
}

// Sets up the terminal environment
fn setup_terminal() -> crossterm::Result<()> {
    let mut stdout = std::io::stdout();
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::Hide)?;
    terminal::enable_raw_mode()?;
    Ok(())
}


fn main_loop() -> crossterm::Result<()> {
    let mut running = true;

    // Get terminal dimensions
    let (width, _) = terminal::size()?;
    
    // Initialize streams for each column
    let mut streams = (0..width).map(|x| {
        // Randomly generate characters for each stream
        let mut rng = thread_rng();
        let characters: Vec<char> = (0..10)  // Example length of a stream
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect();

        TextStream {
            characters,
            position: 0,
            x,              // Set the x-coordinate for the stream
            speed: rng.gen_range(1..=3), // Random speed
            color: Color::Green,
        }
    }).collect::<Vec<TextStream>>();

    while running {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('s') => running = !running,
                    KeyCode::Char('q') => break,
                    KeyCode::Up => increase_speed(),
                    KeyCode::Down => decrease_speed(),
                    _ => (),
                }
            }
        }

        if running {
            update_and_render(&mut streams)?;
            thread::sleep(Duration::from_millis(50)); // Frame rate control
        }
    }

    Ok(())
}


// Increases the text scrolling speed
fn increase_speed() {
    // TODO: Implement the logic to increase speed
}

// Decreases the text scrolling speed
fn decrease_speed() {
    // TODO: Implement the logic to decrease speed
}

// Updates and renders the text effect
fn update_and_render(streams: &mut [TextStream]) -> crossterm::Result<()> {
    let mut stdout = stdout();

    // Clear the screen before rendering the new frame
    stdout.execute(terminal::Clear(ClearType::All))?;

    for stream in streams.iter_mut() {
        // Update the vertical position of each stream
        stream.position = (stream.position + stream.speed) % terminal::size()?.1 as usize;

        // Render each character in the stream
        for (i, &ch) in stream.characters.iter().enumerate() {
            // Calculate the vertical position for each character
            let y = (stream.position + i) % terminal::size()?.1 as usize;

            // Optionally set color here (e.g., for fading effect)
            stdout.execute(SetForegroundColor(stream.color))?;

            // Move to the position and print the character
            execute!(stdout, MoveTo(stream.x, y as u16), Print(ch))?;
        }
    }

    // Wait a bit before the next update to control the speed of the animation
    thread::sleep(Duration::from_millis(50));

    Ok(())
}

// Generates a random Matrix character
fn random_matrix_char<R: Rng + ?Sized>(rng: &mut R) -> char {
    rng.sample(Alphanumeric) as char
}


// Creates a text stream for the Matrix effect
fn create_text_stream<R: Rng + ?Sized>(rng: &mut R, length: usize) -> Vec<char> {
    (0..length).map(|_| random_matrix_char(rng)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_generation() {
        let mut rng = rand::thread_rng();
        let char = random_matrix_char(&mut rng);
        assert!(char.is_alphanumeric());
    }
}

