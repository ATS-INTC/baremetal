#!/usr/bin/python
# -*- coding: UTF-8 -*-

import socket

tcp_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    
server_addr = ("172.16.1.2", 80)
tcp_socket.connect(server_addr)

send_data = "connect ok?"
count = 10
while count > 0:
    tcp_socket.send(send_data.encode("utf8"))
    recv_data = tcp_socket.recv(1024)
    tcp_socket.settimeout(50)
    print('recv connect result:', recv_data.decode("utf8"))
    count -= 1

