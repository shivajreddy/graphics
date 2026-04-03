# Graphics Learning Plan
## Goal: Learn OpenGL fundamentals in Rust → apply to wgpu → build 3D CAD software

---

## Context and Why This Path

You are intermediate in Rust but have zero graphics background. You started with wgpu
and found it overwhelming. That is expected: wgpu maps almost 1-to-1 to Vulkan, which
is an explicit GPU API designed for expert users. It exposes every low-level detail —
pipelines, command buffers, bind groups, render passes — with no hand-holding.

The correct path is to first build a mental model using OpenGL, which hides most of
that complexity while teaching you the same conceptual pipeline that wgpu exposes.
Once you understand what a VAO, VBO, shader program, and draw call actually mean,
wgpu stops being confusing and becomes just "more verbose OpenGL."

This document is your complete guide for the OpenGL phase. You write the code.
This doc tells you exactly what to do, in what order, why, and with what tools.

---

## Rust Crate Stack (OpenGL phase)

You will NOT use C or C++ at any point. Everything is Rust.

| Role | Crate | Version | What it does |
|---|---|---|---|
| Window + event loop | `glfw` | 0.62 | Wraps the GLFW C library. Creates an OS window and an OpenGL context. Handles keyboard/mouse events. |
| OpenGL function loader | `gl` | 0.14 | Generates Rust bindings for every OpenGL function. You call `gl::DrawArrays(...)` instead of C's `glDrawArrays(...)`. |
| Math | `glam` | 0.29 | Vectors, matrices, quaternions. Fast, ergonomic, used in wgpu ecosystem too. You will use this same crate when you transition to wgpu. |
| Image loading | `image` | 0.25 | Load PNG/JPG files for texture chapters. |

### Why glfw and not winit?

winit is the correct long-term choice (it is what wgpu uses). But winit does not manage
an OpenGL context — you need a separate crate like `glutin` for that, and glutin's API
has changed substantially across versions. glfw does both (window + OpenGL context) in
one clean API, which makes the learning setup simpler. You will switch to winit when
you move back to wgpu.

### Why gl and not another binding?

The `gl` crate is a generated binding maintained by the same team as winit/glutin. It
is the standard. Functions map directly 1-to-1 with the learnopengl.com code, just with
`gl::` prefix and Rust types.

---

## Cargo.toml setup

```toml
[package]
name = "opengl-learning"
version = "0.1.0"
edition = "2021"

[dependencies]
glfw = "0.62"
gl = "0.14"
glam = "0.29"
image = "0.25"
```

On macOS you need to link against system OpenGL. Add this to your build or just let
glfw handle it — it does automatically on macOS. No build.rs needed.

On Linux you may need: `sudo apt install libglfw3-dev` or equivalent.

---

## The Mental Model (Read Before Writing Any Code)

Before chapter 1, understand this pipeline. Every chapter is just filling in one piece.

```
Your Rust code (CPU)
        |
        | uploads data to GPU memory
        v
  Vertex Buffer (VBO)
        |
        | GPU reads vertices
        v
  Vertex Shader (GLSL, runs on GPU per-vertex)
        |
        | outputs clip-space positions
        v
  Rasterizer (hardware, converts triangles to fragments/pixels)
        |
        v
  Fragment Shader (GLSL, runs on GPU per-pixel)
        |
        | outputs a color
        v
  Framebuffer (the screen, or an off-screen texture)
```

Everything in OpenGL — VAOs, VBOs, uniforms, textures — is just a way to get data
from your CPU code into one of the stages above, or to configure how a stage runs.

OpenGL is a state machine. You "bind" objects to slots, then issue draw calls, and
OpenGL uses whatever is currently bound. This is why code looks like:
bind thing → set its options → draw → unbind. That pattern never changes.

---

## Learning Chapters

Each chapter below maps to a learnopengl.com section. For each:
- Read the full learnopengl.com page first (conceptual explanation)
- Then write the Rust equivalent (not C)
- The Rust translation notes are provided for every chapter

---

### Chapter 0 — Read Only (no code)
**URL:** https://learnopengl.com/Getting-started/OpenGL

Read this. It explains:
- OpenGL is a specification, not a library. GPU drivers implement it.
- "Core profile" vs "immediate mode" — you are using core profile (modern OpenGL).
- OpenGL is a state machine.
- Objects in OpenGL: create → bind → configure → unbind pattern.

