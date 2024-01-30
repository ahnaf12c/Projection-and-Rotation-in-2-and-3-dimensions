#include<stdio.h>
#define _USE_MATH_DEFINES
#include<math.h>

typedef struct
{
    double x;
    double y;
} Coords2d;

typedef struct
{
    double x;
    double y;
    double z;
} Coords3d;

double DegreeToRadian(double D) {
    return D * M_PI / 180;
}

double RadianToDegree(double R) {
    return 180 * R / M_PI;
}

Coords2d project(double x, double y, double z, double F) {
    if (F <= z) {
        printf("Error! Not possible project a point that is behind the viewing point!\n");
    }
    else {
        Coords2d p;
        p.x = x * F / (F - z);
        p.y = y * F / (F - z);
        return p;
    }
}

Coords2d Rotate2D(double x, double y, double A) {
    Coords2d r;
    r.x = x * cos(A) - y * sin(A);
    r.y = x * sin(A) + y * cos(A);
    return r;
}

Coords3d Rotate3DInX(double x, double y, double z, double A) {
    double R[3][3] = {
        {1, 0, 0},
        {0, cos(A), -sin(A)},
        {0, sin(A), cos(A)}
    };
    Coords3d r;
    r.x = R[0][0] * x + R[0][1] * y + R[0][2] * z;
    r.y = R[1][0] * x + R[1][1] * y + R[1][2] * z;
    r.z = R[2][0] * x + R[2][1] * y + R[2][2] * z;
    return r;
}

Coords3d Rotate3DInY(double x, double y, double z, double A) {
    double R[3][3] = {
        {cos(A), 0, sin(A)},
        {0, 1, 0},
        {-sin(A), 0, cos(A)}
    };
    Coords3d r;
    r.x = R[0][0] * x + R[0][1] * y + R[0][2] * z;
    r.y = R[1][0] * x + R[1][1] * y + R[1][2] * z;
    r.z = R[2][0] * x + R[2][1] * y + R[2][2] * z;
    return r;
}

Coords3d Rotate3DInZ(double x, double y, double z, double A) {
    double R[3][3] = {
        {cos(A), -sin(A), 0},
        {sin(A), cos(A), 0},
        {0, 0, 1}
    };
    Coords3d r;
    r.x = R[0][0] * x + R[0][1] * y + R[0][2] * z;
    r.y = R[1][0] * x + R[1][1] * y + R[1][2] * z;
    r.z = R[2][0] * x + R[2][1] * y + R[2][2] * z;
    return r;
}

int main () {
    double r = DegreeToRadian(45);
    Coords3d R = Rotate3DInX(20, 0, 20, r);
    printf("%lf  %lf  %lf\n", R.x, R.y, R.z);
    return 0;
}
