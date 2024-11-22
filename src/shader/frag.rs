use web_sys::{WebGl2RenderingContext, WebGlShader};

use super::Shader;

pub struct FragmentShader {
    gl_shader: WebGlShader,
}

impl Shader for FragmentShader {
    fn gl_enum() -> u32 {
        WebGl2RenderingContext::FRAGMENT_SHADER
    }
}

impl From::<WebGlShader> for FragmentShader {
    fn from(value: WebGlShader) -> Self {
        Self { gl_shader: value }
    }
}
