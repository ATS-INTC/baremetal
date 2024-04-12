import numpy as np
import sys

def main():
    result_file = sys.argv[1]

    with open(result_file, 'r') as f:
        content = f.read()
    result_list = [x for x in content.split(" ")]
    result_list.pop(-1)
    result = np.array([float(x) for x in result_list])
    with open(result_file, "w") as points_file:
        freq = 1.0 / len(result)
        for idx, val in enumerate(result):
            points_file.write(str(val) + " " + str(freq) + "\n")
    mean = np.mean(result)
    std = np.std(result)
    
    with open("analysis_" + result_file, "w") as analysis_file:
        analysis_file.write(str(mean) + "\t\t" + str(std) + "\t\t" + str(len(result)) + "\n")
    
    print("avg delay: ", mean)
    print("delay std: ", std)

if __name__ == "__main__":
    main()