import turtle
import math

def draw_cube(side_length, speed, colour):
    half_side_length = side_length/2
    root_length = 2*(half_side_length**2)
    turtle.color(colour)
    turtle.speed(speed)
    def draw_2d_square():
        for i in range(4):
            turtle.forward(side_length)
            turtle.left(90)
    def set_pen_to_center():
        turtle.left(90)
        turtle.penup()
        turtle.forward(half_side_length)
        turtle.right(90)
        turtle.forward(half_side_length)
        turtle.left(180)
        turtle.pendown()
    def draw_the_diagonal_lines():
        def draw_the_1_diagonal_lines():
            turtle.right(135)
            turtle.forward(math.sqrt(root_length))
            turtle.backward(math.sqrt(root_length))
            turtle.left(135)
        draw_the_1_diagonal_lines()
        def set_the_pen():
            turtle.penup()
            turtle.forward(side_length)
            turtle.pendown()
        set_the_pen()
        turtle.left(90)

        set_the_pen()

        turtle.left(135)
        turtle.forward(math.sqrt(root_length))
        turtle.backward(math.sqrt(root_length))
        turtle.right(45)

        set_the_pen()

        turtle.left(45)
        turtle.forward(math.sqrt(root_length))

    draw_2d_square()
    set_pen_to_center()
    draw_2d_square()
    draw_the_diagonal_lines()

    turtle.hideturtle()

    turtle.exitonclick()

draw_cube(100, 0, "blue")

