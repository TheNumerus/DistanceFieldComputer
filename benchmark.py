import subprocess

def main():
    images = {"4K.png": 256, "2K.png": 128, "512.png": 64, "small_rectangle.png": 64}
    repeat_modes = ["repeat", "clamp", "mirror"]
    with open('results.txt', 'w') as file:
        for repeat in repeat_modes:
            for image in images:
                file.write("{0} {1}\n".format(image, repeat))
                for i in range(10):
                    sub = subprocess.run(["cargo",  "run",  "--release", ".\\tests\\test_images\\{0}".format(image),  "--repeat={0}".format(repeat), "-r", " {0}".format(images[image]), "-m",  "1", "-g"], stdout=subprocess.PIPE, shell=True)
                    parts = str(sub.stdout, 'utf-8').splitlines()
                    file.write("{}\n".format(parts[-3]))


if __name__ == '__main__':
    main()