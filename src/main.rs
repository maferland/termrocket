use base64::{Engine as _, engine::general_purpose::STANDARD};
use clap::{Parser, Subcommand};
use image::{Rgba, RgbaImage};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// Colors from gitrocket CSS
const BODY_LIGHT: Rgba<u8> = Rgba([209, 28, 64, 255]);
const BODY_DARK: Rgba<u8> = Rgba([167, 23, 50, 255]);
const FIN_COLOR: Rgba<u8> = Rgba([127, 14, 39, 255]);
const FLAME_OUTER: Rgba<u8> = Rgba([244, 82, 36, 255]);
const FLAME_INNER: Rgba<u8> = Rgba([255, 237, 213, 255]);

// Rocket dimensions
const ROCKET_WIDTH: u32 = 256;
const ROCKET_HEIGHT: u32 = 384;

// Animation settings
const FPS: u32 = 60;
const DURATION_SECS: f32 = 2.6;
const IMAGE_ID: u32 = 31415;

#[derive(Parser)]
#[command(name = "termrocket")]
#[command(about = "Animated rocket on git push. Inspired by gitrocket.")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Launch,
    Test,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Launch) | None => launch_rocket(),
        Some(Commands::Test) => test_terminal(),
    }
}

fn is_kitty_terminal() -> bool {
    std::env::var("TERM").map(|t| t.contains("kitty")).unwrap_or(false)
        || std::env::var("KITTY_WINDOW_ID").is_ok()
}

fn get_terminal_size() -> (u32, u32) {
    unsafe {
        let mut size: libc::winsize = std::mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut size) == 0 {
            return (size.ws_col as u32, size.ws_row as u32);
        }
    }
    (80, 24)
}

fn blend_colors(c1: Rgba<u8>, c2: Rgba<u8>, t: f32) -> Rgba<u8> {
    let t = t.clamp(0.0, 1.0);
    Rgba([
        ((1.0 - t) * c1.0[0] as f32 + t * c2.0[0] as f32) as u8,
        ((1.0 - t) * c1.0[1] as f32 + t * c2.0[1] as f32) as u8,
        ((1.0 - t) * c1.0[2] as f32 + t * c2.0[2] as f32) as u8,
        ((1.0 - t) * c1.0[3] as f32 + t * c2.0[3] as f32) as u8,
    ])
}

fn draw_rocket() -> RgbaImage {
    let mut img = RgbaImage::new(ROCKET_WIDTH, ROCKET_HEIGHT);
    let center_x = ROCKET_WIDTH / 2;

    // Rocket body dimensions
    let nose_top = 30;
    let nose_bottom = 90;
    let body_bottom = 210;
    let max_width: i32 = 32;

    // Draw fins FIRST (behind body)
    let fin_start_y = 180;
    let fin_end_y = 225;

    for y in fin_start_y..fin_end_y {
        let t = (y - fin_start_y) as f32 / (fin_end_y - fin_start_y) as f32;
        let inner = (28.0 + t * 8.0) as i32;
        let outer = (28.0 + 25.0 * t.sqrt() * (1.0 - t * 0.4)) as i32;

        if outer > inner {
            for x in inner..=outer {
                // Left fin
                let px_left = center_x as i32 - x;
                if px_left >= 0 {
                    img.put_pixel(px_left as u32, y, FIN_COLOR);
                }
                // Right fin
                let px_right = center_x as i32 + x;
                if px_right < ROCKET_WIDTH as i32 {
                    img.put_pixel(px_right as u32, y, FIN_COLOR);
                }
            }
        }
    }

    // Draw nose cone
    let nose_height = nose_bottom - nose_top;
    for y in nose_top..nose_bottom {
        let t = (y - nose_top) as f32 / nose_height as f32;
        let width = (max_width as f32 * (t * (2.0 - t)).sqrt()) as i32;

        for x in -width..=width {
            let shade = if width > 0 {
                ((x + width) as f32 / (2 * width) as f32) * 0.6
            } else {
                0.3
            };
            let color = blend_colors(BODY_LIGHT, BODY_DARK, shade);
            let px = center_x as i32 + x;
            if px >= 0 && px < ROCKET_WIDTH as i32 {
                img.put_pixel(px as u32, y, color);
            }
        }
    }

    // Draw main body
    for y in nose_bottom..body_bottom {
        let t = (y - nose_bottom) as f32 / (body_bottom - nose_bottom) as f32;
        let width = if t > 0.85 {
            (max_width as f32 * (1.0 - (t - 0.85) * 0.5)) as i32
        } else {
            max_width
        };

        for x in -width..=width {
            let shade = if width > 0 {
                ((x + width) as f32 / (2 * width) as f32) * 0.6
            } else {
                0.3
            };
            let color = blend_colors(BODY_LIGHT, BODY_DARK, shade);
            let px = center_x as i32 + x;
            if px >= 0 && px < ROCKET_WIDTH as i32 {
                img.put_pixel(px as u32, y, color);
            }
        }
    }

    // Draw flame
    draw_flame(&mut img, center_x as i32 - 15, body_bottom as i32 - 3, 30);

    img
}