No code. Just read. Estimated time: 20 minutes.

---

### Chapter 1 — Hello Window
**URL:** https://learnopengl.com/Getting-started/Hello-Window

**Goal:** Open a window with an OpenGL context. Clear it to a color. Close on Escape.

**What you are learning:**
- How glfw creates a window and an OpenGL context
- The event loop: poll events → update → render → swap buffers
- How to load OpenGL function pointers with `gl::load_with`
- `gl::ClearColor` and `gl::Clear`

**Rust translation notes:**

The C code uses `glfwInit()` and `glfwCreateWindow()`. In Rust with the glfw crate:

```rust
use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    // macOS only:
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(800, 600, "Hello OpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);

    // Load all OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    while !window.should_close() {
        // process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true);
            }
        }

        // render
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
    }
}
```

**Key concepts to understand before moving on:**
- `make_current()` — tells the OS this window's OpenGL context is active on this thread
- `gl::load_with` — OpenGL functions are not statically linked; you load them at runtime from the driver. This is why you call this AFTER creating the context.
- `swap_buffers()` — OpenGL renders to a back buffer; this displays it (double buffering)
- All gl:: calls that touch GPU state are `unsafe` in Rust

**Do not proceed until:** you have a window that opens, shows a teal/dark color, and closes when you press Escape.

---

### Chapter 2 — Hello Triangle
**URL:** https://learnopengl.com/Getting-started/Hello-Triangle

**Goal:** Draw a colored triangle on the screen.

**This is the most important chapter.** Everything else builds on it.

**What you are learning:**
- VBO (Vertex Buffer Object): a chunk of GPU memory that holds your vertex data
- VAO (Vertex Array Object): records how to interpret the data in the VBO
- Vertex shader: a small GPU program that positions each vertex
- Fragment shader: a small GPU program that colors each pixel
- Shader program: vertex shader + fragment shader compiled and linked together
- Draw call: `gl::DrawArrays` — tells the GPU to draw using bound state

**The sequence every draw call requires:**
1. Compile vertex shader source → get shader ID
2. Compile fragment shader source → get shader ID  
3. Link both into a shader program → get program ID
4. Create VBO → upload vertex float array to GPU
5. Create VAO → bind it → tell OpenGL the layout of the vertex data
6. In render loop: use program → bind VAO → DrawArrays

**Rust translation notes:**

GLSL shaders are strings in your Rust code (or loaded from files):

```rust
let vertex_shader_source = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

let fragment_shader_source = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0, 0.5, 0.2, 1.0);
    }
"#;
```

Compiling a shader in Rust (the C code uses `glShaderSource`, which takes char**):

```rust
unsafe fn compile_shader(source: &str, shader_type: gl::types::GLenum) -> u32 {
    let shader = gl::CreateShader(shader_type);
    let c_str = std::ffi::CString::new(source).unwrap();
    gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
    gl::CompileShader(shader);

    // check for errors
    let mut success = 0i32;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success == 0 {
        let mut log = vec![0u8; 512];
        gl::GetShaderInfoLog(shader, 512, std::ptr::null_mut(), log.as_mut_ptr() as *mut i8);
        eprintln!("Shader compile error: {}", String::from_utf8_lossy(&log));
    }
    shader
}
```

The key Rust-specific issue: `gl::ShaderSource` takes a `*const *const i8`. You must
use `CString` to create a null-terminated C string, then pass a pointer to that pointer.
This is the ugliest part of raw OpenGL in Rust. You will only write it once.

Uploading vertex data:

```rust
let vertices: [f32; 9] = [
    -0.5, -0.5, 0.0,
     0.5, -0.5, 0.0,
     0.0,  0.5, 0.0,
];

let (mut vao, mut vbo) = (0u32, 0u32);
gl::GenVertexArrays(1, &mut vao);
gl::GenBuffers(1, &mut vbo);

gl::BindVertexArray(vao);
gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
gl::BufferData(
    gl::ARRAY_BUFFER,
    (vertices.len() * std::mem::size_of::<f32>()) as isize,
    vertices.as_ptr() as *const _,
    gl::STATIC_DRAW,
);

// tell OpenGL how to interpret the vertex data
gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 12, std::ptr::null());
gl::EnableVertexAttribArray(0);
```

`VertexAttribPointer` arguments: attribute index (0), size (3 floats), type, normalized,
stride (3 floats * 4 bytes = 12), offset (0 = null pointer). This is what a VAO records.

