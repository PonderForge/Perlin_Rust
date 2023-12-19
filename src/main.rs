use rand::prelude::*;
use noise::{NoiseFn, PerlinSurflet};
use sdl2::{event::Event, surface::Surface, pixels::PixelFormatEnum};
use rayon::prelude::*;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Perlin Window", 600, 600)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut rng = rand::thread_rng();
    println!("Starting Generator");
    let perlin = PerlinSurflet::new(rng.gen());
    let mut zoom: f64 = 0.05;
    let mut buffer = vec![0 as u8; 800*800*3];
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'main
                },
                Event::MouseWheel { y, .. } => {
                    if y < 0 {
                        zoom += 0.01;
                    } else if y > 0 {
                        zoom -= 0.01;
                    }
                },
                _ => {}
            }
        }
        buffer.par_chunks_mut(3).enumerate().for_each(|( i, o )| {
            let y: f64 = (i as f64) % 800.0;
            let x: f64 = ((i as f64) - y)/800.0;
            //println!("{:?}", (x,y, i));
            let color: u8 = ((perlin.get([x*zoom, y*zoom]) + 1.0)*127.0) as u8;
            if color < 2 {
                o[0] = 255;
                o[1] = 255;
                o[2] = 255;
            } else if color < 30 {
                o[0] = 127;
                o[1] = 131;
                o[2] = 134;
            } else if color < 80 {
                o[0] = 126;
                o[1] = 200;
                o[2] = 80;
            } else if color < 110 {
                o[0] = 253;
                o[1] = 217;
                o[2] = 181;
            } else {
                o[0] = 0;
                o[1] = 169;
                o[2] = 204;
            }
        });
        // Show it on the screen
        let surf = Surface::from_data(&mut buffer, 800, 800, 800*3, PixelFormatEnum::RGB24).unwrap();
        let tex = canvas.create_texture_from_surface(&surf).unwrap();
        canvas.copy(&tex, None, None).unwrap();
        canvas.present();
    }
    Ok(())
}