fn draw_flame(img: &mut RgbaImage, x: i32, y: i32, width: i32) {
    let flame_height = 70;
    let center_x = x + width / 2;

    for dy in 0..flame_height {
        let t = dy as f32 / flame_height as f32;
        let half_width = ((1.0 - t.powf(0.8)) * (width as f32 / 2.0)) as i32;

        for dx in 0..=half_width {
            let inner_t = dx as f32 / half_width.max(1) as f32;
            let color = if inner_t < 0.4 {
                blend_colors(FLAME_INNER, FLAME_OUTER, inner_t * 2.5)
            } else {
                let alpha = ((1.0 - t) * 255.0) as u8;
                Rgba([FLAME_OUTER.0[0], FLAME_OUTER.0[1], FLAME_OUTER.0[2], alpha])
            };

            let px_right = center_x + dx;
            let px_left = center_x - dx;
            let py = y + dy;

            if px_right >= 0 && px_right < img.width() as i32 && py >= 0 && py < img.height() as i32 {
                blend_pixel(img, px_right as u32, py as u32, color);
            }
            if dx > 0 && px_left >= 0 && px_left < img.width() as i32 && py >= 0 && py < img.height() as i32 {
                blend_pixel(img, px_left as u32, py as u32, color);
            }
        }
    }
}

fn blend_pixel(img: &mut RgbaImage, x: u32, y: u32, color: Rgba<u8>) {
    let existing = img.get_pixel(x, y);
    let alpha = color.0[3] as f32 / 255.0;
    let blended = Rgba([
        ((1.0 - alpha) * existing.0[0] as f32 + alpha * color.0[0] as f32) as u8,
        ((1.0 - alpha) * existing.0[1] as f32 + alpha * color.0[1] as f32) as u8,
        ((1.0 - alpha) * existing.0[2] as f32 + alpha * color.0[2] as f32) as u8,
        existing.0[3].max(color.0[3]),
    ]);
    img.put_pixel(x, y, blended);
}

fn encode_png(img: &RgbaImage) -> Vec<u8> {
    use image::ImageEncoder;
    let mut buffer = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
    encoder.write_image(
        img.as_raw(),
        img.width(),
        img.height(),
        image::ExtendedColorType::Rgba8,
    ).expect("Failed to encode PNG");
    buffer
}

fn upload_image(png_data: &[u8]) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    let b64_data = STANDARD.encode(png_data);

    let chunk_size = 4096;
    let chunks: Vec<&str> = b64_data
        .as_bytes()
        .chunks(chunk_size)
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect();

    for (i, chunk) in chunks.iter().enumerate() {
        let m = if i == chunks.len() - 1 { 0 } else { 1 };
        if i == 0 {
            write!(stdout, "\x1b_Ga=t,f=100,t=d,i={},q=2,m={};{}\x1b\\", IMAGE_ID, m, chunk)?;
        } else {
            write!(stdout, "\x1b_Gm={};{}\x1b\\", m, chunk)?;
        }
    }
    stdout.flush()
}

