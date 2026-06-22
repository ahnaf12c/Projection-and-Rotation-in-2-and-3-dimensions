#include <stdio.h>

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

int main() {
    int q;
    // Clear the screen first (optional but helpful)
    clearScreen();

    // Print 'X' at Column 20, Row 10
    moveCursor(64, 18);
    putchar('X');

    // Move the cursor out of the way at the end
    moveCursor(1, 15);
    printf("\n");
    scanf("%d", &q);

    return 0;
}
