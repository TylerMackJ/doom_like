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

    'game: loop {
        let start_time = Instant::now();
        let screen_space = terminal::size().unwrap();

        for x in 0..screen_space.0 {
            let mut ray: (f64, f64) = player.position();
            let mut angle: f64 = player.direction() + (player.horizontal_fov() * (x as f64 / (screen_space.0 - 1) as f64)) - (player.horizontal_fov() / 2.0);
            while map[ray] == ' ' {
                ray.0 += angle.cos() * 0.1;
                ray.1 += angle.sin() * 0.1;
            }
            let ray_distance: f64 = ((ray.0 - player.position().0).powf(2.0) + (ray.1 - player.position().1).powf(2.0)).sqrt();

            for y in 0..screen_space.1 {
                angle = (player.vertical_fov() * (y as f64 / (screen_space.1 - 1) as f64)) - (player.vertical_fov() / 2.0);
                let height: f64 = angle.sin() * ray_distance;
                if height < 1.0 && height > -1.0 && ray_distance < player.view_distance() {
                    let brightness_multiplier: f64 = 1.0 - (ray_distance / player.view_distance());
                    queue!(stdout, cursor::MoveTo(x, y), SetBackgroundColor(Color::Rgb{r: (255.0 * brightness_multiplier) as u8, g: (128.0 * brightness_multiplier) as u8, b: (128.0 * brightness_multiplier) as u8}), Print(" ".to_string())).unwrap();
                } else {
                    queue!(stdout, cursor::MoveTo(x, y), SetBackgroundColor(Color::Rgb{r: 0, g: 0, b: 0}), Print(" ".to_string())).unwrap();
                }
            }
        }
        stdout.flush().unwrap();

        let mut frame_duration = start_time.elapsed();

        while frame_duration < Duration::from_millis(32) {
            if poll(Duration::from_millis(32) - frame_duration).unwrap() {
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
                    _ => {}
                }
                frame_duration = start_time.elapsed()
            }
        }

            /*
            let e = term.get_event(Duration::new(0, 0)).unwrap();
            match e {
                Some(Key('\u{3}')) | Some(Key('\u{1a}')) | Some(Key('\u{11}'))  => break 'game,
                Some(Key('w')) => player.move_camera_relative(( 0.05, 0.0)),
                Some(Key('s')) => player.move_camera_relative((-0.05, 0.0)),
                Some(Key('a')) => player.move_camera_relative(( 0.0, 0.05)),
                Some(Key('d')) => player.move_camera_relative(( 0.0,-0.05)),
                Some(Key('\u{1b}')) => {//1b [ C, D, B, A
                    let e1 = term.get_event(Duration::from_secs(0)).unwrap();
                    let e2 = term.get_event(Duration::from_secs(0)).unwrap();
                    match e1 {
                        Some(Key('[')) => {
                            match e2 {
                                Some(Key('C')) => player.turn( 0.1),
                                Some(Key('D')) => player.turn(-0.1),
                                Some(Key('B')) => {}, // Look down,
                                Some(Key('A')) => {}, // Look up,
                                _ => {},
                            }
                        }
                        _ => {},
                    }
                }
                _ => {},
            }
            */
    }
    terminal::disable_raw_mode().unwrap();
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen).unwrap();
}
