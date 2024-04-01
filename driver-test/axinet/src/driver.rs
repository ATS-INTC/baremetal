use alloc::sync::Arc;
use axi_dma::{AxiDMAResult, AxiDma};
use axi_ethernet::{AxiEthernet, LinkStatus, XAE_BROADCAST_OPTION, XAE_JUMBO_OPTION};
use spin::{Lazy, Mutex};

const DMA_BASE: usize = 0x6010_0000;
const ETH_BASE: usize = 0x60140000;
const BD_CNT: usize = 1024;
const MAC_ADDR: [u8; 6] = [0x00, 0x0A, 0x35, 0x01, 0x02, 0x03];

#[derive(Clone)]
pub struct AxiNet {
    pub dma: Arc<AxiDma>,
    pub eth: Arc<Mutex<AxiEthernet>>,
}

impl AxiNet {
    pub const fn new(dma: Arc<AxiDma>, eth: Arc<Mutex<AxiEthernet>>) -> Self {
        Self { dma, eth }
    }
}

impl Default for AxiNet {
    fn default() -> Self {
        AxiNet::new(AXI_DMA.clone(), AXI_ETH.clone())
    }
}

pub static AXI_ETH: Lazy<Arc<Mutex<AxiEthernet>>> =
    Lazy::new(|| Arc::new(Mutex::new(AxiEthernet::new(ETH_BASE, DMA_BASE))));
pub static AXI_DMA: Lazy<Arc<AxiDma>> = Lazy::new(|| Arc::new(AxiDma::default()));

pub fn init() -> AxiDMAResult {
    /**************** DMA init ***************/
    AXI_DMA.reset()?;
    // enable cyclic mode
    AXI_DMA.cyclic_enable();
    // init cyclic block descriptor
    AXI_DMA.tx_channel_create(BD_CNT)?;
    AXI_DMA.rx_channel_create(BD_CNT)?;
    // enable tx & rx intr
    AXI_DMA.intr_enable();

    /**************** Ethernet init ***************/
    let mut eth = AXI_ETH.lock();
    eth.reset();
    let options = eth.get_options();
    eth.set_options(options | XAE_JUMBO_OPTION);
    eth.clear_options(XAE_BROADCAST_OPTION);
    eth.detect_phy();
    let speed = eth.get_phy_speed_ksz9031();
    eth.set_operating_speed(speed as u16);
    if speed == 0 {
        eth.link_status = LinkStatus::EthLinkDown;
    } else {
        eth.link_status = LinkStatus::EthLinkUp;
    }
    eth.set_mac_address(&MAC_ADDR);
    eth.enable_rx_memovr();
    eth.clear_rx_memovr();
    eth.enable_rx_rject();
    eth.clear_rx_rject();
    eth.enable_rx_cmplt();
    eth.clear_rx_cmplt();
    eth.enable_tx_cmplt();
    eth.clear_tx_cmplt();
    eth.start();
    log::info!("NIC initialization has done!");
    Ok(())
}


