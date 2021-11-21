#[doc = "Register `RX_65TO127OCTETS_FRAMES_GOOD_BAD` reader"]
pub struct R(crate::R<RX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX65_127OCTGB` reader - This field indicates the number of received good and bad frames received with length between 65 and 127 (inclusive) bytes, exclusive of preamble."]
pub struct RX65_127OCTGB_R(crate::FieldReader<u32, u32>);
impl RX65_127OCTGB_R {
    pub(crate) fn new(bits: u32) -> Self {
        RX65_127OCTGB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX65_127OCTGB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames received with length between 65 and 127 (inclusive) bytes, exclusive of preamble."]
    #[inline(always)]
    pub fn rx65_127octgb(&self) -> RX65_127OCTGB_R {
        RX65_127OCTGB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receive Frame Count for Good and Bad 65 to 127 Bytes Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_65to127octets_frames_good_bad](index.html) module"]
pub struct RX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for RX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_65to127octets_frames_good_bad::R](R) reader structure"]
impl crate::Readable for RX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_65TO127OCTETS_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for RX_65TO127OCTETS_FRAMES_GOOD_BAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
