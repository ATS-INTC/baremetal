import numpy as np
import sys

def main():
    result_file = sys.argv[1]

    with open(result_file, 'r') as f:
        content = f.read()
    result_list = [x for x in content.split(", ")]
    result = np.array([float(x) for x in result_list])
    with open("points_" + result_file, "a") as points_file:
        freq = 1.0 / len(result)
        for idx, val in enumerate(result):
            points_file.write(str(val) + " " + str(freq) + "\n")
    mean = np.mean(result)
    std = np.std(result)

    print("avg delay: ", mean)
    print("delay std: ", std)

if __name__ == "__main__":
    main()