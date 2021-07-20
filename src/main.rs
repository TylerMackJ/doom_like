mod map;
mod player;

use std::io::{Write,
    stdout
};
use crossterm::{
    cursor,
    queue,
    execute,
    style::{
        SetBackgroundColor,
        Color,
        Print
    },
    terminal,
    event::{
        KeyCode,
        KeyModifiers,
        read,
        poll,
        Event
    }
};
use map::Map;
use player::Player;
use std::time::{Duration,
    Instant
};
use std::f64::consts::PI;

fn main() {
    let mut stdout = stdout();
    let mut map = Map::new(15, 15, ' ');
    let mut player = Player::new((7.0, 7.0), PI / 4.0, PI / 3.0, 15.0);

    for x in 0..map.width {
        map[(x, 0)] = '#';
        let height: usize = map.height;
        map[(x, height - 1)] = '#';
    }
    for y in 0..map.height {
        map[(0, y)] = '#';
        let width: usize = map.width;
        map[(width - 1, y)] = '#';
    }

    map[(5, 5)] = '#';

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide).unwrap();
    terminal::enable_raw_mode().unwrap();

    let mut screen_space = terminal::size().unwrap();
    let mut last_frame: Vec<Vec<Color>> = vec![vec![Color::Rgb{r: 0, g: 0, b: 0}; screen_space.1 as usize]; screen_space.0 as usize];

    for x in 0..screen_space.0 {
        for y in 0..screen_space.1 {
            queue!(stdout, cursor::MoveTo(x, y), SetBackgroundColor(Color::Rgb{r: 0, g: 0, b: 0}), Print(" ".to_string())).unwrap();
        }
    }
    stdout.flush().unwrap();

    'game: loop {
        let start_time = Instant::now();

        for x in 0..screen_space.0 {
            // Create ray for each column and get distance to wall
            let mut ray: (f64, f64) = player.position();
            let mut angle: f64 = player.direction() + (player.horizontal_fov() * (x as f64 / (screen_space.0 - 1) as f64)) - (player.horizontal_fov() / 2.0);
            let mut ray_distance: f64 = 0.0;
            while map[ray] == ' ' && ray_distance < player.view_distance()   {
                ray.0 += angle.cos() * 0.1;
                ray.1 += angle.sin() * 0.1;
                ray_distance += 0.1;
            }

            // Calculate in advance for color calculations later
            let brightness_multiplier: f64 = 1.0 - (ray_distance / player.view_distance());
            let wall_color: Color = Color::Rgb{
                r: (255.0 * brightness_multiplier) as u8,
                g: (128.0 * brightness_multiplier) as u8,
                b: (128.0 * brightness_multiplier) as u8
            };

            for y in 0..screen_space.1 {
                // Calculate wall height based on y row
                angle = (player.vertical_fov() * (y as f64 / (screen_space.1 - 1) as f64)) - (player.vertical_fov() / 2.0);
                let height: f64 = angle.sin() * ray_distance;

                // Draw color
                let draw_color: Color;
                if height < 1.0 && height > -1.0 && ray_distance < player.view_distance() {
                    draw_color = wall_color;
                } else {
                    draw_color = Color::Rgb{r: 0, g: 0, b: 0};
                }
                if last_frame[x as usize][y as usize] != draw_color {
                    queue!(stdout, cursor::MoveTo(x, y), SetBackgroundColor(draw_color), Print(" ".to_string())).unwrap();
                    last_frame[x as usize][y as usize] = draw_color;
                }
            }
        }
        
        // Execute all queued changed
        stdout.flush().unwrap();

        // Poll for events and handle FPS
        let mut frame_duration = start_time.elapsed();
        let frame_time = Duration::from_millis(16);
        while frame_duration < frame_time {
            if poll(frame_time - frame_duration).unwrap() {
                match read().unwrap() {
                    Event::Key(key_event) => {
                        match key_event {
                            e if e.code == KeyCode::Char('c') && e.modifiers == KeyModifiers::CONTROL => break 'game,
                            e if e.code == KeyCode::Char('w') && e.modifiers == KeyModifiers::NONE    => player.move_camera_relative(( 0.05, 0.0)),
                            e if e.code == KeyCode::Char('s') && e.modifiers == KeyModifiers::NONE    => player.move_camera_relative((-0.05, 0.0)),
                            e if e.code == KeyCode::Char('a') && e.modifiers == KeyModifiers::NONE    => player.move_camera_relative(( 0.0, 0.05)),
                            e if e.code == KeyCode::Char('d') && e.modifiers == KeyModifiers::NONE    => player.move_camera_relative(( 0.0,-0.05)),
                            e if e.code == KeyCode::Left      && e.modifiers == KeyModifiers::NONE    => player.turn(-0.1),
                            e if e.code == KeyCode::Right     && e.modifiers == KeyModifiers::NONE    => player.turn( 0.1),
                            _ => {}
                        }
                    },
                    Event::Resize(columns, rows) => screen_space = (columns, rows),
                    _ => {}
                }
                frame_duration = start_time.elapsed();
            }
        }
    }

    // Clean up terminal
    terminal::disable_raw_mode().unwrap();
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen).unwrap();
}
