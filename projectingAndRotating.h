#define PROJECTING_AND_ROTATING_H

#include <stdio.h>
#define _USE_MATH_DEFINES
#include <math.h>

typedef struct {
    double x;
    double y;
} Coords2d;

typedef struct {
    double x;
    double y;
    double z;
} Coords3d;

double DegreeToRadian(double D) {
    return D * M_PI / 180.0;
}

double RadianToDegree(double R) {
    return 180.0 * R / M_PI;
}

Coords2d Rotate2D (double x, double y, double A) {
    Coords2d r;
    double cosA = cos(A);
    double sinA = sin(A);
    r.x = x*cosA - y*sinA;
    r.y = x*sinA + y*cosA;
    return r;
}

Coords2d project(double x, double y, double z, double F) {
    Coords2d p = {0.0, 0.0}; // Default fallback
    if (F <= z) {
        // Prevent printing on every frame to avoid clogging the terminal
        // Just return the fallback 0,0 clipping point safely
        return p; 
    }
    
    p.x = x * F / (F - z);
    p.y = y * F / (F - z);
    return p;
}

// Optimized 3D rotations (removing redundant 0 and 1 multiplications)
Coords3d Rotate3DInX(double x, double y, double z, double A) {
    Coords3d r;
    double cosA = cos(A);
    double sinA = sin(A);
    r.x = x;
    r.y = y * cosA - z * sinA;
    r.z = y * sinA + z * cosA;
    return r;
}

Coords3d Rotate3DInY(double x, double y, double z, double A) {
    Coords3d r;
    double cosA = cos(A);
    double sinA = sin(A);
    r.x = x * cosA + z * sinA;
    r.y = y;
    r.z = -x * sinA + z * cosA;
    return r;
}

Coords3d Rotate3DInZ(double x, double y, double z, double A) {
    Coords3d r;
    double cosA = cos(A);
    double sinA = sin(A);
    r.x = x * cosA - y * sinA;
    r.y = x * sinA + y * cosA;
    r.z = z;
    return r;
}

#endif // PROJECTING_AND_ROTATING_H
