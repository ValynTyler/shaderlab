use web_sys::{WebGl2RenderingContext as GlContext, WebGlShader};

pub trait Shader {
    fn gl_enum() -> u32;
    fn compile(
        context: &GlContext,
        source: &str,
    ) -> Result<Self, String>
        where Self: Sized + From::<WebGlShader> {
        match compile_shader(context, Self::gl_enum(), source) {
            Ok(gl_shader) => Ok(Self::from(gl_shader)),
            Err(e) => Err(e),
        }
    }
}

fn compile_shader(
    context: &GlContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    match context
        .get_shader_parameter(&shader, GlContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        true => Ok(shader),
        false => Err(
            context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"))
        ),
    }
}
