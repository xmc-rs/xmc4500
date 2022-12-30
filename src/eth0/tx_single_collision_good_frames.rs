#[doc = "Register `TX_SINGLE_COLLISION_GOOD_FRAMES` reader"]
pub struct R(crate::R<TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXSNGLCOLG` reader - This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode."]
pub type TXSNGLCOLG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode."]
    #[inline(always)]
    pub fn txsnglcolg(&self) -> TXSNGLCOLG_R {
        TXSNGLCOLG_R::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Frames Transmitted after Single Collision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_single_collision_good_frames](index.html) module"]
pub struct TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC;
impl crate::RegisterSpec for TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_single_collision_good_frames::R](R) reader structure"]
impl crate::Readable for TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_SINGLE_COLLISION_GOOD_FRAMES to value 0"]
impl crate::Resettable for TX_SINGLE_COLLISION_GOOD_FRAMES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
