
# This is the AxiNic Test crate

The AxiNic consists of a Xilinx AXI DMA and a Xilinx AXI Ethernet 1G/2.5G subsystem. The AXI Ethernet is configured as 1Gbits/s.

We will conduct four test: 

- [x] Poll mode.
- [ ] Interrupt mode.
- [ ] Interrupt mode + Rust Future.
- [ ] Rust Future + ATSINTC.

## Poll mode

In this case, we create a buffer and submit it to the AxiNic. Then we wait the transmition to be finished by checking the AXI DMA interrupt status register. We won't submit the next buffer until the previous transmition has been completed. The test result is shown below:

We measure the AxiNic performance by counting the amount of bytes sent within one second. The total data size is 10GB.

| Frame size(byte) | Speed(Gbits/sec) |
| ---------------- | ---------------- |
| 60               | 0.029            |
| 64               | 0.032            |
| 128              | 0.064            |
| 256              | 0.124            |
| 512              | 0.226            |
| 1000             | 0.385            |
| 1024             | 0.396            |
| 1514             | 0.513            |
| 2000             | 0.620            |
| 2048             | 0.636            |
| 3000             | 0.769            |
| 4000             | 0.893            |
| 4096             | 0.906            |
| 4500             | 0.930            |
| 5000             | 0.975            |
| 6000             | 0.995            |
| 7000             | 0.996            |
| 8000             | 0.996            |
| 8192             | 0.996            |
| 9000             | 0.997            |
| 10000            | 0.997            |
| 16128            | 0.998            |

## Interrupt mode

### No interrupt coalesce

The configuration of AxiNic is as the same as the poll mode. However, we won't submit the next buffer until the interrupt of previous transmition has been handled. The test result is shown below:

| Frame size(byte) | Speed(Gbits/sec) |
| ---------------- | ---------------- |
| 60               | 0.028            |
| 64               | 0.030            |
| 128              | 0.058            |
| 256              | 0.113            |
| 512              | 0.211            |
| 1000             | 0.361            |
| 1024             | 0.373            |
| 1514             | 0.490            |
| 2000             | 0.593            |
| 2048             | 0.595            |
| 3000             | 0.741            |
| 4000             | 0.864            |
| 4096             | 0.870            |
| 4500             | 0.902            |
| 5000             | 0.946            |
| 6000             | 0.995            |
| 7000             | 0.996            |
| 8000             | 0.996            |
| 8192             | 0.996            |
| 9000             | 0.997            |
| 10000            | 0.997            |
| 16128            | 0.998            |