def is_point_inside_triangle(i, j, x, y, z):
    # Calculating triangles areas
    triangle_area = abs(0.5 * ((y[0] - x[0]) * (z[1] - x[1]) - (z[0] - x[0]) * (y[1] - x[1])))
    a = abs(0.5 * ((y[0] - i) * (z[1] - j) - (z[0] - i) * (y[1] - j)))
    b = abs(0.5 * ((z[0] - i) * (x[1] - j) - (x[0] - i) * (z[1] - j)))
    c = abs(0.5 * ((x[0] - i) * (y[1] - j) - (y[0] - i) * (x[1] - j)))

    # Checking barycentric coordinates
    return a + b + c <= triangle_area

# Point coordinates
i = int(input("Enter x coordinate of a point: "))
j = int(input("Enter y coordinate of a point: "))

# Triangle vertex coordinates
x = [int(input("Enter x1: ")), int(input("Enter x2: "))]
y = [int(input("Enter y1: ")), int(input("Enter y2: "))]
z = [int(input("Enter z1: ")), int(input("Enter z2: "))]

# Getting result
result = is_point_inside_triangle(i, j, x, y, z)

# Prints true or false
print(result)