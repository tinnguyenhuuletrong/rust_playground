use image::{GenericImageView, Rgba};
use rust_bevy_play::level::{EntityData, LevelData};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: level_generator <path_to_image>");
        return;
    }

    let img_path = Path::new(&args[1]);
    let img = image::open(img_path).expect("Failed to open image");

    let (width, height) = img.dimensions();
    let mut entities = Vec::new();

    let ground_y = height as i32 - 1;

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let entity = match pixel {
                Rgba([0, 0, 0, 255]) => Some(EntityData {
                    entity_type: "box".to_string(),
                    position: [x as i32, y as i32],
                }),
                Rgba([255, 0, 0, 255]) => Some(EntityData {
                    entity_type: "bird".to_string(),
                    position: [x as i32, y as i32],
                }),
                _ => None,
            };

            if let Some(entity) = entity {
                entities.push(entity);
            }
        }
    }

    let level_data = LevelData {
        world_bound: [0, 0, width as i32, height as i32],
        ground_y,
        entities,
    };

    let json_string =
        serde_json::to_string_pretty(&level_data).expect("Failed to serialize level data");

    let file_name = img_path.file_stem().unwrap().to_str().unwrap();
    let output_path = format!("assets/data/{}.json", file_name);

    let mut file = File::create(&output_path).expect("Failed to create output file");
    file.write_all(json_string.as_bytes())
        .expect("Failed to write to output file");

    println!("Level generated successfully at {}", output_path);
}
