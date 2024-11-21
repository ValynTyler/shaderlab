use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext as GlContext;
use web_sys::{WebGlProgram, WebGlShader};

#[wasm_bindgen(js_namespace = console)]
extern {
    fn log(string: &str);
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    // get DOM elements
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    log("nice shaders...");

    // aquire a webgl context inside the canvas element
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<GlContext>()?;

    // compile shaders
    let vert_shader = compile_shader(
        &context,
        GlContext::VERTEX_SHADER,
        include_str!("shaders/vert.glsl")
    )?;

    let frag_shader = compile_shader(
        &context,
        GlContext::FRAGMENT_SHADER,
        include_str!("shaders/frag.glsl")
    )?;

    // bind and enable shader program
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    // quad vertices
    let vertices: [f32; 9] = [
        -0.7, -0.7,  0.0,
         0.7, -0.7,  0.0,
         0.0,  0.7,  0.0,
    ];

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(GlContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            GlContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            GlContext::STATIC_DRAW,
        );
    }

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        GlContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.bind_vertex_array(Some(&vao));

    let vert_count = (vertices.len() / 3) as i32;
    draw(&context, vert_count);

    Ok(())
}

fn draw(context: &GlContext, vert_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(GlContext::COLOR_BUFFER_BIT);

    context.draw_arrays(GlContext::TRIANGLES, 0, vert_count);
}

pub fn compile_shader(
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

pub fn link_program(
    context: &GlContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    match context 
        .get_program_parameter(&program, GlContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        true => Ok(program),
        false => Err(
            context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object"))
        ),
    }
}

