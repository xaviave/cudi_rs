in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoord;

struct Light {
    vec3 position;
    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
}; 

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
}; 
  
uniform Light light;
uniform Material material;

uniform vec3 viewPos; 
uniform vec3 objectColor;

uniform sampler2D ambientMap;
uniform sampler2D diffuseMap;
uniform sampler2D specularMap;
uniform sampler2D normalMap;

out vec4 FragColor;

void main()
{
    // ambient
    // use texture2D
    vec3 ambient = light.ambient * vec3(texture(ambientMap, TexCoord));

    // diffuse 
    // vec3 norm = normalize(vec3(texture(normalMap, Nosrmal.xy)) * 2.0 - 1.0);
    // vec3 norm = normalize(Normal);
    // vec3 lightDir = normalize(light.position - FragPos);
    // float diff = max(dot(norm, lightDir), 0.0);
  	// vec3 diffuse = light.diffuse * diff * vec3(texture(diffuseMap, TexCoord));
    
    // specular
    // vec3 viewDir = normalize(viewPos - FragPos);
    // vec3 reflectDir = reflect(-lightDir, norm);  
    // float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
  	// vec3 specular = light.specular * spec * vec3(texture(specularMap, TexCoord));
    
    // final color
    // vec3 result = ambient + diffuse + specular + 0.2;
    FragColor = vec4(ambient + TexCoord.xyy, 1.0);
} 