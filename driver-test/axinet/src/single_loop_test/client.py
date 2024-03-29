#!/usr/bin/python
# -*- coding: UTF-8 -*-

import socket
import struct
import time

MTU = 60

raw_socket = socket.socket(socket.PF_PACKET, socket.SOCK_RAW, socket.htons(0x1234))
raw_socket.bind(("enp4s0", 0))
packet = struct.pack("!6s6sH", b"\x00\x0a\x35\x01\x02\x03", b"\x04\x7c\x16\xef\x34\xd1", 0x1234)
while True:
    raw_socket.send(packet + b'a' * (MTU - 14))
    data = raw_socket.recv(MTU)
    time.sleep(0.0000001)
