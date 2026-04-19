pub enum TextureHandle {
    Test,
}
pub enum ShaderHandle {
    DefaultSprite,
}
pub struct Vector2F {
    pub x: f32,
    pub y: f32,
}
pub struct Vector3F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub struct Entity {
    pub position: Vector2F,
    pub size: Vector2F,
    pub layer: u32,
    pub texture: TextureHandle,
    pub shader: ShaderHandle,
}

#[cfg(test)]
mod tests {}
