#version 460 core
layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;
layout(rgba32f, binding = 0) uniform image2D screen;


layout(std430, binding = 2) buffer circle_position_buffer {
    vec2 circle_position[];
};

layout(std430, binding = 3) buffer circle_color_buffer {
    vec3 circle_color[];
};

layout(std430, binding = 4) buffer circle_radius_buffer {
    float circle_radius[];
};


uniform vec2 offset;
uniform float zoom;
uniform float dt;

void main()
{
    ivec2 texel_coords = ivec2(gl_GlobalInvocationID.xy);
    vec2 screen_resolution = vec2(imageSize(screen));
    vec2 coords = (vec2(texel_coords) + vec2(0.5)) / screen_resolution;
    float aspect_ratio = screen_resolution.x / screen_resolution.y;

    coords = (coords - 0.5) * 2.0; // Transform to range [-1, 1]
    coords /= zoom; // Apply zoom
    coords += offset; // Apply camera offset
    coords.x *= aspect_ratio; // Adjust for aspect ratio
    
    vec4 final_color = vec4(0.0);

    for (int i = 0; i < circle_color.length(); ++i) {
        vec2 pos = vec2(circle_position[i].x * aspect_ratio, circle_position[i].y);
        float distance = length(coords - pos);

        float smooth_edge = 0.0000001;
        float alpha = 1.0 - smoothstep(circle_radius[i] - smooth_edge, circle_radius[i], distance);

        vec4 color = vec4(circle_color[i], alpha);

        final_color = final_color + color * color.a * (1.0 - final_color.a);

        float glow_radius = circle_radius[i] * 1.2; // Reduced glow radius
        float glow_distance = distance - circle_radius[i];
        float glow = 0.001 / glow_distance; // Adjusted glow scaling factor
        glow = clamp(glow, 0.0, 1.0);

        vec4 glow_color = vec4(circle_color[i] * glow, glow);

        final_color.rgb += glow_color.rgb * glow_color.a * (1.0 - final_color.a);
        final_color.a += glow_color.a * (1.0 - final_color.a);
    }

    // Trails effect with blur
    float trail_decay = 0.8; // Adjust decay rate for trails
    float blur_radius = 0.5; // Adjust blur radius

    vec4 trail_color = vec4(0.0);

    // Simple box blur: sample neighboring pixels
    for (int dx = -1; dx <= 1; ++dx) {
        for (int dy = -1; dy <= 1; ++dy) {
            ivec2 offset_coords = texel_coords + ivec2(dx, dy);
            vec4 sample_color = imageLoad(screen, offset_coords);
            trail_color += sample_color;
        }
    }

    trail_color /= 8.0; // Divide by the number of samples to average

    // Apply decay to the blurred trail color
    trail_color *= trail_decay;

    // Extract bright areas for bloom
    vec4 bloom_color = vec4(0.0);
    if (trail_color.r > 0.8 || trail_color.g > 0.8 || trail_color.b > 0.8) {
        bloom_color = trail_color;
    }

    // Apply Gaussian blur to bloom color (simple approximation)
    vec4 blurred_bloom = vec4(0.0);
    float blur_weights[5] = float[](0.227027, 0.1945946, 0.1216216, 0.05405405, 0.016216216);
    for (int i = -2; i <= 2; ++i) {
        for (int j = -2; j <= 2; ++j) {
            ivec2 offset_coords = texel_coords + ivec2(i, j);
            vec4 sample_color = imageLoad(screen, offset_coords) * blur_weights[abs(i)] * blur_weights[abs(j)];
            blurred_bloom += sample_color;
        }
    }

    // Blend the blurred bloom color with the final color
    final_color = max(final_color, blurred_bloom); // Keep the brightest color

    // Blend the blurred trail color with the final color
    final_color = max(final_color, trail_color);

    imageStore(screen, texel_coords, final_color);
}
