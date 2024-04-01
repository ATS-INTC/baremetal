import numpy as np
import sys

def main():
    result_file = sys.argv[1]

    with open(result_file, 'r') as f:
        content = f.read()
    result_list = [x for x in content.split(", ")]
    result = np.array([float(x) for x in result_list])
    mean = np.mean(result)
    std = np.std(result)

    print("avg cycle: ", mean)
    print("cycle std: ", std)

if __name__ == "__main__":
    main()