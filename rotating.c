#include <stdio.h>
#include <windows.h> // Replaced unistd.h with the Windows API header
#include "projectingAndRotating.h"

int main() {
    // 1. Clear the screen and the scrollback buffer completely
    printf("\033[2J\033[3J\033[H");
    fflush(stdout);

    // Terminal center coordinates
    double centerX = 64.0;
    double centerY = 18.0;

    // The starting point of our 'o' relative to the center
    double radiusX = 15.0;
    double radiusY = 0.0;

    double angle = 0.0;

    // Infinite animation loop
    while (1) {
        // 2. Rotate the starting point by the current angle
        Coords2d rotated = Rotate2D(radiusX, radiusY, angle);

        // 3. Shift coordinates to the center. 
        // Added the * 2.0 multiplier to fix the terminal's tall font aspect ratio!
        int screenX = (int)(centerX + (rotated.x * 2.0));
        int screenY = (int)(centerY + rotated.y);

        // 4. Move cursor to position and print 'o'
        printf("\033[%d;%dH", screenY, screenX);
        putchar('o');
        fflush(stdout);

        // 5. Windows Sleep uses milliseconds (33ms is approx 30 frames per second)
        Sleep(33); 

        // 6. Erase the 'o' we just drew
        /**printf("\033[%d;%dH", screenY, screenX);
        putchar(' ');
        fflush(stdout);**/

        // 7. Increment the angle for the next frame
        angle += DegreeToRadian(5);
        if (angle >= 2 * M_PI) {
            angle -= 2 * M_PI;
        }
    }

    return 0;
}