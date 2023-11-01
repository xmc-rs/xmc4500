#[doc = "Register `PRSET1` writer"]
pub type W = crate::W<PRSET1_SPEC>;
#[doc = "CCU43 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU43RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<CCU43RS_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU43RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU43RS` writer - CCU43 Reset Assert"]
pub type CCU43RS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCU43RS_AW>;
impl<'a, REG, const O: u8> CCU43RS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU43RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCU43RS_AW::VALUE2)
    }
}
#[doc = "LEDTS Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEDTSCU0RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<LEDTSCU0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0RS` writer - LEDTS Reset Assert"]
pub type LEDTSCU0RS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LEDTSCU0RS_AW>;
impl<'a, REG, const O: u8> LEDTSCU0RS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LEDTSCU0RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LEDTSCU0RS_AW::VALUE2)
    }
}
#[doc = "MultiCAN Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCAN0RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<MCAN0RS_AW> for bool {
    #[inline(always)]
    fn from(variant: MCAN0RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0RS` writer - MultiCAN Reset Assert"]
pub type MCAN0RS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MCAN0RS_AW>;
impl<'a, REG, const O: u8> MCAN0RS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MCAN0RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MCAN0RS_AW::VALUE2)
    }
}
#[doc = "DAC Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<DACRS_AW> for bool {
    #[inline(always)]
    fn from(variant: DACRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACRS` writer - DAC Reset Assert"]
pub type DACRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DACRS_AW>;
impl<'a, REG, const O: u8> DACRS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DACRS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DACRS_AW::VALUE2)
    }
}
#[doc = "MMC Interface Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCIRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<MMCIRS_AW> for bool {
    #[inline(always)]
    fn from(variant: MMCIRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCIRS` writer - MMC Interface Reset Assert"]
pub type MMCIRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MMCIRS_AW>;
impl<'a, REG, const O: u8> MMCIRS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MMCIRS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MMCIRS_AW::VALUE2)
    }
}
#[doc = "USIC1 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC1RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<USIC1RS_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC1RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1RS` writer - USIC1 Reset Assert"]
pub type USIC1RS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USIC1RS_AW>;
impl<'a, REG, const O: u8> USIC1RS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1RS_AW::VALUE2)
    }
}
#[doc = "USIC2 Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC2RS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<USIC2RS_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC2RS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC2RS` writer - USIC2 Reset Assert"]
pub type USIC2RS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USIC2RS_AW>;
impl<'a, REG, const O: u8> USIC2RS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC2RS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC2RS_AW::VALUE2)
    }
}
#[doc = "PORTS Reset Assert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPORTSRS_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Assert reset"]
    VALUE2 = 1,
}
impl From<PPORTSRS_AW> for bool {
    #[inline(always)]
    fn from(variant: PPORTSRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTSRS` writer - PORTS Reset Assert"]
pub type PPORTSRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PPORTSRS_AW>;
impl<'a, REG, const O: u8> PPORTSRS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PPORTSRS_AW::VALUE1)
    }
    #[doc = "Assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PPORTSRS_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - CCU43 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ccu43rs(&mut self) -> CCU43RS_W<PRSET1_SPEC, 0> {
        CCU43RS_W::new(self)
    }
    #[doc = "Bit 3 - LEDTS Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn ledtscu0rs(&mut self) -> LEDTSCU0RS_W<PRSET1_SPEC, 3> {
        LEDTSCU0RS_W::new(self)
    }
    #[doc = "Bit 4 - MultiCAN Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn mcan0rs(&mut self) -> MCAN0RS_W<PRSET1_SPEC, 4> {
        MCAN0RS_W::new(self)
    }
    #[doc = "Bit 5 - DAC Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn dacrs(&mut self) -> DACRS_W<PRSET1_SPEC, 5> {
        DACRS_W::new(self)
    }
    #[doc = "Bit 6 - MMC Interface Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn mmcirs(&mut self) -> MMCIRS_W<PRSET1_SPEC, 6> {
        MMCIRS_W::new(self)
    }
    #[doc = "Bit 7 - USIC1 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn usic1rs(&mut self) -> USIC1RS_W<PRSET1_SPEC, 7> {
        USIC1RS_W::new(self)
    }
    #[doc = "Bit 8 - USIC2 Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn usic2rs(&mut self) -> USIC2RS_W<PRSET1_SPEC, 8> {
        USIC2RS_W::new(self)
    }
    #[doc = "Bit 9 - PORTS Reset Assert"]
    #[inline(always)]
    #[must_use]
    pub fn pportsrs(&mut self) -> PPORTSRS_W<PRSET1_SPEC, 9> {
        PPORTSRS_W::new(self)
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
#[doc = "RCU Peripheral 1 Reset Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prset1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSET1_SPEC;
impl crate::RegisterSpec for PRSET1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prset1::W`](W) writer structure"]
impl crate::Writable for PRSET1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSET1 to value 0"]
impl crate::Resettable for PRSET1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
