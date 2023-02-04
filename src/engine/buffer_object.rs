#[allow(dead_code)]
pub enum BufferType {
    Vertex,
    VertexArray,
    Element,
    Texture
}

#[allow(dead_code)]
pub struct VertexArrayObject {
    id: u32
}

#[allow(dead_code)]
pub struct TextureBufferObject {
    id: u32
}

#[allow(dead_code)]
pub struct ElementBufferObject {
    id: u32
}

#[allow(dead_code)]
pub struct VertexBufferObject {
    id: u32
}