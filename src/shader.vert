#version 140

uniform float scale;
uniform float size;
uniform vec2 offset;
uniform float t_max;

in vec2 position;
in float dist;
out float dist_param;

void main() {
    gl_Position = vec4((position / size + offset) * scale, 0.0, 1.0);
    dist_param = dist / t_max;
}