**Do not proceed until:** you see a triangle on screen.

---

### Chapter 3 — Shaders
**URL:** https://learnopengl.com/Getting-started/Shaders

**Goal:** Understand GLSL more deeply. Pass data from CPU to GPU via uniforms. Pass
data between vertex and fragment shader via varying variables.

**What you are learning:**
- GLSL types: vec2, vec3, vec4, mat4, sampler2D, etc.
- `uniform` variables: values you set from Rust code, same for all vertices/fragments
- `in`/`out` variables: pass data between shader stages
- Shader struct: encapsulate compile + link into a reusable Rust struct

**Rust translation notes:**

Setting a uniform from Rust:

```rust
// after gl::UseProgram(program):
let name = std::ffi::CString::new("ourColor").unwrap();
let location = gl::GetUniformLocation(program, name.as_ptr());
gl::Uniform4f(location, 0.0, green_value, 0.0, 1.0);
```

Build a `Shader` struct now. You will use it for all remaining chapters:

```rust
pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_src: &str, fragment_src: &str) -> Self { ... }
    pub fn use_program(&self) { unsafe { gl::UseProgram(self.id); } }
    pub fn set_float(&self, name: &str, value: f32) { ... }
    pub fn set_vec3(&self, name: &str, v: glam::Vec3) { ... }
    pub fn set_mat4(&self, name: &str, m: &glam::Mat4) { ... }
}
```

**Do not proceed until:** you can change a triangle's color from Rust code using a uniform.

---

### Chapter 4 — Textures
**URL:** https://learnopengl.com/Getting-started/Textures

**Goal:** Apply an image as a texture to a rectangle (two triangles).

**What you are learning:**
- Texture objects: upload image data to GPU memory
- UV coordinates: how to map a 2D image onto 3D geometry
- Texture sampling in GLSL: `sampler2D` uniform, `texture()` function
- Texture parameters: wrapping mode, filtering (nearest vs linear, mipmaps)

**Rust translation notes:**

Loading an image with the `image` crate:

```rust
let img = image::open("texture.png").unwrap().flipv();
let (width, height) = img.dimensions();
let data = img.into_rgb8();

let mut texture = 0u32;
gl::GenTextures(1, &mut texture);
gl::BindTexture(gl::TEXTURE_2D, texture);
gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
gl::TexImage2D(
    gl::TEXTURE_2D, 0, gl::RGB as i32,
    width as i32, height as i32, 0,
    gl::RGB, gl::UNSIGNED_BYTE,
    data.as_ptr() as *const _,
);
gl::GenerateMipmap(gl::TEXTURE_2D);
```

Note `flipv()` — OpenGL's texture coordinate (0,0) is bottom-left but images typically
store top-left first. Flip vertically when loading.

**Do not proceed until:** you can render a textured rectangle.

---

### Chapter 5 — Transformations
**URL:** https://learnopengl.com/Getting-started/Transformations

**Goal:** Move, rotate, and scale objects using transformation matrices.

**What you are learning:**
- Why matrices: transformations compose by multiplication
- Model matrix: positions an object in the world
- Translation, rotation, scale as 4x4 matrices
- Passing a mat4 to a shader via uniform

**Rust translation notes:**

The learnopengl.com code uses the GLM library (C++). In Rust you use `glam`. The API
is nearly identical.

```rust
use glam::{Mat4, Vec3, Quat};

// translation
let model = Mat4::from_translation(Vec3::new(0.5, -0.5, 0.0));

// rotation + scale
let model = Mat4::from_scale_rotation_translation(
    Vec3::splat(1.0),                                    // scale
    Quat::from_rotation_z(time.elapsed().as_secs_f32()), // rotation
    Vec3::new(0.5, -0.5, 0.0),                           // translation
);
```

Passing to shader:

```rust
shader.set_mat4("transform", &model);

// inside set_mat4:
let loc = gl::GetUniformLocation(self.id, c_name.as_ptr());
gl::UniformMatrix4fv(loc, 1, gl::FALSE, m.as_ref().as_ptr());
```

**Do not proceed until:** you can animate a rotating/scaling textured rectangle.

---

### Chapter 6 — Coordinate Systems
**URL:** https://learnopengl.com/Getting-started/Coordinate-Systems

**Goal:** Render a 3D scene by understanding model/view/projection matrices.

**This is a critical chapter for CAD.** Every 3D CAD viewport is driven by these three matrices.

