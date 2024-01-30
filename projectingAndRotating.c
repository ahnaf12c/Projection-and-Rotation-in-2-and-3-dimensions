#include<stdio.h>
#define _USE_MATH_DEFINES
#include<math.h>

double DegreeToRadian(double D) {
    return D * M_PI / 180;
}

double RadianToDegree(double R) {
    return 180 * R / M_PI;
}

void project(double x, double y, double z, double F) {
    if (F <= z) {
        printf("Error! Not possible project a point that is behind the viewing point!\n");
    }
    else {
        double px = x * F / (F - z);
        double py = y * F / (F - z); 
        printf("(%lf, %lf)\n", px, py);
    }
    return;
}

void Rotate2D(double x, double y, double A) {
    double rx = x * cos(A) - y * sin(A);
    double ry = x * sin(A) + y * cos(A);
    printf("(%lf, %lf)\n", rx, ry);
    return;
}

void Rotate3DInX(double x, double y, double z, double A) {
    double R[3][3] = {
        {1, 0, 0},
        {0, cos(A), -sin(A)},
        {0, sin(A), cos(A)}
    };
    double rx = R[0][0] * x + R[0][1] * y + R[0][2] * z;
    double ry = R[1][0] * x + R[1][1] * y + R[1][2] * z;
    double rz = R[2][0] * x + R[2][1] * y + R[2][2] * z;
    printf("(%lf, %lf, %lf)\n", rx, ry, rz);
    return;
}

void Rotate3DInY(double x, double y, double z, double A) {
    double R[3][3] = {
        {cos(A), 0, sin(A)},
        {0, 1, 0},
        {-sin(A), 0, cos(A)}
    };
    double rx = R[0][0] * x + R[0][1] * y + R[0][2] * z;
    double ry = R[1][0] * x + R[1][1] * y + R[1][2] * z;
    double rz = R[2][0] * x + R[2][1] * y + R[2][2] * z;
    printf("(%lf, %lf, %lf)\n", rx, ry, rz);
    return;
}

void Rotate3DInZ(double x, double y, double z, double A) {
    double R[3][3] = {
        {cos(A), -sin(A), 0},
        {sin(A), cos(A), 0},
        {0, 0, 1}
    };
    double rx = R[0][0] * x + R[0][1] * y + R[0][2] * z;
    double ry = R[1][0] * x + R[1][1] * y + R[1][2] * z;
    double rz = R[2][0] * x + R[2][1] * y + R[2][2] * z;
    printf("(%lf, %lf, %lf)\n", rx, ry, rz);
    return;
}

/// @brief calls all the functions
/// @return 0
int main () {
    double r = DegreeToRadian(45);
    //printf("%lf\n", r);
    //double d = RadianToDegree(r);
    //printf("%lf     %lf\n", r, d);
    //project(2, 2, 2, 4);
    //Rotate2D(2, 2, r);
    //Rotate3DInX(2, 2, 2, r);
    //Rotate3DInY(2, 2, 2, r);
    //Rotate3DInZ(2, 2, 2, r);
    return 0;
}
