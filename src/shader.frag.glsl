#version 330 core
#extension GL_ARB_separate_shader_objects : enable
layout (location = 0) out vec4 FragColor;

layout (location = 0) in vec3 ourColor;
layout (location = 1) in vec2 TexCoord;

uniform sampler2D tex;

void main() {
	FragColor = texture(tex, TexCoord) * vec4(ourColor, 1.0);
}
