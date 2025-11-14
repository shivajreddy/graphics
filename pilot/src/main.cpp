#include <glad/glad.h> // must be included before glfw

#include <GLFW/glfw3.h>
#include <iostream>

void resize_callback(GLFWwindow* window, int w, int h) {
    std::cout << window << " is resized" << std::endl;
    glViewport(0, 0, w, h);
};

void process_input(GLFWwindow* window) {
    if (glfwGetKey(window, GLFW_KEY_Q) == GLFW_PRESS) {
        glfwSetWindowShouldClose(window, 1);
    }
}

const char *vertex_shader_source = "#version 330 core\n"
    "layout (location = 0) in vec3 aPos;\n"
    "void main()\n"
    "{\n"
    "    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);\n"
    "}\0";

const char *fragment_shader_source = "#version 330 core\n"
    "out vec4 frag_color;\n"
    "void main()\n"
    "{\n"
    "    frag_color = vec4(1.0f, 0.5f, 0.2f, 1.0f);\n"
    "}\0";
    

int main() {
    std::cout << "Hello OpenGL" << std::endl;

    glfwInit(); // Create GL context

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);                 // OpenGL 3.3
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE); // Core


    GLFWwindow* window = glfwCreateWindow(800, 600, "OpenGL", NULL, NULL);
    glfwMakeContextCurrent(window);

    // Initialize GLAD - load OpenGL function pointers
    if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) {
        std::cerr << "Failed to initialize GLAD" << std::endl;
        return -1;
    }

    glViewport(0, 0, 800, 600);
    glfwSetFramebufferSizeCallback(window, resize_callback);

    // Render loop
    while (!glfwWindowShouldClose(window)) {
        // keyboard input
        process_input(window);

        // configure which color that glClear should clear the screen with
        glClearColor(0.2f, 0.3f, 0.3f, 1.0f); // modifies gl state
        // utilizes the state
        glClear(GL_COLOR_BUFFER_BIT); // should pass the buffer to clear
        glClear(GL_DEPTH_BUFFER_BIT);
        glClear(GL_STENCIL_BUFFER_BIT);

        // render
        // triangle vertices
        float vertices[] = {
            -0.5f, -0.5f, 0.0f, 0.5f, -0.5f, 0.0f, 0.0f, -0.5f, 0.0f,
        };

        // Create VBO(Virtual Buffer Object)
        unsigned int VBO1;
        glGenBuffers(1, &VBO1);
        // there are different kinds of buffers: GL_ARRAY_BUFFER, others...
        glBindBuffer(GL_ARRAY_BUFFER, VBO1); // bind our VBO to ARRAY_BUFFER
        // from now on, any buffer calls on ARRAY_BUFFER will be used to configure our
        // VBO1 since it was bounded to the ARRAY_BUFFER

        // copy user data into the currently bound buffer
        glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

        // Check and call events and swap the buffers
        glfwPollEvents();
        glfwSwapBuffers(window);

        // Create a vertex shader
        unsigned int vertex_shader;
        vertex_shader = glCreateShader(GL_VERTEX_SHADER);

        // attach the shader source code to the shader object & compile it
        glShaderSource(vertex_shader, 1, &vertex_shader_source, nullptr);
        glCompileShader(vertex_shader);

        // check if the vertex shader compiled successfully
        int success;
        char info_log[512];
        glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &success);
        if (!success){
            glGetShaderInfoLog(vertex_shader, 512, nullptr, info_log);
            std::cout<< "ERROR::SHADER::VERTEX::COMPILATION_FAIL\n" << info_log << std::endl;
        }

        // Fragment Shader: calculate the color output of pixels
        unsigned int fragment_shader;
        fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(fragment_shader, 1, &fragment_shader_source, nullptr);
        glCompileShader(fragment_shader);

        // check if fragment shader compiled
        glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &success);
        if (!success){
            glGetShaderInfoLog(success, 512, nullptr, info_log);
            std::cout << "ERROR::SHADER::FRAGMENT::COMPILATION_FAIL\n" << info_log << std::endl;
        }

        // Link the vertex-shader & fragment-shader into a shader program
        unsigned int shader_program;
        shader_program = glCreateProgram();

        glAttachShader(shader_program, vertex_shader);
        glAttachShader(shader_program, fragment_shader);
        glLinkProgram(shader_program);

        glGetProgramiv(shader_program, GL_LINK_STATUS, &success);
        if(!success){
            glGetProgramInfoLog(shader_program, 512, nullptr, info_log);
            std::cout << "ERROR::SHADER::PROGRAM::LINKING_FAIL\n" << info_log << std::endl;
        }

        glUseProgram(shader_program);

        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);

    }

    glfwTerminate();
    return 0;
}
