#version 330

#define PI 3.14159265358979323846264338327950288

struct Camera {
    vec3 position;
    vec3 angles;
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
uniform Sphere spheres[3];

uniform float width;
uniform float height;

vec3 rotate_x(vec3 point, float angle) {
    return vec3(point.x, point.y * cos(angle) - point.z * sin(angle), point.y * sin(angle) + point.z * cos(angle));
}

vec3 rotate_y(vec3 point, float angle) {
    return vec3(point.x * cos(angle) + point.z * sin(angle), point.y, point.x * -sin(angle) + point.z * cos(angle));
}

vec3 rotate_z(vec3 point, float angle) {
    return vec3(point.x * cos(angle) + point.y * -sin(angle), point.x * sin(angle) + point.y * cos(angle), point.z);
}

vec3 rotate(vec3 point, vec3 angles) {
    return rotate_z(rotate_y(rotate_x(point, angles.x), angles.y), angles.z);
}
struct HitInfo {
    bool background;
    vec3 normal;
    vec3 position;
    float distance;
    Material material;
};

struct Ray {
    vec3 origin;
    vec3 direction;
};

HitInfo trace_ray(Ray ray) {
    HitInfo closest_hit = HitInfo(true, vec3(1, 1, 1), vec3(0, 0, 0), 0, Material(vec3(1, 1, 1), 0, 0));
    for(int i = 0; i < spheres.length(); i++) {
        Sphere sphere = spheres[i];

        // A to B = B - A
        vec3 relative = ray.origin - sphere.position;

        float b = 2.0 * dot(ray.direction, relative);
        float c = dot(relative, relative) - sphere.radius * sphere.radius;

        float d = b * b - 4.0 * c;

        if(d >= 0.0) {

            float t1 = (-b + sqrt(d)) / 2.0;
            float t2 = (-b - sqrt(d)) / 2.0;

            float t = (t1 > 0.0 && (t1 < t2 || t2 <= 0.0)) ? t1 : t2;

            if(t >= 0.0 && (closest_hit.background || t < closest_hit.distance)) {

                vec3 hit_position = ray.origin + t * ray.direction;
                vec3 normal = normalize(hit_position - sphere.position);

                closest_hit = HitInfo(false, normal, hit_position, t, sphere.material);
            }
        }

    }
    return closest_hit;
}

vec3 calculate_color(Ray ray) {
    const int MAX_DEPTH = 5;
    vec3 color = vec3(0);

    HitInfo bounces[MAX_DEPTH];

    for(int depth = 0; depth < MAX_DEPTH; depth++) {
        HitInfo closest_hit = trace_ray(ray);

        bounces[depth] = closest_hit;
        if(!closest_hit.background) {
            ray = Ray(closest_hit.position, reflect(ray.direction, closest_hit.normal));
        }
    }

    for(int i = 0; i < bounces.length(); i++) {
        if(bounces[i].background == false) {
            float luminence = dot(bounces[i].normal, normalize(vec3(1, 1, -1)));
            color = (color + bounces[i].material.albedo * max(luminence, 0.2)) / 2;
        } else {
            break;
        }
    }

    return color;
}

void main() {

    vec2 fragCoord = gl_FragCoord.xy;
    vec2 uv = fragCoord - vec2(width / 2.0, height / 2.0);
    float focal_length = width / (2.0 * tan(camera.fov * PI / 360.0));

    Ray ray = Ray(camera.position, rotate(normalize(vec3(uv.x, uv.y, focal_length)), camera.angles));

    vec3 color = calculate_color(ray);

    gl_FragColor = vec4(color, 1.0);
}