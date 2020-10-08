#version 330 core
#extension GL_ARB_separate_shader_objects : enable
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

layout (location = 0) out vec3 ourColor;
layout (location = 1) out vec2 TexCoord;

uniform mat4 mvp;
uniform vec3 color;

void main() {
	gl_Position = mvp * vec4(aPos, 1.0);
	ourColor = color;
	TexCoord = aTexCoord;
}
