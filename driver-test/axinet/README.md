
# This is the AxiNic Test crate

The AxiNic consists of a Xilinx AXI DMA and a Xilinx AXI Ethernet 1G/2.5G subsystem. The AXI Ethernet is configured as 1Gbits/s.

We built three models:

- [x] Poll.
- [x] Interrupt.
- [x] Rust Future + ATSINTC.

## Tests

- [x] Line speed test.
- [x] Transmit and receiver test.
- [x] Single loop test.
- [x] Single tcp test.
- [x] Network Protocal Stack test.
- [x] Multiple priority connections test.

How to run tests on FPGA?
- `make DRIVER=axinet run TEST=$(TEST)`


### [Line speed test](./src/transmit_line_speed/)

```
make DRIVER=axinet run TEST=transmit_line_speed_$(MODE) MTU=***
```

```
python receiver.py $(MTU)
```

- MODE: poll / intr / atsintc.
- MTU: The length of each ethernet package.

### [Transmit and receiver test](./src/transeiver/)

There are only results in this directory. The [test code](https://github.com/ATS-INTC/baremetal/tree/main/driver-test/axinet/src/single_loop_test) is under another branch of this repository.

### [Single loop test](./src/single_loop/)

```
make DRIVER=axinet run TEST=single_loop_$(MODE) MTU=*** SCALE=*** 
```

```
python client.py $(MODE) $(MTU) $(SCALE)
```

- MODE: poll / intr / atsintc.
- MTU: The length of each ethernet package.
- SCALE: The size of the matrix(workload).

### [single_tcp_test](./src/single_tcp/)

```
make DRIVER=axinet run TEST=single_tcp_$(MODE) SCALE=***
```

```
python tcp_client.py $(MODE) $(SCALE)
```

- MODE: poll / intr / atsintc.
- SCALE: The size of the matrix(workload).

### [Network Protocal Stack test](./src/ns_ping/)

```
make DRIVER=axinet run TEST=ns_ping_$(MODE)
```

```
python client.py $(MODE) $(COUNT)
```

- MODE: poll / intr / atsintc.
- COUNT: The total number of ping request is `COUNT * 200`

### [Multiple priority connections test](./src/prio_connect/)

```
make DRIVER=axinet run TEST=prio_connect SCALE=*** PRIO=*** CPN=***
```

```
python tcp_client_prio.py $(SCALE) $(PRIO) $(CPN)
```

- SCALE: The size of the matrix(workload).
- PRIO: The number of priority.
- CPN: The connection number of each priority.
