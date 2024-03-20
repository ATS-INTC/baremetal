
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
| 1514             | 0.513            |
| 9000             | 0.997            |
| 10000            | 0.997            |
| 16128            | 0.998            |