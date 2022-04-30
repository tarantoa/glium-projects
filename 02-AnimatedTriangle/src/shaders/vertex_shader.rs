pub const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec2 position;
    uniform float t;

    void main() {
        vec2 pos = position;
        pos.x += t;
        gl_Position = vec4(pos, 0.0, 1.0);
    }
"#;