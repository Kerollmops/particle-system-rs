#version 400

const vec2 madd = vec2(0.5, 0.5);

attribute vec2 vertexIn;
varying vec2 textureCoord;

out vec2 v_uv;

void main() {
   textureCoord = vertexIn.xy * madd + madd; // scale vertex attribute to [0-1] range
   gl_Position = vec4(vertexIn.xy, 0.0, 1.0);
}
