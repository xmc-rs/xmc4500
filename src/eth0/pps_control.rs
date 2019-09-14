#[doc = "Reader of register PPS_CONTROL"]
pub type R = crate::R<u32, super::PPS_CONTROL>;
#[doc = "Writer for register PPS_CONTROL"]
pub type W = crate::W<u32, super::PPS_CONTROL>;
#[doc = "Register PPS_CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::PPS_CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PPSCTRL_PPSCMD`"]
pub type PPSCTRL_PPSCMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PPSCTRL_PPSCMD`"]
pub struct PPSCTRL_PPSCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PPSCTRL_PPSCMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PPSEN0`"]
pub type PPSEN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRGTMODSEL0`"]
pub type TRGTMODSEL0_R = crate::R<u8, u8>;
#[doc = "Reader of field `PPSCMD1`"]
pub type PPSCMD1_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRGTMODSEL1`"]
pub type TRGTMODSEL1_R = crate::R<u8, u8>;
#[doc = "Reader of field `PPSCMD2`"]
pub type PPSCMD2_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRGTMODSEL2`"]
pub type TRGTMODSEL2_R = crate::R<u8, u8>;
#[doc = "Reader of field `PPSCMD3`"]
pub type PPSCMD3_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRGTMODSEL3`"]
pub type TRGTMODSEL3_R = crate::R<u8, u8>;
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
}
