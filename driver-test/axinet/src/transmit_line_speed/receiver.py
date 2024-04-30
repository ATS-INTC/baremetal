#!/usr/bin/python
# -*- coding: UTF-8 -*-

import socket
import time
import sys


MTU = int(sys.argv[1])


raw_socket = socket.socket(socket.PF_PACKET, socket.SOCK_RAW, socket.htons(0x1234))
raw_socket.bind(("enp4s0", 0))

recv_bytes = 0
data = raw_socket.recv(MTU)
pass_time = time.time()
while True:
    data = raw_socket.recv(MTU)
    recv_bytes += MTU
    start = time.time()
    if start - pass_time > 1:
        speed = recv_bytes / (start - pass_time)
        print("speed: " + str(speed * 8 / 1000000) + "Mbits/sec")
        pass_time = time.time()
        recv_bytes = 0
