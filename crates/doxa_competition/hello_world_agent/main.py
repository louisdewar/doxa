import os

x = input()

with open("/output/test.txt", "w") as f:
    f.write("echo {}".format(x))

print("echo", x)

input()

if len(os.listdir("/output")) != 0:
    raise Exception(f"""After the second input the helloworld competition should have taken the
        file but it sill exists ({os.listdir('/output')})""")

print("done")
