#[allow(dead_code)]
pub enum BufferType {
    Vertex,
    VertexArray,
    Element,
    Texture
}

#[allow(dead_code)]
pub struct Buffer {
    b_type: BufferType,
    id: u32

}