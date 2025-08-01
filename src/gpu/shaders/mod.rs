
pub mod vertex_shader {
    vulkano_shaders::shader!{
        ty: "vertex",
        src: r"
            #version 460

            layout(location = 0) in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        ",
    }
}




pub mod fragment_shader {
    vulkano_shaders::shader!{
        ty: "fragment",
        src: r"
            #version 460

            // variable for the color of each pixel
            // we can have any number of images to write to, so there is no builtin
            layout(location = 0) out vec4 f_color;



            float distanceToLineSegment(vec2 p, vec2 a, vec2 b) {
                vec2 ab = b - a;
                vec2 ap = p - a;
                float t = clamp(dot(ap, ab) / dot(ab, ab), 0.0, 1.0);
                vec2 closestPoint = a + t * ab;
                return length(p - closestPoint);
            }

            void main() {
                // Get the fragment's position in normalized device coordinates
                vec2 fragCoord = gl_FragCoord.xy / vec2(1024.0, 1024.0); // Adjust based on your viewport size

                // Define the triangle vertices (in normalized device coordinates)
                vec2 v0 = vec2(0.0, 0.5);
                vec2 v1 = vec2(-0.5, -0.5);
                vec2 v2 = vec2(0.5, -0.5);

                // Calculate distances to each edge of the triangle
                float d0 = distanceToLineSegment(fragCoord, v0, v1);
                float d1 = distanceToLineSegment(fragCoord, v1, v2);
                float d2 = distanceToLineSegment(fragCoord, v2, v0);

                // Set border width
                float borderWidth = 0.1; // Adjust as needed

                // Determine if the fragment is within the border
                if (d0 < borderWidth || d1 < borderWidth || d2 < borderWidth) {
                    f_color = vec4(0.0, 0.0, 0.0, 1.0); // Border color (black)
                } else {
                    f_color = vec4(1.0, 0.0, 0.0, 1.0); // Triangle color (red)
                }
                // f_color = vec4(fragCoord, 0.0, 1.0);
            }
        ",
    }
}