**What you are learning:**
- Model matrix: transforms object from local space to world space
- View matrix: transforms world space to camera space (where is the camera, where is it looking)
- Projection matrix: transforms camera space to clip space (perspective or orthographic)
- MVP: multiply Model * View * Projection in the vertex shader
- Depth buffer: prevents back faces from drawing over front faces

```glsl
// vertex shader
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}
```

**Rust with glam:**

```rust
// perspective projection (for 3D CAD viewport)
let projection = Mat4::perspective_rh_gl(
    45f32.to_radians(), // field of view
    800.0 / 600.0,      // aspect ratio
    0.1,                // near plane
    100.0,              // far plane
);

// camera looking at origin from z=3
let view = Mat4::look_at_rh(
    Vec3::new(0.0, 0.0, 3.0),  // eye position
    Vec3::ZERO,                  // target
    Vec3::Y,                     // up vector
);

// object at origin
let model = Mat4::IDENTITY;
```

Note: `glam` uses right-handed coordinate systems and provides `_rh_gl` variants for
OpenGL's clip space conventions. When you move to wgpu you will use `_rh` (wgpu/Vulkan
use a different NDC convention). This is one of the real things to watch for at transition.

Enabling the depth buffer:

```rust
// once at startup
gl::Enable(gl::DEPTH_TEST);

// in render loop, clear both color AND depth each frame
gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
```

**Do not proceed until:** you can render multiple 3D cubes at different positions that
correctly occlude each other.

---

### Chapter 7 — Camera
**URL:** https://learnopengl.com/Getting-started/Camera

**Goal:** Implement a fly camera controlled by keyboard and mouse.

**What you are learning:**
- Camera position, front direction, up vector
- View matrix from these three vectors (`look_at`)
- Mouse delta → rotation (yaw and pitch angles)
- WASD keyboard movement relative to camera orientation
- Scroll wheel → zoom (change FOV)

**This is directly applicable to CAD.** A CAD viewport camera is this camera plus orbit
controls. Once you build a fly camera, orbit mode is a small modification.

**Camera struct in Rust:**

```rust
pub struct Camera {
    pub position: glam::Vec3,
    pub yaw: f32,   // horizontal rotation, degrees
    pub pitch: f32, // vertical rotation, degrees
    pub fov: f32,
}

impl Camera {
    pub fn view_matrix(&self) -> glam::Mat4 {
        let front = self.front();
        glam::Mat4::look_at_rh(self.position, self.position + front, glam::Vec3::Y)
    }

    pub fn front(&self) -> glam::Vec3 {
        glam::Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ).normalize()
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) { ... }
    pub fn process_mouse(&mut self, x_offset: f32, y_offset: f32) { ... }
    pub fn process_scroll(&mut self, y_offset: f32) { ... }
}
```

Capturing mouse movement in glfw:

```rust
window.set_cursor_mode(glfw::CursorMode::Disabled);
window.set_cursor_pos_polling(true);

// in event loop:
glfw::WindowEvent::CursorPos(x, y) => {
    let x_offset = x as f32 - last_x;
    let y_offset = last_y - y as f32; // inverted: y goes down in screen space
    camera.process_mouse(x_offset, y_offset);
}
```

**Do not proceed until:** you can fly through a scene of 3D cubes with mouse look and
WASD movement.

---

## What You Have After Chapter 7

At the end of chapter 7, you have built:
- A window with an OpenGL context and event loop
- A shader system that loads, compiles, and uses vertex + fragment shaders
- The ability to upload geometry (VBO/VAO) and textures to the GPU
- The MVP matrix pipeline (model/view/projection)
- A working 3D camera

This is everything you need to understand wgpu. Every single concept maps directly:

| OpenGL concept | wgpu equivalent |
|---|---|
| VBO + VAO | `wgpu::Buffer` + `VertexBufferLayout` |
| Shader program | `wgpu::ShaderModule` + `wgpu::RenderPipeline` |
| Uniform | `wgpu::Buffer` (uniform) + `wgpu::BindGroup` |
| Texture | `wgpu::Texture` + `wgpu::TextureView` + `wgpu::Sampler` |
| Framebuffer / swap | `wgpu::Surface` + `SurfaceTexture` |
| `gl::DrawArrays` | `render_pass.draw(...)` |
| Bind texture | `render_pass.set_bind_group(...)` |
| `gl::Enable(DEPTH_TEST)` | depth stencil state in `RenderPipelineDescriptor` |

