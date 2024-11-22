use web_sys::{WebGl2RenderingContext, WebGlShader};

use super::Shader;

pub struct VertexShader {
    gl_shader: WebGlShader,
}

impl Shader for VertexShader {
    fn gl_enum() -> u32 {
        WebGl2RenderingContext::VERTEX_SHADER
    }
}

impl From::<WebGlShader> for VertexShader {
    fn from(value: WebGlShader) -> Self {
        Self { gl_shader: value }
    }
}
