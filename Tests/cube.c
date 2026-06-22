#include <stdio.h>
#include <windows.h>
#include "projectingAndRotating.h"

// Define the 8 vertices of a cube centered at (0,0,0)
// The cube has a side length of 20 units (-10 to 10)
Coords3d cubeVertices[8] = {
    {-10, -10, -10}, {10, -10, -10}, {10, 10, -10}, {-10, 10, -10},
    {-10, -10,  10}, {10, -10,  10}, {10, 10,  10}, {-10, 10,  10}
};

// Index pairs that define the 12 edges of a cube
int cubeEdges[12][2] = {
    {0, 1}, {1, 2}, {2, 3}, {3, 0}, // Back face edges
    {4, 5}, {5, 6}, {6, 7}, {7, 4}, // Front face edges
    {0, 4}, {1, 5}, {2, 6}, {3, 7}  // Connecting edges
};

int main() {
    // Canvas dimensions (your custom resolution)
    double centerX = 64.0;
    double centerY = 18.0;

    // Focal length / Camera distance
    // F is the viewing point. Z must be smaller than F.
    // If the cube is at Z=0, keeping F=60 means it's 60 units away from the camera.
    double F = 60.0; 

    double angleX = 0.0;
    double angleY = 0.0;

    // Array to store projected 2D points for rendering each frame
    // We will draw the 8 vertices + points along the edges
    int totalPointsToDraw = 8 + (12 * 5); // 8 vertices + 5 points per edge
    Coords2d pointsToDraw[100]; 

    // Hide cursor for a cleaner look
    printf("\033[?25l");

    while (1) {
        // 1. Clear the screen and buffer
        printf("\033[2J\033[3J\033[H");

        int pointIndex = 0;

        // 2. Rotate and project the 8 main vertices
        Coords3d rotatedVertices[8];
        for (int i = 0; i < 8; i++) {
            // Apply 3D rotation on multiple axes to make it spin dynamically
            Coords3d r = Rotate3DInX(cubeVertices[i].x, cubeVertices[i].y, cubeVertices[i].z, angleX);
            r = Rotate3DInY(r.x, r.y, r.z, angleY);
            rotatedVertices[i] = r;

            // Project 3D to 2D
            pointsToDraw[pointIndex++] = project(r.x, r.y, r.z, F);
        }

        // 3. Generate intermediate points along the edges to "draw" the lines
        for (int i = 0; i < 12; i++) {
            Coords3d p1 = rotatedVertices[cubeEdges[i][0]];
            Coords3d p2 = rotatedVertices[cubeEdges[i][1]];

            // Put 5 points between vertex 1 and vertex 2
            for (int j = 1; j <= 5; j++) {
                double t = j / 6.0; // Interpolation factor
                double ix = p1.x + (p2.x - p1.x) * t;
                double iy = p1.y + (p2.y - p1.y) * t;
                double iz = p1.z + (p2.z - p1.z) * t;

                pointsToDraw[pointIndex++] = project(ix, iy, iz, F);
            }
        }

        // 4. Render all points to your 128 x 36 terminal screen
    for (int i = 0; i < pointIndex; i++) {
        // Scaled down from 2.5 to 1.3 to make the cube smaller on screen
        double scale = 1.0; 

        int screenX = (int)(centerX + (pointsToDraw[i].x * 2.0 * scale));
        int screenY = (int)(centerY + (pointsToDraw[i].y * scale));

        // Boundary check to ensure we don't print off the screen
        if (screenX > 0 && screenX < 128 && screenY > 0 && screenY < 36) {
            printf("\033[%d;%dHo", screenY, screenX);
        }
    }

        fflush(stdout);

        // 5. Adjust rotation speeds for a mesmerizing tumbling effect
        angleX += 0.03;
        angleY += 0.04;

        Sleep(10); // Control frame rate (~25 FPS)
    }

    return 0;
}
