#!/usr/bin/python
# -*- coding: UTF-8 -*-

import socket
import time
import numpy as np
import sys
import os
import stat

MODE = sys.argv[1]
SCALE = sys.argv[2]

tcp_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    
server_addr = ("172.16.1.2", 80)
tcp_socket.connect(server_addr)

send_data = "connect ok?"
count = 100000
delays = []
while count > 0:
    tcp_socket.send(send_data.encode("utf8"))
    start = time.time()
    recv_data = tcp_socket.recv(1024)
    end = time.time()
    res = (end - start) * 1000000
    delays.append(res)
    count -= 1

avarage = np.mean(delays)
std_deviation = np.std(delays)
# print(delays)
print(avarage)
print(std_deviation)

path = "./" + MODE + SCALE
if os.path.exists(path + "/" + MODE + ".dat"):
    with open(path + "/" + MODE + ".dat", mode='a', encoding='utf-8') as file:
        content = "{:07.3f}\t\t\t\t{:07.3f}".format(avarage, std_deviation)
        file.write(content + "\n")
else:
    os.mkdir(path)
    os.chmod(path, stat.S_IRWXO + stat.S_IRWXG + stat.S_IRWXU)
    with open(path + "/" + MODE + ".dat", mode='w', encoding='utf-8') as file:
        file.write("Avarage(us)\t\t\tStd_deviation\n")
        content = "{:07.3f}\t\t\t\t{:07.3f}".format(avarage, std_deviation)
        file.write(content + "\n")
    os.chmod(path + "/" + MODE + ".dat", stat.S_IRWXO + stat.S_IRWXG + stat.S_IRWXU)

with open(path + "/" + MODE + "_delay.dat", mode='a', encoding='utf-8') as file:
    content = str(delays)
    content = content.removeprefix('[')
    content = content.removesuffix(']')
    content = content.replace(',', '')
    content += ' '
    file.write(content)
os.chmod(path + "/" + MODE + "_delay.dat", stat.S_IRWXO + stat.S_IRWXG + stat.S_IRWXU)