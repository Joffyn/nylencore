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
    position: Vector2F,
    size: Vector2F,
    layer: u32,
    texture: TextureHandle,
    shader: ShaderHandle,
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
    fn new(pos: Vector2F, sprite: TextureHandle) -> Self {
        Entity {
            position: pos,
            size: Vector2F { x: 1.0, y: 1.0 },
            layer: Default::default(),
            texture: sprite,
            shader: Default::default(),
        }
    }
    fn get_pos(&self) -> &Vector2F {
        &self.position
    }
    fn get_size(&self) -> &Vector2F {
        &self.size
    }
}

#[cfg(test)]
mod tests {}
