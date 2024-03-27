
# This is the AxiNic Test crate

The AxiNic consists of a Xilinx AXI DMA and a Xilinx AXI Ethernet 1G/2.5G subsystem. The AXI Ethernet is configured as 1Gbits/s.

We will conduct three test: 

- [x] Poll mode.
- [x] Interrupt mode.
- [x] Rust Future + ATSINTC.

**The result of comparation can be seen in the [assets](./assets/) directory.**

## Poll mode

In this case, we create a buffer and submit it to the AxiNic. Then we wait the transmition to be finished by checking the AXI DMA interrupt status register. We won't submit the next buffer until the previous transmition has been completed. The test result is shown below:

We measure the AxiNic performance by counting the amount of bytes sent within one second. The total data size is 10GB.

## Interrupt mode

The configuration of AxiNic is as the same as the poll mode. However, we won't submit the next buffer until the interrupt of previous transmition has been handled.

We set a interrupt threshold on DMA. DMA will trigger an interrupt only when the number of packets sent reaches the threshold. 

## Rust Future + ATSINTC

The `ATSINTC` is the a interrupt controller integrated with asynchonrous task scheduling. When an interrupt occur, it will directly wake the interrupt handler task without breaking the execution of CPU.

This test case is the same as before, we will send package one by one. But we use the Rust `Future` and `ATSINTC`. The execution change is under the help of Rust compiler while the notification mechanism is provided by `ATSINTC`.

