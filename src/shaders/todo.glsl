// TODO: wgsl

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;
    vec2 unit = (uv*2.0) - vec2(1.0, 1.0);
    
    float speed = 6.0;
    float t = iTime * speed + sin(iTime * speed * 0.8);

    float fov = 0.2;
    vec2 view_dir = vec2(sin(t), cos(t));

    // Time varying pixel color
    vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));
    
    float ang = acos(dot(view_dir, unit) / (length(view_dir) * length(unit)));
    
    
    // If (Current angle within fov):
    // draw the thing
    // else:
    // regular
    
    //if (distance(unit, vec2(0.0, 0.0)) < 0.3 && (ang - fov > 0.0)) {
    if (ang - fov < 0.1 && distance(unit, vec2(0.0, 0.0)) < 0.5) {
        fragColor = vec4(0.0, 1.0, 0.5, 1.0);
        //fragColor = vec4(col,1.0);

    } else {
         fragColor = vec4(1.0, 1.0, 1.0, 1.0);
         //fragColor = vec4(col,1.0);
    }
}
