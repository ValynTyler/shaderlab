use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::{FragmentShader, VertexShader};

pub struct Program {
    pub gl_program: WebGlProgram
}

impl Program {
    // TODO: make private to prevent incorrect context linking
    pub fn new(context: &WebGl2RenderingContext) -> Result<Self, String> {
        match context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object")) {
                Ok(gl_program) => Ok(Self { gl_program }),
                Err(e) => Err(e),
            }
    }

    pub fn link(
        self: &Self,
        context: &WebGl2RenderingContext,
        vert_shader: &VertexShader,
        frag_shader: &FragmentShader,
    ) -> Result<(), String> {
        context.attach_shader(&self.gl_program, &vert_shader.gl_shader);
        context.attach_shader(&self.gl_program, &frag_shader.gl_shader);
        context.link_program(&self.gl_program);

        match context 
            .get_program_parameter(&self.gl_program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false) {
                true => Ok(()),
                false => Err(
                    context
                    .get_program_info_log(&self.gl_program)
                    .unwrap_or_else(|| String::from("Unknown error creating program object"))
                ),
            }
    }
}
