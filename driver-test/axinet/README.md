
# This is the AxiNic Test crate

The AxiNic consists of a Xilinx AXI DMA and a Xilinx AXI Ethernet 1G/2.5G subsystem. The AXI Ethernet is configured as 1Gbits/s.

We will conduct four test: 

- [x] Poll mode.
- [x] Interrupt mode.
- [x] Rust Future + ATSINTC.

The comparation can be seen in the [assets](./assets/) directory.

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

This test case is the same as before, we will send package one by one. But we use the Rust `Future` and `ATSINTC`. The execution change is under the help of Rust compiler while the notification mechanism is provided by `ATSINTC`.

| Frame size(byte)\threshold | 1      | 2      | 3      | 4      | 5      |
| -------------------------- | ------ | ------ | ------ | ------ | ------ |
| 60                         | 0.033  | 0.043  | 0.050  | 0.053  | 0.056  |
| 64                         | 0.035  | 0.046  | 0.053  | 0.057  | 0.060  |
| 128                        | 0.071  | 0.093  | 0.107  | 0.113  | 0.119  |
| 256                        | 0.141  | 0.185  | 0.212  | 0.224  | 0.236  |
| 512                        | 0.293  | 0.369  | 0.421  | 0.444  | 0.468  |
| 1000                       | 0.560  | 0.491  | 0.585  | 0.649  | 0.915  |
| 1024                       | 0.574  | 0.507  | 0.609  | 0.679  | 0.941  |
| 1514                       | 0.844  | 0.639  | 0.746  | 0.809  |        |
| 2000                       | 0.553  | 0.738  | 0.864  | 0.927  | 0.965  |
| 2048                       | 0.565  | 0.763  | 0.874  | 0.936  | 0.980  |
| 3000                       | 0.697  | 0.879  | 0.981  | 0.991  | 0.992  |
| 4000                       | 0.820  | 0.993  | 0.993  | 0.993  | 0.994  |
| 4096                       | 0.834  | 0.994  | 0.994  | 0.994  | 0.994  |
| 4500                       | 0.860  | 0.994  | 0.994  | 0.994  | 0.994  |
| 5000                       | 0.906  | 0.995  | 0.995  | 0.995  | 0.995  |
| 6000                       | 0.973  | 0.995  | 0.995  | 0.996  | 0.996  |
| 7000                       | 0.996  | 0.996  | 0.996  | 0.996  | 0.996  |
| 8000                       | 0.996  | 0.996  | 0.996  | 0.997  | 0.997  |
| 8192                       | 0.996  | 0.996  | 0.996  | 0.997  | 0.997  |
| 9000                       | 0.997  | 0.997  | 0.997  | 0.997  | 0.997  |
| 10000                      | 0.997  | 0.997  | 0.997  | 0.997  | 0.997  |
| 16128                      | 0.998  | 0.998  | 0.998  | 0.998  | 0.998  |

