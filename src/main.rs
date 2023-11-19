
use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

enum CardType{
    Flous,
    Syouf,
    Gro3,
    Jbaben
}


struct Card {
    pub id: usize,
    pub kind : CardType,
    pub number : i32, 
    pub coords: (Vec2, Vec2),
    pub image_path: &'static str,

}



// Add this definition at the beginning of your file
#[derive(Debug, Clone, Copy)]
struct TextureCoordinates {
    min: Vec2,
    max: Vec2,
}



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, render_cards)
        .run();
}

fn render_cards(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

    // Load the card texture
   
    // Define card coordinates
    let card1_coords = (Vec2::new(0.0, 0.0), Vec2::new(200.0, 321.0));
    let card2_coords = (Vec2::new(201.0, 0.0), Vec2::new(410.0, 320.0));
    let card3_coords = (Vec2::new(413.0, 0.0), Vec2::new(623.0, 320.0));
    let card4_coords = (Vec2::new(625.0, 0.0), Vec2::new(824.0, 320.0));

    // Create an array of cards with image paths
    let cards = vec![
        Card { id: 1, coords: card1_coords, number : 1, kind: CardType::Flous ,image_path: "cards/1.jpg" },
        Card { id: 2, coords: card2_coords, number : 2, kind: CardType::Flous ,image_path: "cards/2.jpg" },
        Card { id: 3, coords: card3_coords, number : 3, kind: CardType::Flous ,image_path: "cards/3.jpg" },
        Card { id: 4, coords: card4_coords, number : 4, kind: CardType::Flous ,image_path: "cards/4.jpg" },

    ];



    for card in cards {
        let x = card.coords.0.x;
        let y = card.coords.0.y;
        let texture : Handle<Image> = asset_server.load(card.image_path);

        commands.spawn(SpriteBundle {
            texture: texture.clone(),
            sprite: Sprite {
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            ..Default::default()
        });

    }

    // Spawn cards in a grid

}

