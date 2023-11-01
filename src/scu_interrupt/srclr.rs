#[doc = "Register `SRCLR` writer"]
pub type W = crate::W<SRCLR_SPEC>;
#[doc = "WDT pre-warning Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRWARN_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<PRWARN_AW> for bool {
    #[inline(always)]
    fn from(variant: PRWARN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRWARN` writer - WDT pre-warning Interrupt Clear"]
pub type PRWARN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PRWARN_AW>;
impl<'a, REG, const O: u8> PRWARN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PRWARN_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PRWARN_AW::VALUE2)
    }
}
#[doc = "RTC Periodic Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PI_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<PI_AW> for bool {
    #[inline(always)]
    fn from(variant: PI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PI` writer - RTC Periodic Interrupt Clear"]
pub type PI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PI_AW>;
impl<'a, REG, const O: u8> PI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PI_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PI_AW::VALUE2)
    }
}
#[doc = "RTC Alarm Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AI_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<AI_AW> for bool {
    #[inline(always)]
    fn from(variant: AI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AI` writer - RTC Alarm Interrupt Clear"]
pub type AI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AI_AW>;
impl<'a, REG, const O: u8> AI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AI_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AI_AW::VALUE2)
    }
}
#[doc = "DLR Request Overrun Interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLROVR_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<DLROVR_AW> for bool {
    #[inline(always)]
    fn from(variant: DLROVR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLROVR` writer - DLR Request Overrun Interrupt clear"]
pub type DLROVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DLROVR_AW>;
impl<'a, REG, const O: u8> DLROVR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DLROVR_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DLROVR_AW::VALUE2)
    }
}
#[doc = "HDCLR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCLR_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HDCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: HDCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCLR` writer - HDCLR Mirror Register Update Clear"]
pub type HDCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HDCLR_AW>;
impl<'a, REG, const O: u8> HDCLR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HDCLR_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HDCLR_AW::VALUE2)
    }
}
#[doc = "HDSET Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSET_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HDSET_AW> for bool {
    #[inline(always)]
    fn from(variant: HDSET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSET` writer - HDSET Mirror Register Update Clear"]
pub type HDSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HDSET_AW>;
impl<'a, REG, const O: u8> HDSET_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HDSET_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HDSET_AW::VALUE2)
    }
}
#[doc = "HDCR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCR_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<HDCR_AW> for bool {
    #[inline(always)]
    fn from(variant: HDCR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCR` writer - HDCR Mirror Register Update Clear"]
pub type HDCR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HDCR_AW>;
impl<'a, REG, const O: u8> HDCR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HDCR_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HDCR_AW::VALUE2)
    }
}
#[doc = "OSCSICTRL Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCSICTRL_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<OSCSICTRL_AW> for bool {
    #[inline(always)]
    fn from(variant: OSCSICTRL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSICTRL` writer - OSCSICTRL Mirror Register Update Clear"]
pub type OSCSICTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OSCSICTRL_AW>;
impl<'a, REG, const O: u8> OSCSICTRL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSICTRL_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSICTRL_AW::VALUE2)
    }
}
#[doc = "OSCULCTRL Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCULCTRL_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<OSCULCTRL_AW> for bool {
    #[inline(always)]
    fn from(variant: OSCULCTRL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCULCTRL` writer - OSCULCTRL Mirror Register Update Clear"]
pub type OSCULCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OSCULCTRL_AW>;
impl<'a, REG, const O: u8> OSCULCTRL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OSCULCTRL_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OSCULCTRL_AW::VALUE2)
    }
}
#[doc = "RTC CTR Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_CTR_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_CTR_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_CTR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_CTR` writer - RTC CTR Mirror Register Update Clear"]
pub type RTC_CTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTC_CTR_AW>;
impl<'a, REG, const O: u8> RTC_CTR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_CTR_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_CTR_AW::VALUE2)
    }
}
#[doc = "RTC ATIM0 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_ATIM0_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM0` writer - RTC ATIM0 Mirror Register Update Clear"]
pub type RTC_ATIM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTC_ATIM0_AW>;
impl<'a, REG, const O: u8> RTC_ATIM0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM0_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM0_AW::VALUE2)
    }
}
#[doc = "RTC ATIM1 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM1_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_ATIM1_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM1` writer - RTC ATIM1 Mirror Register Update Clear"]
pub type RTC_ATIM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTC_ATIM1_AW>;
impl<'a, REG, const O: u8> RTC_ATIM1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM1_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_ATIM1_AW::VALUE2)
    }
}
#[doc = "RTC TIM0 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_TIM0_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM0` writer - RTC TIM0 Mirror Register Update Clear"]
pub type RTC_TIM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTC_TIM0_AW>;
impl<'a, REG, const O: u8> RTC_TIM0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM0_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM0_AW::VALUE2)
    }
}
#[doc = "RTC TIM1 Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM1_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RTC_TIM1_AW> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM1` writer - RTC TIM1 Mirror Register Update Clear"]
pub type RTC_TIM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTC_TIM1_AW>;
impl<'a, REG, const O: u8> RTC_TIM1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM1_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTC_TIM1_AW::VALUE2)
    }
}
#[doc = "Retention Memory Mirror Register Update Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMX_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the status bit"]
    VALUE2 = 1,
}
impl From<RMX_AW> for bool {
    #[inline(always)]
    fn from(variant: RMX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMX` writer - Retention Memory Mirror Register Update Clear"]
pub type RMX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RMX_AW>;
impl<'a, REG, const O: u8> RMX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RMX_AW::VALUE1)
    }
    #[doc = "Clear the status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RMX_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn prwarn(&mut self) -> PRWARN_W<SRCLR_SPEC, 0> {
        PRWARN_W::new(self)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PI_W<SRCLR_SPEC, 1> {
        PI_W::new(self)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<SRCLR_SPEC, 2> {
        AI_W::new(self)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn dlrovr(&mut self) -> DLROVR_W<SRCLR_SPEC, 3> {
        DLROVR_W::new(self)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hdclr(&mut self) -> HDCLR_W<SRCLR_SPEC, 17> {
        HDCLR_W::new(self)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hdset(&mut self) -> HDSET_W<SRCLR_SPEC, 18> {
        HDSET_W::new(self)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hdcr(&mut self) -> HDCR_W<SRCLR_SPEC, 19> {
        HDCR_W::new(self)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn oscsictrl(&mut self) -> OSCSICTRL_W<SRCLR_SPEC, 21> {
        OSCSICTRL_W::new(self)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn osculctrl(&mut self) -> OSCULCTRL_W<SRCLR_SPEC, 23> {
        OSCULCTRL_W::new(self)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ctr(&mut self) -> RTC_CTR_W<SRCLR_SPEC, 24> {
        RTC_CTR_W::new(self)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim0(&mut self) -> RTC_ATIM0_W<SRCLR_SPEC, 25> {
        RTC_ATIM0_W::new(self)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim1(&mut self) -> RTC_ATIM1_W<SRCLR_SPEC, 26> {
        RTC_ATIM1_W::new(self)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim0(&mut self) -> RTC_TIM0_W<SRCLR_SPEC, 27> {
        RTC_TIM0_W::new(self)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim1(&mut self) -> RTC_TIM1_W<SRCLR_SPEC, 28> {
        RTC_TIM1_W::new(self)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rmx(&mut self) -> RMX_W<SRCLR_SPEC, 29> {
        RMX_W::new(self)
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
#[doc = "SCU Service Request Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCLR_SPEC;
impl crate::RegisterSpec for SRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`srclr::W`](W) writer structure"]
impl crate::Writable for SRCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCLR to value 0"]
impl crate::Resettable for SRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
