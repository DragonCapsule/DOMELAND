#version 330 core

#include <globals.glsl>

in vec3 f_pos;

layout (std140)
uniform u_locals {
	vec4 nul;
};

out vec4 tgt_color;

const float PI = 3.141592;

vec3 get_sky_color(vec3 dir, float time_of_day) {
	const float TIME_FACTOR = (PI * 2.0) / (3600.0 * 24.0);

	const vec3 SKY_TOP    = vec3(0.0, 0.3, 1.0);
	const vec3 SKY_BOTTOM = vec3(0.0, 0.05, 0.2);

	const vec3 SUN_HALO_COLOR  = vec3(1.0, 0.8, 0.5);
	const vec3 SUN_SURF_COLOR  = vec3(1.0, 0.8, 0.0) * 100.0;

	float sun_angle_rad = time_of_day * TIME_FACTOR;
	vec3 sun_dir = vec3(sin(sun_angle_rad), 0.0, cos(sun_angle_rad));

	vec3 sun_halo = pow(max(dot(dir, sun_dir), 0.0), 8.0) * SUN_HALO_COLOR;
	vec3 sun_surf = pow(max(dot(dir, sun_dir), 0.0), 1000.0) * SUN_SURF_COLOR;
	vec3 sun_light = sun_halo + sun_surf;

	return mix(SKY_BOTTOM, SKY_TOP, (dir.z + 1.0) / 2.0) + sun_light;
}

void main() {
	tgt_color = vec4(get_sky_color(normalize(f_pos), time_of_day.x), 1.0);
}
