import numpy as np
import sys

def main():
    result_file = sys.argv[1]

    with open(result_file, 'r') as f:
        content = f.read()
    result_list = [x for x in content.split(" ")]
    result_list.pop(-1)
    result = np.array([float(x) for x in result_list])
    freq_file = result_file.removesuffix(".dat") + "_freq.dat"
    with open(freq_file, "w") as points_file:
        freq = 1.0 / len(result)
        for idx, val in enumerate(result):
            points_file.write(str(int(val)) + " " + str(freq) + "\n")
    mean = np.mean(result)
    std = np.std(result)
    stat_file = result_file.removesuffix(".dat") + "_stat.dat"
    with open(stat_file, "w") as points_file:
        points_file.write("avg delay: " + str(mean) + "\n")
        points_file.write("delay std: " + str(std) + "\n")
    print("avg delay: ", mean)
    print("delay std: ", std)
    print("min: ", min(result))
    print("max: ", max(result))

if __name__ == "__main__":
    main()