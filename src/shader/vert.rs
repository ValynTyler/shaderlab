use web_sys::WebGlShader;

pub struct VertexShader {
    gl_shader: WebGlShader,
}

impl From::<WebGlShader> for VertexShader {
    fn from(value: WebGlShader) -> Self {
        Self { gl_shader: value }
    }
}
