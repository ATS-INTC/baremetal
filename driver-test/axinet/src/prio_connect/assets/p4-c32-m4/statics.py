
import numpy as np
import sys

def main():
    result_file = sys.argv[1]

    with open(result_file, 'r') as f:
        content = f.read()
    result_list = [x for x in content.split(" ")]
    result_list.pop(-1)
    result = np.array([float(x) for x in result_list])
    sort_array = np.sort(result)
    length = len(sort_array)
    lastp1 = length * 0.01
    print(length)
    print(lastp1)
    # print(sort_array[-int(lastp1):])

    lastp1_result = sort_array[-int(lastp1):]
    print(np.mean(lastp1_result))
    # lastp1file = result_file.removesuffix(".dat") + "_lp1.dat"
    # with open(lastp1file, "w") as points_file:
    #     points_file.write("avarage: " + str(np.mean(lastp1_result)) + "\n")
        # freq = 1.0 / lastp1
        # for idx, val in enumerate(lastp1_result):
        #     points_file.write(str(int(val)) + " " + str(freq) + "\n")

if __name__ == "__main__":
    main()


