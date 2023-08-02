in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoord;

struct Material {
	vec3 ambient;
	vec3 diffuse;
	vec3 specular;
	float specular_power;
	float specular_intensity;
}; 

struct BaseLight
{
	vec3 color;
	float ambient_intensity;
	float diffuse_intensity;
};

struct DirectionalLight
{
	BaseLight base;
	vec3 direction;
};

struct Attenuation
{
	float constant;
	float linear;
	float exp_;
};

struct PointLight
{
	DirectionalLight base;
	Attenuation attenuation;
	vec3 position;
};

struct SpotLight
{
	PointLight base;
	float cut_off;
};

uniform int debug;
uniform float time;
uniform DirectionalLight directional_light;

#define MAX_POINT_LIGHTS 5
uniform int point_light_number;
uniform PointLight point_lights[MAX_POINT_LIGHTS];

#define MAX_SPOT_LIGHTS 5
uniform int spot_light_number;
uniform SpotLight spot_lights[MAX_SPOT_LIGHTS];

uniform Material material;
uniform vec3 viewPos;
uniform vec3 objectcolor;

uniform sampler2D ambientMap;
uniform sampler2D diffuseMap;
uniform sampler2D specularMap;
uniform sampler2D normalMap;

out vec4 Fragcolor;

vec4 calc_light(BaseLight base, vec3 direction, vec3 normal)
{
	vec4 diffuse = vec4(0.);
	vec4 specular = vec4(0.);
	
	// ambient
	vec4 ambient = vec4(base.ambient_intensity * base.color, 1.)
		* (texture2D(ambientMap, TexCoord) + vec4(material.ambient, 1.));
	
	float diffuse_factor = dot(normal, -direction);
	if (diffuse_factor > 0.)
	{
		// diffuse
		diffuse = vec4(base.color, 1.)
			* base.diffuse_intensity
			* diffuse_factor
			* (texture2D(diffuseMap, TexCoord) + vec4(material.diffuse, 1.));

		// specular
		vec3 vertex_to_eye = normalize(viewPos - FragPos);
		vec3 light_reflect = normalize(reflect(direction, normal));
		float specular_factor = dot(vertex_to_eye, light_reflect);
		if (specular_factor > 0.)
		{
			specular_factor = pow(specular_factor, material.specular_power);
			specular = vec4(base.color * material.specular_intensity * specular_factor, 1.0f)
				* (texture2D(specularMap, TexCoord) + vec4(material.specular, 1.));
		}
	}
	return (ambient + diffuse + specular);
}

vec4 calc_directional_light(vec3 normal)
{
	return (calc_light(directional_light.base, directional_light.direction, normal));
}

vec4 calc_point_light(PointLight l, vec3 normal)                                                 
{
	vec3 light_direction = FragPos - l.position;
	float dist = length(light_direction);
	light_direction = normalize(light_direction);

	vec4 color = calc_light(l.base.base, light_direction, normal);
	float attenuation_factor = l.attenuation.constant +
		l.attenuation.linear * dist +
		l.attenuation.exp_ * dist * dist;

	return (color / attenuation_factor);
}

vec4 calc_spot_light(SpotLight l, vec3 normal)
{
    vec3 light_to_pixel = normalize(FragPos - l.base.position);
	l.base.base.direction = vec3(cos(time * 0.5) * 2., -1., -1.);
    float spot_factor = dot(light_to_pixel, l.base.base.direction);
    if (spot_factor > l.cut_off)
	{
        vec4 color = calc_point_light(l.base, normal);
        return (color * (1.0 - (1.0 - spot_factor) * 1.0 / (1.0 - l.cut_off)));
    }
	return (vec4(0.));
}

void main()
{
	vec3 norm_normal = normalize(Normal);
	vec4 lights = calc_directional_light(norm_normal);

	for (int i = 0 ; i < point_light_number ; i++) {
		lights += calc_point_light(point_lights[i], norm_normal);
	}

	for (int i = 0 ; i < spot_light_number ; i++) {
		lights += calc_spot_light(spot_lights[i], norm_normal);
	}

	// final color
	vec4 result = lights;
	if (debug == 1 && FragPos.x > 0.)
		result = vec4(1.0, 0., 0., 1.);
	Fragcolor = vec4(result.rgb, 1.);
}
