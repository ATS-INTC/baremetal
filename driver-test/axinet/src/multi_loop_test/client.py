#!/usr/bin/python
# -*- coding: UTF-8 -*-

import socket
import struct
import time
import os
import sys
import stat   

import numpy as np

MODE = sys.argv[1]
MTU = int(sys.argv[2])
SCALE = sys.argv[3]


TIME_OUT = 1
raw_socket = socket.socket(socket.PF_PACKET, socket.SOCK_RAW, socket.htons(0x1234))
raw_socket.bind(("enp4s0", 0))
packet = struct.pack("!6s6sH", b"\x00\x0a\x35\x01\x02\x03", b"\x04\x7c\x16\xef\x34\xd1", 0x1234)
delays = []

start_time = time.time()
while True:
    if time.time() -start_time > TIME_OUT:
        break
    raw_socket.send(packet + b'a' * (MTU - 14))
    start = time.time()
    data = raw_socket.recv(MTU)
    end = time.time()
    delays.append((end - start) * 1000000)
    time.sleep(0.000000001)


pps = len(delays) / TIME_OUT
avarage = np.mean(delays)
std_deviation = np.std(delays)

path = "../../assets/multi_loop/" + MODE
if os.path.exists(path + "/" + MODE + ".dat"):
    with open(path + "/" + MODE + ".dat", mode='a', encoding='utf-8') as file:
        if SCALE == None:
            content = "{:-5d}\t\t\t{:07.3f}\t\t\t\t{:07.3f}\t\t\t\t{:07.1f}".format(MTU, avarage, std_deviation, pps)
            file.write(content + "\n")
        else:
            content = "{:-5d}\t\t\t{:07.3f}\t\t\t\t{:07.3f}\t\t\t\t{:07.1f}\t\t\t\t{:5s}".format(MTU, avarage, std_deviation, pps, SCALE)
            file.write(content + "\n")
else:
    os.mkdir(path)
    os.chmod(path, stat.S_IRWXO + stat.S_IRWXG + stat.S_IRWXU)
    with open(path + "/" + MODE + ".dat", mode='w', encoding='utf-8') as file:
        if SCALE == None:
            file.write("Frame_size(Bytes)\tAvarage(us)\t\t\tStd_deviation\t\t\tThroughput(pps)\n")
            content = "{:-5d}\t\t\t{:07.3f}\t\t\t\t{:07.3f}\t\t\t\t{:07.1f}".format(MTU, avarage, std_deviation, pps)
            file.write(content + "\n")
        else:
            file.write("Frame_size(Bytes)\tAvarage(us)\t\t\tStd_deviation\t\t\tThroughput(pps)\t\t\tMatrix_size\n")
            content = "{:-5d}\t\t\t{:07.3f}\t\t\t\t{:07.3f}\t\t\t\t{:07.1f}\t\t\t\t{:5s}".format(MTU, avarage, std_deviation, pps, SCALE)
            file.write(content + "\n")
    os.chmod(path + "/" + MODE + ".dat", stat.S_IRWXO + stat.S_IRWXG + stat.S_IRWXU)

with open(path + "/" + MODE + str(MTU) + "_delay.dat", mode='w', encoding='utf-8') as file:
    content = str(delays)
    content = content.removeprefix('[')
    content = content.removesuffix(']')
    content = content.replace(',', '')
    file.write(content)
os.chmod(path + "/" + MODE + str(MTU) + "_delay.dat", stat.S_IRWXO + stat.S_IRWXG + stat.S_IRWXU)