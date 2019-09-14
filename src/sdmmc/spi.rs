#[doc = "Reader of register SPI"]
pub type R = crate::R<u32, super::SPI>;
#[doc = "Writer for register SPI"]
pub type W = crate::W<u32, super::SPI>;
#[doc = "Register SPI `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_INT_SUPPORT`"]
pub type SPI_INT_SUPPORT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_INT_SUPPORT`"]
pub struct SPI_INT_SUPPORT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_INT_SUPPORT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SPI INT SUPPORT"]
    #[inline(always)]
    pub fn spi_int_support(&self) -> SPI_INT_SUPPORT_R {
        SPI_INT_SUPPORT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI INT SUPPORT"]
    #[inline(always)]
    pub fn spi_int_support(&mut self) -> SPI_INT_SUPPORT_W {
        SPI_INT_SUPPORT_W { w: self }
    }
}
