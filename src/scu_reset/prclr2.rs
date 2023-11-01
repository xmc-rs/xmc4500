#[doc = "Register `PRCLR2` writer"]
pub type W = crate::W<PRCLR2_SPEC>;
#[doc = "WDT Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<WDTRS_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRS` writer - WDT Reset Clear"]
pub type WDTRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WDTRS_AW>;
impl<'a, REG, const O: u8> WDTRS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTRS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WDTRS_AW::VALUE2)
    }
}
#[doc = "ETH0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<ETH0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: ETH0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0RS` writer - ETH0 Reset Clear"]
pub type ETH0RS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ETH0RS_AW>;
impl<'a, REG, const O: u8> ETH0RS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0RS_AW::VALUE2)
    }
}
#[doc = "DMA0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<DMA0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0RS` writer - DMA0 Reset Clear"]
pub type DMA0RS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA0RS_AW>;
impl<'a, REG, const O: u8> DMA0RS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RS_AW::VALUE2)
    }
}
#[doc = "DMA1 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<DMA1RS_AW> for bool {
    #[inline(always)]
    fn from(variant: DMA1RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1RS` writer - DMA1 Reset Clear"]
pub type DMA1RS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA1RS_AW>;
impl<'a, REG, const O: u8> DMA1RS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RS_AW::VALUE2)
    }
}
#[doc = "FCE Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCERS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<FCERS_AW> for bool {
    #[inline(always)]
    fn from(variant: FCERS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCERS` writer - FCE Reset Clear"]
pub type FCERS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FCERS_AW>;
impl<'a, REG, const O: u8> FCERS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FCERS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FCERS_AW::VALUE2)
    }
}
#[doc = "USB Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<USBRS_AW> for bool {
    #[inline(always)]
    fn from(variant: USBRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRS` writer - USB Reset Clear"]
pub type USBRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USBRS_AW>;
impl<'a, REG, const O: u8> USBRS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USBRS_AW::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USBRS_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 1 - WDT Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrs(&mut self) -> WDTRS_W<PRCLR2_SPEC, 1> {
        WDTRS_W::new(self)
    }
    #[doc = "Bit 2 - ETH0 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eth0rs(&mut self) -> ETH0RS_W<PRCLR2_SPEC, 2> {
        ETH0RS_W::new(self)
    }
    #[doc = "Bit 4 - DMA0 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dma0rs(&mut self) -> DMA0RS_W<PRCLR2_SPEC, 4> {
        DMA0RS_W::new(self)
    }
    #[doc = "Bit 5 - DMA1 Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rs(&mut self) -> DMA1RS_W<PRCLR2_SPEC, 5> {
        DMA1RS_W::new(self)
    }
    #[doc = "Bit 6 - FCE Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn fcers(&mut self) -> FCERS_W<PRCLR2_SPEC, 6> {
        FCERS_W::new(self)
    }
    #[doc = "Bit 7 - USB Reset Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usbrs(&mut self) -> USBRS_W<PRCLR2_SPEC, 7> {
        USBRS_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCU Peripheral 2 Reset Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prclr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRCLR2_SPEC;
impl crate::RegisterSpec for PRCLR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prclr2::W`](W) writer structure"]
impl crate::Writable for PRCLR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRCLR2 to value 0"]
impl crate::Resettable for PRCLR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
