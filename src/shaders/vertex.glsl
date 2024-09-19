
const vec4 colors[3] = vec4[3](
    vec4(1.0, 0.0, 0.0, 1.0),
    vec4(0.0, 1.0, 0.0, 1.0),
    vec4(0.0, 0.0, 1.0, 1.0)
);

in vec2 v_position;
out vec4 v_color;
uniform float u_angle;

void main() {
    v_color = colors[gl_VertexID % 3];
    gl_Position = vec4(v_position, 0.5, 1.0);
}
