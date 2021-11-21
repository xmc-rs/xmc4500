#[doc = "Register `SPI` reader"]
pub struct R(crate::R<SPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI` writer"]
pub struct W(crate::W<SPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SPEC>;
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
impl From<crate::W<SPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_INT_SUPPORT` reader - SPI INT SUPPORT"]
pub struct SPI_INT_SUPPORT_R(crate::FieldReader<u8, u8>);
impl SPI_INT_SUPPORT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI_INT_SUPPORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_INT_SUPPORT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_INT_SUPPORT` writer - SPI INT SUPPORT"]
pub struct SPI_INT_SUPPORT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_INT_SUPPORT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Interrupt Support Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi](index.html) module"]
pub struct SPI_SPEC;
impl crate::RegisterSpec for SPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi::R](R) reader structure"]
impl crate::Readable for SPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi::W](W) writer structure"]
impl crate::Writable for SPI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI to value 0"]
impl crate::Resettable for SPI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
