#[derive(Debug, Clone, Copy, PartialEq, Default, Eq, PartialOrd, Ord)]
pub enum TextureHandle {
    #[default]
    Default,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShaderHandle {
    #[default]
    Sprite,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector2F {
    pub x: f32,
    pub y: f32,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub position: Vector2F,
    pub size: Vector2F,
    pub layer: u32,
    pub texture: TextureHandle,
    pub shader: ShaderHandle,
}
impl Default for Entity {
    fn default() -> Self {
        Self {
            position: Default::default(),
            size: Vector2F { x: 1.0, y: 1.0 },
            layer: Default::default(),
            texture: Default::default(),
            shader: Default::default(),
        }
    }
}

impl Entity {
    pub fn new(pos: Vector2F, sprite: TextureHandle) -> Self {
        Entity {
            position: pos,
            size: Vector2F { x: 1.0, y: 1.0 },
            layer: Default::default(),
            texture: sprite,
            shader: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {}
