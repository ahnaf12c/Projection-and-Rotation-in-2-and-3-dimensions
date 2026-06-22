#include <stdio.h>
#include <unistd.h> // Standard Unix header for sleep functions
#include "projectingAndRotating.h"

// Define the 8 vertices of a cube centered at (0,0,0)
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
    // Canvas dimensions (128 x 36)
    double centerX = 64.0;
    double centerY = 18.0;

    // Camera distance
    double F = 60.0; 

    double angleX = 0.0;
    double angleY = 0.0;

    int totalPointsToDraw = 8 + (12 * 5); 
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
            Coords3d r = Rotate3DInX(cubeVertices[i].x, cubeVertices[i].y, cubeVertices[i].z, angleX);
            r = Rotate3DInY(r.x, r.y, r.z, angleY);
            rotatedVertices[i] = r;

            pointsToDraw[pointIndex++] = project(r.x, r.y, r.z, F);
        }

        // 3. Generate intermediate points along the edges
        for (int i = 0; i < 12; i++) {
            Coords3d p1 = rotatedVertices[cubeEdges[i][0]];
            Coords3d p2 = rotatedVertices[cubeEdges[i][1]];

            for (int j = 1; j <= 5; j++) {
                double t = j / 6.0; 
                double ix = p1.x + (p2.x - p1.x) * t;
                double iy = p1.y + (p2.y - p1.y) * t;
                double iz = p1.z + (p2.z - p1.z) * t;

                pointsToDraw[pointIndex++] = project(ix, iy, iz, F);
            }
        }

        // 4. Render all points
        for (int i = 0; i < pointIndex; i++) {
            // Using your adjusted smaller scale factor (1.3)
            double scale = 1.3; 

            int screenX = (int)(centerX + (pointsToDraw[i].x * 2.0 * scale));
            int screenY = (int)(centerY + (pointsToDraw[i].y * scale));

            if (screenX > 0 && screenX < 128 && screenY > 0 && screenY < 36) {
                printf("\033[%d;%dHo", screenY, screenX);
            }
        }

        fflush(stdout);

        // 5. Adjust rotation speeds
        angleX += 0.03;
        angleY += 0.04;

        // Unix usleep uses microseconds (40,000 microseconds = 40 milliseconds)
        usleep(40000); 
    }

    return 0;
}