The wgpu confusion you had before came from not knowing what these things ARE.
After chapter 7, you will.

---

## Optional Chapters (if you want more before moving to wgpu)

These are useful but not required before the wgpu transition:

| Chapter | URL | Why useful for CAD |
|---|---|---|
| Basic Lighting | https://learnopengl.com/Lighting/Basic-Lighting | CAD viewports need basic diffuse/specular shading to show 3D shape |
| Depth testing | https://learnopengl.com/Advanced-OpenGL/Depth-testing | How the depth buffer works in detail |
| Face culling | https://learnopengl.com/Advanced-OpenGL/Face-culling | Only render front-facing polygons — important for solid model rendering |
| Framebuffers | https://learnopengl.com/Advanced-OpenGL/Framebuffers | Off-screen rendering — needed for selection highlighting and picking in CAD |

---

## After OpenGL: The wgpu Transition

When you return to learn-wgpu (https://sotrh.github.io/learn-wgpu/), here is the
mapping to be aware of:

**Windowing:** learn-wgpu uses winit + wgpu directly. winit is like glfw but without
the OpenGL context part (wgpu handles the GPU context itself). The event loop pattern
is nearly identical.

**NDC convention:** OpenGL uses [-1, 1] for the Z clip range. wgpu (Vulkan/Metal/DX12)
uses [0, 1]. When you port your projection matrix from glam, switch from
`Mat4::perspective_rh_gl(...)` to `Mat4::perspective_rh(...)`.

**Coordinate system:** Both use right-handed coordinates. No change needed.

**Explicit state:** wgpu requires you to declare everything upfront in a
`RenderPipelineDescriptor` (vertex layout, depth test config, blend state, etc.) rather
than setting state piecemeal. This is wgpu's verbosity — it is not fundamentally different
from what you did in OpenGL, just declared differently.

---

## After wgpu: CAD-Specific Stack

Once you have the wgpu mental model, the CAD-specific work begins:

| Concern | Crate | Notes |
|---|---|---|
| Geometry kernel (B-rep solids) | `truck` | Rust CAD kernel. Handles surfaces, solids, boolean ops. |
| Curve/path tessellation | `lyon` | Converts curves to triangles for GPU rendering. |
| UI panels, toolbars | `egui` with `egui-wgpu` | Integrates cleanly into wgpu render loop. |
| Math (already using this) | `glam` | Same crate throughout — no switching. |
| 3D math reference | https://gamemath.com | Free book. Vectors, matrices, transforms, projections. Read alongside coding. |

---

## Reference Links

| Resource | URL | Use |
|---|---|---|
| learnopengl.com | https://learnopengl.com | Primary learning material. Read every page. |
| learn-wgpu | https://sotrh.github.io/learn-wgpu/ | Return here after chapter 7 |
| gl crate docs | https://docs.rs/gl/latest/gl/ | OpenGL bindings API reference |
| glfw crate docs | https://docs.rs/glfw/latest/glfw/ | Window/context API reference |
| glam crate docs | https://docs.rs/glam/latest/glam/ | Math library reference |
| glam cheatsheet | https://github.com/bitshifter/glam-rs | README has usage examples |
| 3D Math Primer | https://gamemath.com | Free book on vectors/matrices/transforms |
| wgpu docs | https://docs.rs/wgpu/latest/wgpu/ | For when you transition back |
| truck (CAD kernel) | https://github.com/ricosjp/truck | Rust B-rep geometry kernel |

---

## Checklist

- [ ] Chapter 0: Read learnopengl.com/Getting-started/OpenGL
- [ ] Set up Cargo project with glfw, gl, glam, image
- [ ] Chapter 1: Window opens and clears to a color
- [ ] Chapter 2: Triangle renders on screen
- [ ] Chapter 3: Shader struct built, uniform colors working
- [ ] Chapter 4: Textured rectangle renders
- [ ] Chapter 5: Animated rotation/scale working with glam matrices
- [ ] Chapter 6: Multiple 3D cubes with depth testing
- [ ] Chapter 7: Fly camera with mouse look and WASD
- [ ] (Optional) Basic lighting chapter
- [ ] Return to learn-wgpu with new mental model
- [ ] Port camera and geometry code to wgpu
- [ ] Integrate egui for UI
- [ ] Integrate truck for geometry kernel
