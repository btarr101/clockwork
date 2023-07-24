use std::{ collections::HashMap, cell::Cell };
use serde::Deserialize;
use serde_json::Value;
use crate::graphics::TextureId;

/// Id for accessing a sprite from a [TextureAtlas].
#[derive(Debug, Clone, Copy)]
pub struct SpriteId(usize);

/// Id for accessing a sprite from a [TextureAtlas], however,
/// it's initialized with the sprite metadata rather than the raw Id.
///
/// This is typically the much easier construct to use.
pub struct LazySpriteId(Cell<LazySpriteIdState>);

#[derive(Clone, Copy)]
enum LazySpriteIdState {
    Uncached(&'static str, Option<&'static str>),
    Cached(SpriteId),
}

/// Contains information on how to render a specific animation
/// from a larger texture with multiple animations bundled together.
#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub texture: TextureId,
    pub uv_topleft: [f32; 2],
    pub uv_dims: [f32; 2],
    pub frames: usize,
}

/// Mapping from [SpriteId] to [Sprite].
#[derive(Debug, Default)]
pub struct TextureAtlas {
    identifiers: HashMap<(String, Option<String>), usize>,
    sprites: Vec<Sprite>,
}

// ####################################
// For deserializing the Aseprite file
// ####################################
#[derive(Deserialize)]
struct Tag {
    name: Option<String>,
    from: usize,
    to: usize,
}

#[derive(Deserialize)]
struct Frame {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}
// ####################################

impl LazySpriteId {
    /// Constructs a new [LazySpriteId].
    pub fn new(image: &'static str, tag: Option<&'static str>) -> Self {
        Self(Cell::new(LazySpriteIdState::Uncached(image, tag)))
    }
}

impl TextureAtlas {
    /// Creates a new empty [TextureAtlas].
    pub fn new() -> Self {
        Self { identifiers: HashMap::new(), sprites: Vec::new() }
    }

    /// Adds sprites from an aseprite generated json document.
    ///
    /// Note this currently assumes two things:
    /// - Animations are marked by tags
    /// - Animations are oriented in row fasion
    pub fn add_aseprite_sprites(&mut self, aseprite_json_context: &str, texture: TextureId) {
        let aseprite_json: Value = serde_json::from_str(aseprite_json_context).unwrap();
        let meta = aseprite_json.get("meta").unwrap();

        let image = meta.get("image").unwrap().as_str().unwrap();
        let dimensions = meta
            .get("size")
            .map(|value| {
                (
                    value.get("w").unwrap().as_u64().unwrap() as f64,
                    value.get("h").unwrap().as_u64().unwrap() as f64,
                )
            })
            .unwrap();

        let frames: Vec<Frame> = aseprite_json
            .get("frames")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|value|
                serde_json::from_value::<Frame>(value.get("frame").unwrap().clone()).unwrap()
            )
            .collect();

        let tag_array = meta.get("frameTags").unwrap().as_array().unwrap();

        let tags: Vec<Tag> = match tag_array.is_empty() {
            true => vec![Tag { name: None, from: 0, to: frames.len() - 1 }],
            false =>
                tag_array
                    .iter()
                    .map(|value| serde_json::from_value::<Tag>(value.clone()).unwrap())
                    .collect(),
        };

        let sprites_and_tags = tags.iter().map(|tag| {
            let first_frame = &frames[tag.from];
            let uv_topleft = [
                ((first_frame.x as f64) / dimensions.0) as f32,
                ((first_frame.y as f64) / dimensions.1) as f32,
            ];
            let uv_dims = [
                ((first_frame.w as f64) / dimensions.0) as f32,
                ((first_frame.h as f64) / dimensions.1) as f32,
            ];
            let frames = tag.to + 1 - tag.from;

            let sprite = Sprite {
                texture,
                uv_topleft,
                uv_dims,
                frames,
            };

            (tag.name.clone(), sprite)
        });

        sprites_and_tags.for_each(|(tag, sprite)| {
            self.add_sprite(sprite, image, tag.as_deref());
        });
    }

    /// Gets a [Sprite] from this [TextureAtlas] given the [SpriteId].
    ///
    /// Panics if the [SpriteId] is invalid.
    pub fn get_sprite(&self, sprite_id: SpriteId) -> &Sprite {
        &self.sprites[sprite_id.0]
    }

    /// Gets a [Sprite] from this [TextureAtlas] given a [LazySpriteId].
    ///
    /// Panics if the [LazySpriteId] is invalid.
    pub fn get_sprite_lazily(&self, lazy_sprite_id: &LazySpriteId) -> &Sprite {
        let sprite_id = match lazy_sprite_id.0.get() {
            LazySpriteIdState::Uncached(image, tag) => {
                let sprite_id = self.get_sprite_id(image, tag).unwrap();
                lazy_sprite_id.0.set(LazySpriteIdState::Cached(sprite_id));
                sprite_id
            }
            LazySpriteIdState::Cached(sprite_id) => sprite_id,
        };

        self.get_sprite(sprite_id)
    }

    /// Gets a [SpriteId] given proper identification.
    pub fn get_sprite_id(&self, image: &str, tag: Option<&str>) -> Option<SpriteId> {
        let index = *self.identifiers.get(&(image.to_string(), tag.map(|str| str.to_string())))?;
        Some(SpriteId(index))
    }

    /// Adds a [Sprite] to this [TextureAtlas], and returns its [SpriteId].
    pub fn add_sprite(&mut self, sprite: Sprite, image: &str, tag: Option<&str>) -> SpriteId {
        let index = self.sprites.len();
        self.sprites.push(sprite);
        self.identifiers.insert((image.to_string(), tag.map(|str| str.to_string())), index);
        SpriteId(index)
    }
}

impl Sprite {
    /// Gets the appropriate uv coordinate window within the texture to render the
    /// corresponding frame for this sprite.
    pub fn get_uv_window(&self, frame: usize) -> [f32; 4] {
        let left = self.uv_topleft[0] + self.uv_dims[0] * ((frame % self.frames) as f32);

        [left, self.uv_topleft[1], self.uv_dims[0], self.uv_dims[1]]
    }
}
