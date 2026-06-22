#include<stdio.h>
#include"projectingAndRotating.h"
#include<windows.h>

void moveCursor(int x, int y) {
    // ANSI escape code: \033[Y;XH moves the cursor to row Y, column X
    // Note: Terminals usually treat 1,1 as the top-left corner
    printf("\033[%d;%dH", y, x);
}

void clearScreen() {
    // This clears the screen, clears the scrollback buffer, and moves cursor to 1,1
    printf("\033[2J\033[3J\033[H");
    fflush(stdout);
}

int main () {
    double r = DegreeToRadian(10);
    Coords3d R = Rotate3DInX(5, 6, 5, 45);
    printf("%lf  %lf  %lf\n", R.x, R.y, R.z);
    Coords2d P = project(R.x, R.y, R.z, 25);
    Coords2d newCoord;
    clearScreen();
    moveCursor((int)P.x + 64, (int)P.y + 18);
    printf("O\n");
    newCoord = P;
    while(1) {
        //clearScreen();
        newCoord = Rotate2D(newCoord.x, newCoord.y, r);
        moveCursor((int)(newCoord.x * 2 + 64), (int)(newCoord.y + 18));
        printf("o\n");
        Sleep(500);
    }
    printf("%d  %d\n", (int)P.x, (int)P.y);
    return 0;
}
