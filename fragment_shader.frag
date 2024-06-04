#version 330

#define PI 3.1415926535

struct Camera {
    vec3 position;
    float fov;
};

struct Material {
    vec3 albedo;
    float roughness;
    float metallic;
};

struct Sphere {
    vec3 position;
    float radius;
    Material material;
};

uniform Camera camera;
uniform Sphere sphere;

void main() {
    vec3 color = vec3(0.0, 0.0, 0.0);

    float width = 960.0;
    float height = 540.0;

    vec2 size = vec2(width, height);

    vec3 albedo = sphere.material.albedo;
    float roughness = sphere.material.roughness;
    float metallic = sphere.material.metallic;

    vec2 fragCoord = gl_FragCoord.xy;

    vec2 uv = fragCoord - vec2(width / 2.0, height / 2.0);

    float focal_length = width / (2.0 * tan(camera.fov * PI / 360.0));

    vec3 ray_direction = normalize(vec3(uv.x, uv.y, focal_length));
    vec3 ray_origin = camera.position - sphere.position;

    float b = 2.0 * dot(ray_direction, ray_origin);
    float c = dot(ray_origin, ray_origin) - sphere.radius * sphere.radius;

    float d = b * b - 4.0 * c;

    if (d > 0.0) {

        float t1 = -b + sqrt(d);
        float t2 = -b - sqrt(d);

        float t = -1.0;

        if (t1 > 0.0 && t2 > 0.0) {
            t = min(t1, t2);
        } else if (t1 > 0.0) {
            t = t1;
        } else if (t2 > 0.0) {
            t = t2;
        }
        
        if (t > 0.0) {
            vec3 hit_position = ray_origin + t * ray_direction;
            vec3 normal = hit_position - sphere.position;

            vec3 light = normalize(vec3(-1.0, -1.0, -0.5));


            float luminence = dot(normal, -light);

            color = albedo * luminence;
        }

    }



    gl_FragColor = vec4(color, 1.0);
}
