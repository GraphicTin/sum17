

use macroquad::prelude::*;

pub struct Textures {
    pub slice: Texture2D,
    // Add more here later: pub player: Texture2D, etc.
}

pub struct Assets {
    pub textures: Textures,
    pub font: Font,
}

impl Assets {
    pub async fn load() -> Result<Self, String> {

        let slice = load_texture("assets/slice/slice (1).png")
            .await.map_err(|e| e.to_string())?;

        let font = load_ttf_font("assets/font/bungee-regular.ttf")
            .await.map_err(|e| e.to_string())?;

        Ok( Self {
            textures: Textures { slice },
            font,
        } )

    }
}