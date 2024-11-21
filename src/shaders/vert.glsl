#version 300 es

in vec4 position;

void main() {
  // don't modify vertex position
  gl_Position = position;
}
