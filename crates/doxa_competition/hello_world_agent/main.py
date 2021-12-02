x = input()

with open("/output/test.txt", "w") as f:
    f.write("echo {}".format(x))

print("echo", x)
