#!/usr/bin/python
# -*- coding: UTF-8 -*-

import socket
import time
import os
import sys
import stat   
import numpy as np
import re

MODE = sys.argv[1]
COUNT = sys.argv[2]
delays = []

NUM = 200
count = NUM
while count > 0:
    result = os.popen('ping -A 172.16.1.2 -4 -c ' + COUNT)
    lines = result.readlines()
    lines.pop(-1)
    lines.pop(-1)
    for idx, line in enumerate(lines):
        match = re.search(r"0.[0-9]*", line)
        if match != None:
            latency = float(match.group())
            if latency > 0:
                delays.append(latency)
    count -= 1

avarage = np.mean(delays)
std_deviation = np.std(delays)

path = "../../assets/multi_prio/" + MODE
if os.path.exists(path + "/" + MODE + ".dat"):
    with open(path + "/" + MODE + ".dat", mode='a', encoding='utf-8') as file:
        content = "{:07.3f}\t\t\t\t{:07.3f}".format(avarage, std_deviation)
        file.write(content + "\n")
else:
    os.mkdir(path)
    os.chmod(path, stat.S_IRWXO + stat.S_IRWXG + stat.S_IRWXU)
    with open(path + "/" + MODE + ".dat", mode='w', encoding='utf-8') as file:
        file.write("Avarage(ms)\t\t\tStd_deviation\n")
        content = "{:07.3f}\t\t\t\t{:07.3f}".format(avarage, std_deviation)
        file.write(content + "\n")
    os.chmod(path + "/" + MODE + ".dat", stat.S_IRWXO + stat.S_IRWXG + stat.S_IRWXU)

with open(path + "/" + MODE + "_delay.dat", mode='w', encoding='utf-8') as file:
    content = str(delays)
    content = content.removeprefix('[')
    content = content.removesuffix(']')
    content = content.replace(',', '')
    file.write(content)
os.chmod(path + "/" + MODE + "_delay.dat", stat.S_IRWXO + stat.S_IRWXG + stat.S_IRWXU)