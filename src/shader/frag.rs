use web_sys::WebGlShader;

pub struct FragmentShader {
    gl_shader: WebGlShader,
}

impl From::<WebGlShader> for FragmentShader {
    fn from(value: WebGlShader) -> Self {
        Self { gl_shader: value }
    }
}
