#[doc = "Register `QMR0` reader"]
pub type R = crate::R<QMR0_SPEC>;
#[doc = "Register `QMR0` writer"]
pub type W = crate::W<QMR0_SPEC>;
#[doc = "Field `ENGT` reader - Enable Gate"]
pub type ENGT_R = crate::FieldReader<ENGT_A>;
#[doc = "Enable Gate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENGT_A {
    #[doc = "0: No conversion requests are issued"]
    VALUE1 = 0,
    #[doc = "1: Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register"]
    VALUE2 = 1,
    #[doc = "2: Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 1"]
    VALUE3 = 2,
    #[doc = "3: Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 0"]
    VALUE4 = 3,
}
impl From<ENGT_A> for u8 {
    #[inline(always)]
    fn from(variant: ENGT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ENGT_A {
    type Ux = u8;
}
impl ENGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENGT_A {
        match self.bits {
            0 => ENGT_A::VALUE1,
            1 => ENGT_A::VALUE2,
            2 => ENGT_A::VALUE3,
            3 => ENGT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "No conversion requests are issued"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENGT_A::VALUE1
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENGT_A::VALUE2
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 1"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ENGT_A::VALUE3
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 0"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ENGT_A::VALUE4
    }
}
#[doc = "Field `ENGT` writer - Enable Gate"]
pub type ENGT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, ENGT_A>;
impl<'a, REG, const O: u8> ENGT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No conversion requests are issued"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ENGT_A::VALUE1)
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ENGT_A::VALUE2)
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 1"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ENGT_A::VALUE3)
    }
    #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx = 0"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(ENGT_A::VALUE4)
    }
}
#[doc = "Field `ENTR` reader - Enable External Trigger"]
pub type ENTR_R = crate::BitReader<ENTR_A>;
#[doc = "Enable External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENTR_A {
    #[doc = "0: External trigger disabled"]
    VALUE1 = 0,
    #[doc = "1: The selected edge at the selected trigger input signal REQTR generates the trigger event"]
    VALUE2 = 1,
}
impl From<ENTR_A> for bool {
    #[inline(always)]
    fn from(variant: ENTR_A) -> Self {
        variant as u8 != 0
    }
}
impl ENTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENTR_A {
        match self.bits {
            false => ENTR_A::VALUE1,
            true => ENTR_A::VALUE2,
        }
    }
    #[doc = "External trigger disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENTR_A::VALUE1
    }
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the trigger event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENTR_A::VALUE2
    }
}
#[doc = "Field `ENTR` writer - Enable External Trigger"]
pub type ENTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENTR_A>;
impl<'a, REG, const O: u8> ENTR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External trigger disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ENTR_A::VALUE1)
    }
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the trigger event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ENTR_A::VALUE2)
    }
}
#[doc = "Clear Valid Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRV_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: The next pending valid queue entry in the sequence and the event flag EV are cleared. If there is a valid entry in the queue backup register (QBUR.V = 1), this entry is cleared, otherwise the entry in queue register 0 is cleared."]
    VALUE2 = 1,
}
impl From<CLRV_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRV` writer - Clear Valid Bit"]
pub type CLRV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CLRV_AW>;
impl<'a, REG, const O: u8> CLRV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CLRV_AW::VALUE1)
    }
    #[doc = "The next pending valid queue entry in the sequence and the event flag EV are cleared. If there is a valid entry in the queue backup register (QBUR.V = 1), this entry is cleared, otherwise the entry in queue register 0 is cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CLRV_AW::VALUE2)
    }
}
#[doc = "Trigger Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TREV_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Generate a trigger event by software"]
    VALUE2 = 1,
}
impl From<TREV_AW> for bool {
    #[inline(always)]
    fn from(variant: TREV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TREV` writer - Trigger Event"]
