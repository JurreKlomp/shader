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
uniform Sphere spheres[2];

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

void main() {

    vec2 fragCoord = gl_FragCoord.xy;
    vec2 uv = fragCoord - vec2(width / 2.0, height / 2.0);
    float focal_length = width / (2.0 * tan(camera.fov * PI / 360.0));


    Ray ray = Ray(camera.position, rotate(normalize(vec3(uv.x, uv.y, focal_length)), camera.angles));

    vec3 color = normalize(ray.direction);

    const int MAX_DEPTH = 1;
    HitInfo bounces[MAX_DEPTH];

    for(int depth = 0; depth < MAX_DEPTH; depth++) {
        HitInfo closest_hit = HitInfo(true, vec3(1), vec3(0), 0, Material(vec3(0), 0, 0));
        for(int i = 0; i < spheres.length(); i++) {
            Sphere sphere = spheres[i];

            vec3 relative = ray.origin - sphere.position;

            float b = 2.0 * dot(ray.direction, relative);
            float c = dot(relative, relative) - sphere.radius * sphere.radius;

            float d = b * b - 4.0 * c;

            if(d >= 0.0) {

                float t1 = (-b + sqrt(d)) / 2.0;
                float t2 = (-b - sqrt(d)) / 2.0;

            // float t = (t1 > 0.0 && (t1 < t2 || t2 <= 0.0)) ? t1 : t2;
                float t = -1.0;

                if(t1 >= 0.0 && t2 >= 0.0) {
                    t = min(t1, t2);
                } else if(t1 >= 0.0) {
                    t = t1;
                } else if(t2 >= 0.0) {
                    t = t2;
                }

                if(t >= 0.0 && (closest_hit.background || t < closest_hit.distance)) {

                    vec3 hit_position = relative + t * ray.direction;
                    vec3 normal = normalize(hit_position - sphere.position);

                    closest_hit = HitInfo(false, normal, hit_position, t, sphere.material);
                }
            }

        }
        if(!closest_hit.background) {

            // ray = Ray(closest_hit.position, normalize(closest_hit.normal));
        }
        bounces[depth] = closest_hit;
    }

    for(int i = 0; i < bounces.length(); i++) {
        if(bounces[i].background == false) {
            color += bounces[i].material.albedo * max(dot(bounces[i].normal, normalize(vec3(1))), 0.2);
        } else {
            break;
        }
    }

    gl_FragColor = vec4(color, 1.0);
}