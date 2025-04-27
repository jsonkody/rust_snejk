use macroquad::audio::{PlaySoundParams, load_sound, play_sound, play_sound_once};
use macroquad::prelude::*;
use std::collections::{LinkedList, VecDeque};

const SQUARES: i16 = 40;
const MAP_SIZE: f32 = 600.0;
const DOT_SIZE: f32 = MAP_SIZE / SQUARES as f32;

type Point = (i16, i16);

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    dir: Point,
    input_queue: VecDeque<Point>,
}

struct FloatingText {
    x: f32,
    y: f32,
    value: i32,
    life: f32,
}

fn new_snake() -> Snake {
    Snake {
        head: (1, 1),
        dir: (1, 0),
        body: {
            let mut b = LinkedList::new();
            b.push_back((0, 1));
            b
        },
        input_queue: VecDeque::new(),
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        window_width: 600,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // benchmark
    // let mut frame_times: Vec<f64> = Vec::new();
    // let mut last_log_time = get_time();

    // 🎵 Load všech assetů před začátkem hry
    let font = load_ttf_font("assets/2p.ttf").await.unwrap();
    let music = load_sound("assets/astrolander.ogg").await.unwrap();
    let point_sound = load_sound("assets/sfx/point.ogg").await.unwrap();

    play_sound(
        &music,
        PlaySoundParams {
            looped: true,
            volume: 0.5,
        },
    );

    let mut snake = new_snake();
    let mut fruit = generate_safe_fruit(&snake);
    let mut score = 0;
    let mut speed_factor = 3.0f32;
    let mut point_value = compute_point_value(speed_factor);
    let mut current_point_value = point_value;
    let mut accumulator = 0.0f32;
    let mut game_over = false;
    let mut floating_texts: Vec<FloatingText> = vec![];

    let up = (0, -1);
    let down = (0, 1);
    let right = (1, 0);
    let left = (-1, 0);

    loop {
        // benchmark
        // let frame_start = get_time();

        let frame_time = get_frame_time();
        accumulator += frame_time;

        if !game_over {
            if is_key_pressed(KeyCode::Right) {
                snake.input_queue.push_back(right);
            }
            if is_key_pressed(KeyCode::Left) {
                snake.input_queue.push_back(left);
            }
            if is_key_pressed(KeyCode::Up) {
                snake.input_queue.push_back(up);
            }
            if is_key_pressed(KeyCode::Down) {
                snake.input_queue.push_back(down);
            }

            let speed = compute_speed(speed_factor);

            while accumulator > speed {
                accumulator -= speed;

                while let Some(new_dir) = snake.input_queue.pop_front() {
                    if (snake.dir.0 + new_dir.0 != 0) || (snake.dir.1 + new_dir.1 != 0) {
                        snake.dir = new_dir;
                        break;
                    }
                }

                snake.body.push_front(snake.head);

                snake.head = (
                    (snake.head.0 + snake.dir.0 + SQUARES) % SQUARES,
                    (snake.head.1 + snake.dir.1 + SQUARES) % SQUARES,
                );

                if snake.head == fruit {
                    floating_texts.push(FloatingText {
                        x: fruit.0 as f32 * DOT_SIZE + DOT_SIZE / 2.0,
                        y: fruit.1 as f32 * DOT_SIZE,
                        value: current_point_value,
                        life: 1.0,
                    });

                    fruit = generate_safe_fruit(&snake);
                    score += current_point_value;
                    speed_factor += 1.0;
                    point_value = compute_point_value(speed_factor);
                    current_point_value = point_value;

                    play_sound_once(&point_sound);
                } else {
                    snake.body.pop_back();
                }

                if current_point_value > 1 {
                    current_point_value -= 1;
                }

                for (x, y) in &snake.body {
                    if *x == snake.head.0 && *y == snake.head.1 {
                        game_over = true;
                    }
                }
            }
        }

        clear_background(Color::from_rgba(8, 5, 15, 255)); // #08050f

        if !game_over {
            let offset_x = 0.0;
            let offset_y = 0.0;

            draw_rectangle(offset_x, offset_y, MAP_SIZE, MAP_SIZE, BLACK);

            // tělo hada
            for (x, y) in &snake.body {
                draw_rectangle(
                    offset_x + *x as f32 * DOT_SIZE,
                    offset_y + *y as f32 * DOT_SIZE,
                    DOT_SIZE,
                    DOT_SIZE,
                    Color::new(0.0, 1.0, 0.57, 1.0), // zelená
                );
            }

            // hlava hada
            draw_rectangle(
                offset_x + snake.head.0 as f32 * DOT_SIZE,
                offset_y + snake.head.1 as f32 * DOT_SIZE,
                DOT_SIZE,
                DOT_SIZE,
                Color::new(0.0, 1.0, 0.57, 1.0),
            );

            // jídlo (měnící saturaci)
            let saturation = current_point_value as f32 / point_value as f32;
            draw_rectangle(
                offset_x + fruit.0 as f32 * DOT_SIZE,
                offset_y + fruit.1 as f32 * DOT_SIZE,
                DOT_SIZE,
                DOT_SIZE,
                hsl_to_color(312.0, saturation, 0.5),
            );

            // skóre text (fialový + průhledný)
            let text = format!("{}", score);
            let params = TextParams {
                font: Some(&font),
                font_size: 24,
                color: Color::from_rgba(148, 94, 255, 128),
                ..Default::default()
            };

            draw_text_ex(
                &text,
                MAP_SIZE - 20.0 - measure_text(&text, Some(&font), 24, 1.0).width,
                30.0,
                params,
            );

            // floating +points
            floating_texts.iter_mut().for_each(|t| {
                let alpha = t.life;
                let float_text = format!("+{}", t.value);
                draw_text_ex(
                    &float_text,
                    t.x - measure_text(&float_text, Some(&font), 18, 1.0).width / 2.0,
                    t.y,
                    TextParams {
                        font: Some(&font),
                        font_size: 18,
                        color: Color::new(1.0, 1.0, 1.0, alpha),
                        ..Default::default()
                    },
                );
                t.y -= 30.0 * get_frame_time();
                t.life -= 1.5 * get_frame_time();
            });

            floating_texts.retain(|t| t.life > 0.0);
        } else {
            let text = "Ještě jednou?";
            let font_size = 20.0;
            let text_size = measure_text(text, Some(&font), font_size as u16, 1.0);

            draw_text_ex(
                text,
                screen_width() / 2.0 - text_size.width / 2.0,
                screen_height() / 2.0,
                TextParams {
                    font: Some(&font),
                    font_size: font_size as u16,
                    color: WHITE,
                    ..Default::default()
                },
            );

            let score_text = format!("{}", score);
            let score_font_size = 28.0;
            let score_text_size =
                measure_text(&score_text, Some(&font), score_font_size as u16, 1.0);

            draw_text_ex(
                &score_text,
                screen_width() / 2.0 - score_text_size.width / 2.0,
                screen_height() / 2.0 + 50.0,
                TextParams {
                    font: Some(&font),
                    font_size: score_font_size as u16,
                    color: YELLOW,
                    ..Default::default()
                },
            );

            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                snake = new_snake();
                fruit = generate_safe_fruit(&snake);
                score = 0;
                speed_factor = 3.0;
                point_value = compute_point_value(speed_factor);
                current_point_value = point_value;
                accumulator = 0.0;
                floating_texts.clear();
                game_over = false;
            }
        }

        // benchmark
        // let frame_end = get_time();
        // frame_times.push(frame_end - frame_start);

        // if frame_end - last_log_time >= 10.0 {
        //     let sum: f64 = frame_times.iter().sum();
        //     let avg = sum / frame_times.len() as f64;
        //     println!(
        //         "📊 Průměrný čas jednoho frame: {:.6} sekund ({:.1} FPS)",
        //         avg,
        //         1.0 / avg
        //     );
        //     frame_times.clear();
        //     last_log_time = frame_end;
        // }

        next_frame().await;
    }
}

fn compute_speed(speed_factor: f32) -> f32 {
    // let ln_factor = speed_factor.ln();
    // (140.0 / (ln_factor * 1.2) + 14.0) / 1000.0
    return 0.02;
}

fn compute_point_value(speed_factor: f32) -> i32 {
    (speed_factor.ln().floor() as i32) * 100
}

fn hsl_to_color(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match h as i32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        300..=359 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };
    Color::new(r + m, g + m, b + m, 1.0)
}

fn generate_safe_fruit(snake: &Snake) -> Point {
    loop {
        let fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
        if fruit == snake.head || snake.body.iter().any(|&p| p == fruit) {
            continue;
        }
        return fruit;
    }
}