pub type TREV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TREV_AW>;
impl<'a, REG, const O: u8> TREV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TREV_AW::VALUE1)
    }
    #[doc = "Generate a trigger event by software"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TREV_AW::VALUE2)
    }
}
#[doc = "Flush Queue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLUSH_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear all queue entries (including backup stage) and the event flag EV. The queue contains no more valid entry."]
    VALUE2 = 1,
}
impl From<FLUSH_AW> for bool {
    #[inline(always)]
    fn from(variant: FLUSH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSH` writer - Flush Queue"]
pub type FLUSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FLUSH_AW>;
impl<'a, REG, const O: u8> FLUSH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FLUSH_AW::VALUE1)
    }
    #[doc = "Clear all queue entries (including backup stage) and the event flag EV. The queue contains no more valid entry."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FLUSH_AW::VALUE2)
    }
}
#[doc = "Clear Event Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEV_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear bit EV"]
    VALUE2 = 1,
}
impl From<CEV_AW> for bool {
    #[inline(always)]
    fn from(variant: CEV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEV` writer - Clear Event Flag"]
pub type CEV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CEV_AW>;
impl<'a, REG, const O: u8> CEV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CEV_AW::VALUE1)
    }
    #[doc = "Clear bit EV"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CEV_AW::VALUE2)
    }
}
#[doc = "Field `RPTDIS` reader - Repeat Disable"]
pub type RPTDIS_R = crate::BitReader<RPTDIS_A>;
#[doc = "Repeat Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPTDIS_A {
    #[doc = "0: A cancelled conversion is repeated"]
    VALUE1 = 0,
    #[doc = "1: A cancelled conversion is discarded"]
    VALUE2 = 1,
}
impl From<RPTDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RPTDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl RPTDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPTDIS_A {
        match self.bits {
            false => RPTDIS_A::VALUE1,
            true => RPTDIS_A::VALUE2,
        }
    }
    #[doc = "A cancelled conversion is repeated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RPTDIS_A::VALUE1
    }
    #[doc = "A cancelled conversion is discarded"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RPTDIS_A::VALUE2
    }
}
#[doc = "Field `RPTDIS` writer - Repeat Disable"]
pub type RPTDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RPTDIS_A>;
impl<'a, REG, const O: u8> RPTDIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A cancelled conversion is repeated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RPTDIS_A::VALUE1)
    }
    #[doc = "A cancelled conversion is discarded"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RPTDIS_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline(always)]
    pub fn engt(&self) -> ENGT_R {
        ENGT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline(always)]
    pub fn entr(&self) -> ENTR_R {
        ENTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline(always)]
    pub fn rptdis(&self) -> RPTDIS_R {
        RPTDIS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline(always)]
    #[must_use]
    pub fn engt(&mut self) -> ENGT_W<QMR0_SPEC, 0> {
        ENGT_W::new(self)
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn entr(&mut self) -> ENTR_W<QMR0_SPEC, 2> {
        ENTR_W::new(self)
    }
    #[doc = "Bit 8 - Clear Valid Bit"]
    #[inline(always)]
    #[must_use]
    pub fn clrv(&mut self) -> CLRV_W<QMR0_SPEC, 8> {
        CLRV_W::new(self)
    }
    #[doc = "Bit 9 - Trigger Event"]
    #[inline(always)]
    #[must_use]
    pub fn trev(&mut self) -> TREV_W<QMR0_SPEC, 9> {
        TREV_W::new(self)
    }
    #[doc = "Bit 10 - Flush Queue"]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FLUSH_W<QMR0_SPEC, 10> {
        FLUSH_W::new(self)
    }
    #[doc = "Bit 11 - Clear Event Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cev(&mut self) -> CEV_W<QMR0_SPEC, 11> {
        CEV_W::new(self)
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rptdis(&mut self) -> RPTDIS_W<QMR0_SPEC, 16> {
        RPTDIS_W::new(self)
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
#[doc = "Queue 0 Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qmr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qmr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QMR0_SPEC;
impl crate::RegisterSpec for QMR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qmr0::R`](R) reader structure"]
impl crate::Readable for QMR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qmr0::W`](W) writer structure"]
impl crate::Writable for QMR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QMR0 to value 0"]
impl crate::Resettable for QMR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