fn place_image(row: i32, _term_height: u32, frame_num: u32) -> io::Result<bool> {
    let pixels_per_row: i32 = 18;

    // Calculate crop if exiting top
    let crop_y = if row < 1 {
        (1 - row) * pixels_per_row
    } else {
        0
    };

    if crop_y >= ROCKET_HEIGHT as i32 {
        return Ok(false);
    }

    let (cols, _) = get_terminal_size();
    let shake = ((frame_num as f32 * 0.3).sin() * 1.005) as i32;
    let col = (cols as i32 - 12 + shake).max(1);
    let display_row = row.max(1);

    let mut stdout = io::stdout().lock();
    write!(stdout, "\x1b[s")?;
    write!(stdout, "\x1b[{};{}H", display_row, col)?;

    if crop_y > 0 {
        let remaining = ROCKET_HEIGHT as i32 - crop_y;
        write!(stdout, "\x1b_Ga=p,i={},q=2,C=1,y={},h={}\x1b\\", IMAGE_ID, crop_y, remaining)?;
    } else {
        write!(stdout, "\x1b_Ga=p,i={},q=2,C=1\x1b\\", IMAGE_ID)?;
    }

    write!(stdout, "\x1b[u")?;
    stdout.flush()?;
    Ok(true)
}

fn delete_placements() -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    write!(stdout, "\x1b_Ga=d,d=i,i={},q=2\x1b\\", IMAGE_ID)?;
    stdout.flush()
}

fn delete_image() -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    write!(stdout, "\x1b_Ga=d,d=I,i={},q=2\x1b\\", IMAGE_ID)?;
    stdout.flush()
}

fn get_frame_position(frame: u32, terminal_height: u32, total_frames: u32) -> i32 {
    let t = frame as f32 / total_frames as f32;
    let start_y = terminal_height as f32;
    let end_y = -30.0;
    (start_y + t * (end_y - start_y)) as i32
}

fn launch_rocket() {
    if !is_kitty_terminal() {
        eprintln!("termrocket: kitty terminal not detected");
        eprintln!("Set TERM=xterm-kitty or run inside kitty terminal");
        std::process::exit(1);
    }

    // Generate and upload rocket image once
    let rocket = draw_rocket();
    let png_data = encode_png(&rocket);
    let (_, term_height) = get_terminal_size();

    // Hide cursor
    print!("\x1b[?25l");
    let _ = io::stdout().flush();

    // Upload image once
    let _ = upload_image(&png_data);

    let total_frames = (FPS as f32 * DURATION_SECS) as u32;
    let frame_duration = Duration::from_secs_f32(1.0 / FPS as f32);

    for i in 0..total_frames {
        let row = get_frame_position(i, term_height, total_frames);

        let _ = delete_placements();
        match place_image(row, term_height, i) {
            Ok(false) => break,
            Err(_) => break,
            _ => {}
        }

        thread::sleep(frame_duration);
    }

    // Cleanup
    let _ = delete_image();
    print!("\x1b[?25h");
    let _ = io::stdout().flush();
}

fn test_terminal() {
    println!("Checking terminal support...\n");

    let term = std::env::var("TERM").unwrap_or_else(|_| "unknown".to_string());
    let kitty_id = std::env::var("KITTY_WINDOW_ID").ok();

    println!("TERM: {}", term);
    println!("KITTY_WINDOW_ID: {}", kitty_id.as_deref().unwrap_or("not set"));
    println!("Kitty detected: {}", is_kitty_terminal());

    let (cols, rows) = get_terminal_size();
    println!("Terminal size: {}x{}", cols, rows);

    if is_kitty_terminal() {
        println!("\n[OK] Terminal supports kitty graphics protocol");
        println!("Run 'termrocket launch' to see the rocket!");
    } else {
        println!("\n[WARN] Kitty terminal not detected");
        println!("termrocket requires kitty terminal for graphics");
    }
}
