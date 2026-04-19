#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextureHandle {
    Test,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShaderHandle {
    DefaultSprite,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2F {
    pub x: f32,
    pub y: f32,
}
#[derive(Debug, Clone, Copy)]
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

#[cfg(test)]
mod tests {}
