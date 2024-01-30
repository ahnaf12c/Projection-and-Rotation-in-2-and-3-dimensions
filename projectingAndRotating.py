import math as m

START_POSITION = [0, 0]

def DegreeToRadian(D):
    return D * m.pi / 180


def RadianToDegree(R):
    return 180 * R / m.pi


def project(x, y, z, F):
    if F <= z:
        print("Error! Can't project a point behind the viewing point.\n")
        return
    else:
        px = F * x / (F - z)
        py = F * y / (F - z)
        P = [px, py]

    return P


def Rotate2D(x, y, angle):
    # Define the 2D rotation matrix
    R = [
        [m.cos(angle), -m.sin(angle)],
        [m.sin(angle), m.cos(angle)]
    ]

    # Apply the rotation matrix to the coordinates
    rx = R[0][0] * x + R[0][1] * y
    ry = R[1][0] * x + R[1][1] * y

    return [round(rx, 8), round(ry, 8)]

def Rotate3DInX(x, y, z, A):
    R = [
        [1, 0, 0],
        [0, m.cos(A), -m.sin(A)],
        [0, m.sin(A), m.cos(A)]
    ]
    
    rx = x * R[0][0] + y * R[0][1] + z * R[0][2]
    ry = x * R[1][0] + y * R[1][1] + z * R[1][2]
    rz = x * R[2][0] + y * R[2][1] + z * R[2][2]
    return [round(rx, 8), round(ry, 8), round(rz, 8)]

def Rotate3DInY(x, y, z, A):
    R = [
        [m.cos(A), 0, m.sin(A)],
        [0, 1, 0],
        [-m.sin(A), 0, m.cos(A)]
    ]
    
    rx = x * R[0][0] + y * R[0][1] + z * R[0][2]
    ry = x * R[1][0] + y * R[1][1] + z * R[1][2]
    rz = x * R[2][0] + y * R[2][1] + z * R[2][2]
    return [round(rx, 8), round(ry, 8), round(rz, 8)]

def Rotate3DInZ(x, y, z, A):
    R = [
        [m.cos(A), -m.sin(A), 0],
        [m.sin(A), m.cos(A),  0],
        [0, 0, 1]
    ]
    
    rx = x * R[0][0] + y * R[0][1] + z * R[0][2]
    ry = x * R[1][0] + y * R[1][1] + z * R[1][2]
    rz = x * R[2][0] + y * R[2][1] + z * R[2][2]
    return [round(rx, 8), round(ry, 8), round(rz, 8)]
