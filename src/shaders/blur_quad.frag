varying vec2 textureCoord;

void main() {
   vec4 color1 = texture2D(t, textureCoord);
   gl_FragColor = color1;
}
