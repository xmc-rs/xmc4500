#[doc = "Register `PPS_CONTROL` reader"]
pub struct R(crate::R<PPS_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPS_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPS_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPS_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPS_CONTROL` writer"]
pub struct W(crate::W<PPS_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPS_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PPS_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPS_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPSCTRL_PPSCMD` reader - PPSCTRL0 or PPSCMD0"]
pub struct PPSCTRL_PPSCMD_R(crate::FieldReader<u8, u8>);
impl PPSCTRL_PPSCMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPSCTRL_PPSCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPSCTRL_PPSCMD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPSCTRL_PPSCMD` writer - PPSCTRL0 or PPSCMD0"]
pub struct PPSCTRL_PPSCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PPSCTRL_PPSCMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PPSEN0` reader - Flexible PPS Output Mode Enable"]
pub struct PPSEN0_R(crate::FieldReader<bool, bool>);
impl PPSEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPSEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPSEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGTMODSEL0` reader - Target Time Register Mode for PPS0 Output"]
pub struct TRGTMODSEL0_R(crate::FieldReader<u8, u8>);
impl TRGTMODSEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGTMODSEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGTMODSEL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPSCMD1` reader - Flexible PPS1 Output Control"]
pub struct PPSCMD1_R(crate::FieldReader<u8, u8>);
impl PPSCMD1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPSCMD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPSCMD1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGTMODSEL1` reader - Target Time Register Mode for PPS1 Output"]
pub struct TRGTMODSEL1_R(crate::FieldReader<u8, u8>);
impl TRGTMODSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGTMODSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGTMODSEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPSCMD2` reader - Flexible PPS2 Output Control"]
pub struct PPSCMD2_R(crate::FieldReader<u8, u8>);
impl PPSCMD2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPSCMD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPSCMD2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGTMODSEL2` reader - Target Time Register Mode for PPS2 Output"]
pub struct TRGTMODSEL2_R(crate::FieldReader<u8, u8>);
impl TRGTMODSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGTMODSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGTMODSEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPSCMD3` reader - Flexible PPS3 Output Control"]
pub struct PPSCMD3_R(crate::FieldReader<u8, u8>);
impl PPSCMD3_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPSCMD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPSCMD3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGTMODSEL3` reader - Target Time Register Mode for PPS3 Output"]
pub struct TRGTMODSEL3_R(crate::FieldReader<u8, u8>);
impl TRGTMODSEL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGTMODSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGTMODSEL3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - PPSCTRL0 or PPSCMD0"]
    #[inline(always)]
    pub fn ppsctrl_ppscmd(&self) -> PPSCTRL_PPSCMD_R {
        PPSCTRL_PPSCMD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output"]
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0_R {
        TRGTMODSEL0_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Flexible PPS1 Output Control"]
    #[inline(always)]
    pub fn ppscmd1(&self) -> PPSCMD1_R {
        PPSCMD1_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 13:14 - Target Time Register Mode for PPS1 Output"]
    #[inline(always)]
    pub fn trgtmodsel1(&self) -> TRGTMODSEL1_R {
        TRGTMODSEL1_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - Flexible PPS2 Output Control"]
    #[inline(always)]
    pub fn ppscmd2(&self) -> PPSCMD2_R {
        PPSCMD2_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 21:22 - Target Time Register Mode for PPS2 Output"]
    #[inline(always)]
    pub fn trgtmodsel2(&self) -> TRGTMODSEL2_R {
        TRGTMODSEL2_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - Flexible PPS3 Output Control"]
    #[inline(always)]
    pub fn ppscmd3(&self) -> PPSCMD3_R {
        PPSCMD3_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 29:30 - Target Time Register Mode for PPS3 Output"]
    #[inline(always)]
    pub fn trgtmodsel3(&self) -> TRGTMODSEL3_R {
        TRGTMODSEL3_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PPSCTRL0 or PPSCMD0"]
    #[inline(always)]
    pub fn ppsctrl_ppscmd(&mut self) -> PPSCTRL_PPSCMD_W {
        PPSCTRL_PPSCMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPS Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pps_control](index.html) module"]
pub struct PPS_CONTROL_SPEC;
impl crate::RegisterSpec for PPS_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pps_control::R](R) reader structure"]
impl crate::Readable for PPS_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pps_control::W](W) writer structure"]
impl crate::Writable for PPS_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPS_CONTROL to value 0"]
impl crate::Resettable for PPS_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
