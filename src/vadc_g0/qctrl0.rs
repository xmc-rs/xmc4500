#[doc = "Register `QCTRL0` reader"]
pub type R = crate::R<Qctrl0Spec>;
#[doc = "Register `QCTRL0` writer"]
pub type W = crate::W<Qctrl0Spec>;
#[doc = "Field `XTSEL` reader - External Trigger Input Selection"]
pub type XtselR = crate::FieldReader;
#[doc = "Field `XTSEL` writer - External Trigger Input Selection"]
pub type XtselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `XTLVL` reader - External Trigger Level"]
pub type XtlvlR = crate::BitReader;
#[doc = "Trigger Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xtmode {
    #[doc = "0: No external trigger"]
    Value1 = 0,
    #[doc = "1: Trigger event upon a falling edge"]
    Value2 = 1,
    #[doc = "2: Trigger event upon a rising edge"]
    Value3 = 2,
    #[doc = "3: Trigger event upon any edge"]
    Value4 = 3,
}
impl From<Xtmode> for u8 {
    #[inline(always)]
    fn from(variant: Xtmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xtmode {
    type Ux = u8;
}
#[doc = "Field `XTMODE` reader - Trigger Operating Mode"]
pub type XtmodeR = crate::FieldReader<Xtmode>;
impl XtmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xtmode {
        match self.bits {
            0 => Xtmode::Value1,
            1 => Xtmode::Value2,
            2 => Xtmode::Value3,
            3 => Xtmode::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "No external trigger"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Xtmode::Value1
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Xtmode::Value2
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Xtmode::Value3
    }
    #[doc = "Trigger event upon any edge"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Xtmode::Value4
    }
}
#[doc = "Field `XTMODE` writer - Trigger Operating Mode"]
pub type XtmodeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Xtmode>;
impl<'a, REG> XtmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No external trigger"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Xtmode::Value1)
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Xtmode::Value2)
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Xtmode::Value3)
    }
    #[doc = "Trigger event upon any edge"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Xtmode::Value4)
    }
}
#[doc = "Write Control for Trigger Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xtwc {
    #[doc = "0: No write access to trigger configuration"]
    Value1 = 0,
    #[doc = "1: Bitfields XTMODE and XTSEL can be written"]
    Value2 = 1,
}
impl From<Xtwc> for bool {
    #[inline(always)]
    fn from(variant: Xtwc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTWC` writer - Write Control for Trigger Configuration"]
pub type XtwcW<'a, REG> = crate::BitWriter<'a, REG, Xtwc>;
impl<'a, REG> XtwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to trigger configuration"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Xtwc::Value1)
    }
    #[doc = "Bitfields XTMODE and XTSEL can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Xtwc::Value2)
    }
}
#[doc = "Field `GTSEL` reader - Gate Input Selection"]
pub type GtselR = crate::FieldReader;
#[doc = "Field `GTSEL` writer - Gate Input Selection"]
pub type GtselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GTLVL` reader - Gate Input Level"]
pub type GtlvlR = crate::BitReader;
#[doc = "Write Control for Gate Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gtwc {
    #[doc = "0: No write access to gate configuration"]
    Value1 = 0,
    #[doc = "1: Bitfield GTSEL can be written"]
    Value2 = 1,
}
impl From<Gtwc> for bool {
    #[inline(always)]
    fn from(variant: Gtwc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTWC` writer - Write Control for Gate Configuration"]
pub type GtwcW<'a, REG> = crate::BitWriter<'a, REG, Gtwc>;
impl<'a, REG> GtwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to gate configuration"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Gtwc::Value1)
    }
    #[doc = "Bitfield GTSEL can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Gtwc::Value2)
    }
}
#[doc = "Timer Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmen {
    #[doc = "0: No timer mode: standard gating mechanism can be used"]
    Value1 = 0,
    #[doc = "1: Timer mode for equidistant sampling enabled: standard gating mechanism must be disabled"]
    Value2 = 1,
}
impl From<Tmen> for bool {
    #[inline(always)]
    fn from(variant: Tmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMEN` reader - Timer Mode Enable"]
pub type TmenR = crate::BitReader<Tmen>;
impl TmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmen {
        match self.bits {
            false => Tmen::Value1,
            true => Tmen::Value2,
        }
    }
    #[doc = "No timer mode: standard gating mechanism can be used"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tmen::Value1
    }
    #[doc = "Timer mode for equidistant sampling enabled: standard gating mechanism must be disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tmen::Value2
    }
}
#[doc = "Field `TMEN` writer - Timer Mode Enable"]
pub type TmenW<'a, REG> = crate::BitWriter<'a, REG, Tmen>;
impl<'a, REG> TmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No timer mode: standard gating mechanism can be used"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmen::Value1)
    }
    #[doc = "Timer mode for equidistant sampling enabled: standard gating mechanism must be disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tmen::Value2)
    }
}
#[doc = "Write Control for Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmwc {
    #[doc = "0: No write access to timer mode"]
    Value1 = 0,
    #[doc = "1: Bitfield TMEN can be written"]
    Value2 = 1,
}
impl From<Tmwc> for bool {
    #[inline(always)]
    fn from(variant: Tmwc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMWC` writer - Write Control for Timer Mode"]
pub type TmwcW<'a, REG> = crate::BitWriter<'a, REG, Tmwc>;
impl<'a, REG> TmwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to timer mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmwc::Value1)
    }
    #[doc = "Bitfield TMEN can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tmwc::Value2)
    }
}
impl R {
    #[doc = "Bits 8:11 - External Trigger Input Selection"]
    #[inline(always)]
    pub fn xtsel(&self) -> XtselR {
        XtselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Trigger Level"]
    #[inline(always)]
    pub fn xtlvl(&self) -> XtlvlR {
        XtlvlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Trigger Operating Mode"]
    #[inline(always)]
    pub fn xtmode(&self) -> XtmodeR {
        XtmodeR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Gate Input Selection"]
    #[inline(always)]
    pub fn gtsel(&self) -> GtselR {
        GtselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Gate Input Level"]
    #[inline(always)]
    pub fn gtlvl(&self) -> GtlvlR {
        GtlvlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 28 - Timer Mode Enable"]
    #[inline(always)]
    pub fn tmen(&self) -> TmenR {
        TmenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:11 - External Trigger Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn xtsel(&mut self) -> XtselW<Qctrl0Spec> {
        XtselW::new(self, 8)
    }
    #[doc = "Bits 13:14 - Trigger Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn xtmode(&mut self) -> XtmodeW<Qctrl0Spec> {
        XtmodeW::new(self, 13)
    }
    #[doc = "Bit 15 - Write Control for Trigger Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn xtwc(&mut self) -> XtwcW<Qctrl0Spec> {
        XtwcW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Gate Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn gtsel(&mut self) -> GtselW<Qctrl0Spec> {
        GtselW::new(self, 16)
    }
    #[doc = "Bit 23 - Write Control for Gate Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn gtwc(&mut self) -> GtwcW<Qctrl0Spec> {
        GtwcW::new(self, 23)
    }
    #[doc = "Bit 28 - Timer Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmen(&mut self) -> TmenW<Qctrl0Spec> {
        TmenW::new(self, 28)
    }
    #[doc = "Bit 31 - Write Control for Timer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmwc(&mut self) -> TmwcW<Qctrl0Spec> {
        TmwcW::new(self, 31)
    }
}
#[doc = "Queue 0 Source Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Qctrl0Spec;
impl crate::RegisterSpec for Qctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qctrl0::R`](R) reader structure"]
impl crate::Readable for Qctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`qctrl0::W`](W) writer structure"]
impl crate::Writable for Qctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QCTRL0 to value 0"]
impl crate::Resettable for Qctrl0Spec {
    const RESET_VALUE: u32 = 0;
}