/*
pub mod acerola_vertex_shader {
    vulkano_shaders::shader!{
        ty: "vertex",
        src: r"
#version 450

// This is the uniform buffer that contains all of the settings we sent over from the cpu in _render_callback. Must match with the one in the fragment shader.
layout(set = 0, binding = 0, std140) uniform UniformBufferObject {
    mat4 MVP;
    vec3 _LightDirection;
    float _GradientRotation;
    float _NoiseRotation;
    float _TerrainHeight;
    vec2 _AngularVariance;
    float _Scale;
    float _Octaves;
    float _AmplitudeDecay;
    float _NormalStrength;
    vec3 _Offset;
    float _Seed;
    float _InitialAmplitude;
    float _Lacunarity;
    vec2 _SlopeRange;
    vec4 _LowSlopeColor;
    vec4 _HighSlopeColor;
    float _FrequencyVarianceLowerBound;
    float _FrequencyVarianceUpperBound;
    float _SlopeDamping;
    vec4 _AmbientLight;
};

// This is the vertex data layout that we defined in initialize_render after line 198
layout(location = 0) in vec3 a_Position;
layout(location = 1) in vec4 a_Color;

// This is what the vertex shader will output and send to the fragment shader.
layout(location = 2) out vec4 v_Color;
layout(location = 3) out vec3 pos;

#define PI 3.141592653589793238462

// UE4's PseudoRandom function
// https://github.com/EpicGames/UnrealEngine/blob/release/Engine/Shaders/Private/Random.ush
float pseudo(vec2 v) {
    v = fract(v/128.)*128. + vec2(-64.340622, -72.465622);
    return fract(dot(v.xyx * v.xyy, vec3(20.390625, 60.703125, 2.4281209)));
}

// Takes our xz positions and turns them into a random number between 0 and 1 using the above pseudo random function
float HashPosition(vec2 pos) {
    return pseudo(pos * vec2(_Seed, _Seed + 4));
}

// Generates a random gradient vector for the perlin noise lattice points, watch my perlin noise video for a more in depth explanation
vec2 RandVector(float seed) {
    float theta = seed * 360 * 2 - 360;
    theta += _GradientRotation;
    theta = theta * PI / 180.0;
    return normalize(vec2(cos(theta), sin(theta)));
}

// Normal smoothstep is cubic -- to avoid discontinuities in the gradient, we use a quintic interpolation instead as explained in my perlin noise video
vec2 quinticInterpolation(vec2 t) {
    return t * t * t * (t * (t * vec2(6) - vec2(15)) + vec2(10));
}

// Derivative of above function
vec2 quinticDerivative(vec2 t) {
    return vec2(30) * t * t * (t * (t - vec2(2)) + vec2(1));
}

// it's perlin noise that returns the noise in the x component and the derivatives in the yz components as explained in my perlin noise video
vec3 perlin_noise2D(vec2 pos) {
    vec2 latticeMin = floor(pos);
    vec2 latticeMax = ceil(pos);

    vec2 remainder = fract(pos);

    // Lattice Corners
    vec2 c00 = latticeMin;
    vec2 c10 = vec2(latticeMax.x, latticeMin.y);
    vec2 c01 = vec2(latticeMin.x, latticeMax.y);
    vec2 c11 = latticeMax;

    // Gradient Vectors assigned to each corner
    vec2 g00 = RandVector(HashPosition(c00));
    vec2 g10 = RandVector(HashPosition(c10));
    vec2 g01 = RandVector(HashPosition(c01));
    vec2 g11 = RandVector(HashPosition(c11));

    // Directions to position from lattice corners
    vec2 p0 = remainder;
    vec2 p1 = p0 - vec2(1.0);

    vec2 p00 = p0;
    vec2 p10 = vec2(p1.x, p0.y);
    vec2 p01 = vec2(p0.x, p1.y);
    vec2 p11 = p1;
    
    vec2 u = quinticInterpolation(remainder);
    vec2 du = quinticDerivative(remainder);

    float a = dot(g00, p00);
    float b = dot(g10, p10);
    float c = dot(g01, p01);
    float d = dot(g11, p11);

    // Expanded interpolation freaks of nature from https://iquilezles.org/articles/gradientnoise/
    float noise = a + u.x * (b - a) + u.y * (c - a) + u.x * u.y * (a - b - c + d);

    vec2 gradient = g00 + u.x * (g10 - g00) + u.y * (g01 - g00) + u.x * u.y * (g00 - g10 - g01 + g11) + du * (u.yx * (a - b - c + d) + vec2(b, c) - a);
    return vec3(noise, gradient);
}

// The fractional brownian motion that sums many noise values as explained in the video accompanying this project
vec3 fbm(vec2 pos) {
    float lacunarity = _Lacunarity;
    float amplitude = _InitialAmplitude;

    // height sum
    float height = 0.0;

    // derivative sum
    vec2 grad = vec2(0.0);

    // accumulated rotations
    mat2 m = mat2(1.0, 0.0,
                    0.0, 1.0);

    // generate random angle variance if applicable
    float angle_variance = mix(_AngularVariance.x, _AngularVariance.y, HashPosition(vec2(_Seed, 827)));
    float theta = (_NoiseRotation + angle_variance) * PI / 180.0;

    // rotation matrix
    mat2 m2 = mat2(cos(theta), -sin(theta),
                    sin(theta),  cos(theta));
        
    mat2 m2i = inverse(m2);

    for(int i = 0; i < int(_Octaves); ++i) {
        vec3 n = perlin_noise2D(pos);
        
        // add height scaled by current amplitude
        height += amplitude * n.x;	
        
        // add gradient scaled by amplitude and transformed by accumulated rotations
        grad += amplitude * m * n.yz;
        
        // apply amplitude decay to reduce impact of next noise layer
        amplitude *= _AmplitudeDecay;
        
        // generate random angle variance if applicable
        angle_variance = mix(_AngularVariance.x, _AngularVariance.y, HashPosition(vec2(i * 419, _Seed)));
        theta = (_NoiseRotation + angle_variance) * PI / 180.0;

        // reconstruct rotation matrix, kind of a performance stink since this is technically expensive and doesn't need to be done if no random angle variance but whatever it's 2025
        m2 = mat2(cos(theta), -sin(theta),
                    sin(theta),  cos(theta));
        
        m2i = inverse(m2);

        // generate frequency variance if applicable
        float freq_variance = mix(_FrequencyVarianceLowerBound, _FrequencyVarianceUpperBound, HashPosition(vec2(i * 422, _Seed)));

        // apply frequency adjustment to sample position for next noise layer
        pos = (lacunarity + freq_variance) * m2 * pos;
        m = (lacunarity + freq_variance) * m2i * m;
    }

    return vec3(height, grad);
}

void main() {
    // Passes the vertex color over to the fragment shader, even though we don't use it but you can use it if you want I guess
    v_Color = a_Color;

    // The fragment shader also calculates the fractional brownian motion for pixel perfect normal vectors and lighting, so we pass the vertex position to the fragment shader
    pos = a_Position;

    // Initial noise sample position offset and scaled by uniform variables
    vec3 noise_pos = (pos + vec3(_Offset.x, 0, _Offset.z)) / _Scale;

    // The fractional brownian motion
    vec3 n = fbm(noise_pos.xz);

    // Adjust height of the vertex by fbm result scaled by final desired amplitude
    pos.y += _TerrainHeight * n.x + _TerrainHeight - _Offset.y;
    
    // Multiply final vertex position with model/view/projection matrices to convert to clip space
    gl_Position = MVP * vec4(pos, 1);
}
		"
    }
}
*/