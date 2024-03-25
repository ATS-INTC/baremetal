
# This is the AxiNic Test crate

The AxiNic consists of a Xilinx AXI DMA and a Xilinx AXI Ethernet 1G/2.5G subsystem. The AXI Ethernet is configured as 1Gbits/s.

We will conduct four test: 

- [x] Poll mode.
- [x] Interrupt mode.
- [x] Rust Future + ATSINTC.

## Poll mode

In this case, we create a buffer and submit it to the AxiNic. Then we wait the transmition to be finished by checking the AXI DMA interrupt status register. We won't submit the next buffer until the previous transmition has been completed. The test result is shown below:

We measure the AxiNic performance by counting the amount of bytes sent within one second. The total data size is 10GB.

| Frame size(byte) | Throughput(Gbits/sec) |
| ---------------- | --------------------- |
| 60               | 0.029                 |
| 64               | 0.032                 |
| 128              | 0.064                 |
| 256              | 0.124                 |
| 512              | 0.226                 |
| 1000             | 0.385                 |
| 1024             | 0.396                 |
| 1514             | 0.513                 |
| 2000             | 0.620                 |
| 2048             | 0.636                 |
| 3000             | 0.769                 |
| 4000             | 0.893                 |
| 4096             | 0.906                 |
| 4500             | 0.930                 |
| 5000             | 0.975                 |
| 6000             | 0.995                 |
| 7000             | 0.996                 |
| 8000             | 0.996                 |
| 8192             | 0.996                 |
| 9000             | 0.997                 |
| 10000            | 0.997                 |
| 16128            | 0.998                 |

## Interrupt mode

The configuration of AxiNic is as the same as the poll mode. However, we won't submit the next buffer until the interrupt of previous transmition has been handled.

We set a interrupt threshold on DMA. DMA will trigger an interrupt only when the number of packets sent reaches the threshold. 

The test result is shown below:

| Frame size(byte)\threshold | 1      | 2      | 3      | 4      | 5      |
| -------------------------- | ------ | ------ | ------ | ------ | ------ |
| 60                         | 0.027  | 0.037  | 0.044  | 0.050  | 0.052  |
| 64                         | 0.029  | 0.040  | 0.047  | 0.053  | 0.056  |
| 128                        | 0.057  | 0.079  | 0.093  | 0.104  | 0.110  |
| 256                        | 0.110  | 0.154  | 0.182  | 0.206  | 0.216  |
| 512                        | 0.205  | 0.300  | 0.344  | 0.393  | 0.415  |
| 1000                       | 0.352  | 0.510  | 0.591  | 0.674  | 0.714  |
| 1024                       | 0.362  | 0.533  | 0.619  | 0.700  | 0.743  |
| 1514                       | 0.478  | 0.667  | 0.749  | 0.824  | 0.873  |
| 2000                       | 0.579  | 0.773  | 0.858  | 0.952  | 0.972  |
| 2048                       | 0.581  | 0.790  | 0.873  | 0.971  | 0.988  |
| 3000                       | 0.725  | 0.904  | 0.992  | 0.992  | 0.992  |
| 4000                       | 0.849  | 0.993  | 0.993  | 0.994  | 0.994  |
| 4096                       | 0.863  | 0.994  | 0.994  | 0.994  | 0.994  |
| 4500                       | 0.868  | 0.994  | 0.994  | 0.994  | 0.994  |
| 5000                       | 0.910  | 0.995  | 0.995  | 0.995  | 0.995  |
| 6000                       | 0.972  | 0.995  | 0.995  | 0.996  | 0.996  |
| 7000                       | 0.996  | 0.996  | 0.996  | 0.996  | 0.996  |
| 8000                       | 0.996  | 0.996  | 0.996  | 0.997  | 0.997  |
| 8192                       | 0.996  | 0.996  | 0.996  | 0.997  | 0.997  |
| 9000                       | 0.997  | 0.997  | 0.997  | 0.997  | 0.997  |
| 10000                      | 0.997  | 0.997  | 0.997  | 0.997  | 0.997  |
| 16128                      | 0.998  | 0.998  | 0.998  | 0.998  | 0.998  |

## Rust Future + ATSINTC

The `ATSINTC` is the a interrupt controller integrated with asynchonrous task scheduling. When an interrupt occur, it will directly wake the interrupt handler task without breaking the execution of CPU.

### Transmit package one by one

This test case is the same as before, we will send package one by one. But we use the Rust `Future` and `ATSINTC`. The execution change is under the help of Rust compiler while the notification mechanism is provided by `ATSINTC`.

| Frame size(byte) | Throughput(Gbits/sec) |
| ---------------- | --------------------- |
| 60               | 0.035                 |
| 64               | 0.037                 |
| 128              | 0.075                 |
| 256              | 0.103                 |
| 512              | 0.195                 |
| 1000             | 0.336                 |
| 1024             | 0.346                 |
| 1514             | 0.456                 |
| 2000             | 0.557                 |
| 2048             | 0.564                 |
| 3000             | 0.702                 |
| 4000             | 0.824                 |
| 4096             | 0.824                 |
| 4500             | 0.864                 |
| 5000             | 0.910                 |
| 6000             | 0.977                 |
| 7000             | 0.996                 |
| 8000             | 0.996                 |
| 8192             | 0.996                 |
| 9000             | 0.997                 |
| 10000            | 0.997                 |
| 16128            | 0.998                 |

### Bulk transmit package

