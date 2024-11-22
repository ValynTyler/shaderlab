use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::{FragmentShader, VertexShader};

pub struct Program {
    pub gl_program: WebGlProgram
}

impl Program {
    pub fn link(
        context: &WebGl2RenderingContext,
        vert_shader: &VertexShader,
        frag_shader: &FragmentShader,
    ) -> Result<Self, String> {
        let gl_program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.attach_shader(&gl_program, &vert_shader.gl_shader);
        context.attach_shader(&gl_program, &frag_shader.gl_shader);
        context.link_program(&gl_program);

        match context 
            .get_program_parameter(&gl_program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false) {
                true => Ok(Self { gl_program }),
                false => Err(
                    context
                    .get_program_info_log(&gl_program)
                    .unwrap_or_else(|| String::from("Unknown error creating program object"))
                ),
            }
    }
}
