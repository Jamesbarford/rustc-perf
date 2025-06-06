#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    _reserved1: [u8; 0x08],
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dier: crate::Reg<dier::DIER_SPEC>,
    #[doc = "0x10 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x14 - event generation register"]
    pub egr: crate::Reg<egr::EGR_SPEC>,
    _reserved_4_ccmr1: [u8; 0x04],
    _reserved5: [u8; 0x04],
    #[doc = "0x20 - capture/compare enable register"]
    pub ccer: crate::Reg<ccer::CCER_SPEC>,
    #[doc = "0x24 - counter"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x28 - prescaler"]
    pub psc: crate::Reg<psc::PSC_SPEC>,
    #[doc = "0x2c - auto-reload register"]
    pub arr: crate::Reg<arr::ARR_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x34 - capture/compare register"]
    pub ccr: [crate::Reg<ccr::CCR_SPEC>; 1],
}
impl RegisterBlock {
    #[doc = "0x34 - capture/compare register"]
    #[inline(always)]
    pub fn ccr1(&self) -> &crate::Reg<ccr::CCR_SPEC> {
        &self.ccr[0]
    }
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub fn ccmr1_input(&self) -> &crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>)
        }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn ccmr1_output(&self) -> &crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>)
        }
    }
}
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "DIER register accessor: an alias for `Reg<DIER_SPEC>`"]
pub type DIER = crate::Reg<dier::DIER_SPEC>;
#[doc = "DMA/Interrupt enable register"]
pub mod dier;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "EGR register accessor: an alias for `Reg<EGR_SPEC>`"]
pub type EGR = crate::Reg<egr::EGR_SPEC>;
#[doc = "event generation register"]
pub mod egr;
#[doc = "CCMR1_Output register accessor: an alias for `Reg<CCMR1_OUTPUT_SPEC>`"]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>;
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccmr1_output;
#[doc = "CCMR1_Input register accessor: an alias for `Reg<CCMR1_INPUT_SPEC>`"]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>;
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod ccmr1_input;
#[doc = "CCER register accessor: an alias for `Reg<CCER_SPEC>`"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "capture/compare enable register"]
pub mod ccer;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "prescaler"]
pub mod psc;
#[doc = "ARR register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "auto-reload register"]
pub mod arr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "capture/compare register"]
pub mod ccr;
