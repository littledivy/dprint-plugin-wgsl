struct VertexInput {
  @location(0) position: vec2f,
  @builtin(instance_index) instance: u32
};

struct VertexOutput {
  @builtin(position) position: vec4f,
  @location(1) @interpolate(flat) instance: u32,
};

struct Rectangle {
  color: vec4f,
  position: vec2f,
  _unused: f32,
  sigma: f32,
  size: vec2f,
  window: vec2f,
};

struct UniformStorage {
  rectangles: array<Rectangle>,
};

@group(0) @binding(0) var<storage> data: UniformStorage;

@vertex
fn vertexMain(input: VertexInput) -> VertexOutput {
  var output: VertexOutput;
  let r = data.rectangles[input.instance];
  let padding = 3.0 * r.sigma;
  let vertex = mix(
    r.position.xy - padding,
    r.position.xy + r.size + padding,
    input.position
  );

  output.position = vec4f(vertex / r.window * 2.0 - 1.0, 0.0, 1.0);
  output.position.y = -output.position.y;
  output.instance = input.instance;
  return output;
}

@fragment
fn fragmentMain(input: VertexOutput) -> @location(0) vec4f {
  let r = data.rectangles[input.instance];

  let alpha = r.color.a;
  return vec4f(r.color.rgb, alpha);
}
