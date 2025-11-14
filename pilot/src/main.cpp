#include <glad/glad.h> // must be included before glfw

#include <GLFW/glfw3.h>
#include <iostream>

void resize_callback(GLFWwindow* window, int w, int h) {
    std::cout << window << " is resized" << std::endl;
    glViewport(0, 0, w, h);
};

void process_input(GLFWwindow* window) {
    // if (glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS) {
    if (glfwGetKey(window, GLFW_KEY_Q) == GLFW_PRESS) {
        glfwSetWindowShouldClose(window, 1);
    }
}

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
        // Triangle
        float vertices[] = {
            -0.5f, -0.5f, 0.0f, 0.5f, -0.5f, 0.0f, 0.0f, -0.5f, 0.0f,
        };

        // Check and call events and swap the buffers
        glfwPollEvents();
        glfwSwapBuffers(window);
    }

    glfwTerminate();
    return 0;
}
