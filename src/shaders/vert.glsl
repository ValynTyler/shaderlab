#version 300 es

layout(location = 0) in vec4 position; // between 0 and 15

void main() {
  // don't modify vertex position
  gl_Position = position;
}
