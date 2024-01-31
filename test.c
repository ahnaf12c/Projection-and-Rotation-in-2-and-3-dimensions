#include<stdio.h>
#include"projectingAndRotating.h"

int main () {
    double r = DegreeToRadian(45);
    Coords3d R = Rotate3DInX(20, 0, 20, r);
    printf("%lf  %lf  %lf\n", R.x, R.y, R.z);
    return 0;
}
