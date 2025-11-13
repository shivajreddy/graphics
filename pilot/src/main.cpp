/* Do Vector Math
class FloatArray {
    float* data;
    size_t size;

public:
    FloatArray(size_t s) : size(s) {
        data = new float(s);
    }
    ~FloatArray() {
        delete[] data;
    }
};

int main(int argc, char* args[]) {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}
*/
#define GL_SILENCE_DEPRECATION // since macos deprecated opengl since 10.14
#include <glad/glad.h>         // must be included before glfw

#include <GLFW/glfw3.h>
#include <iostream>

int main() {
    std::cout << "Hello There" << std::endl;

    glfwInit();
    GLFWwindow* window = glfwCreateWindow(800, 600, "OpenGL", NULL, NULL);
    glfwMakeContextCurrent(window);

    // Initialize GLAD - load OpenGL function pointers
    if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) {
        std::cerr << "Failed to initialize GLAD" << std::endl;
        return -1;
    }

    while (!glfwWindowShouldClose(window)) {
        glClear(GL_COLOR_BUFFER_BIT);
        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    glfwTerminate();
    return 0;
}
