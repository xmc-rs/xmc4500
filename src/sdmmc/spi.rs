#[doc = "Register `SPI` reader"]
pub type R = crate::R<SpiSpec>;
#[doc = "Register `SPI` writer"]
pub type W = crate::W<SpiSpec>;
#[doc = "Field `SPI_INT_SUPPORT` reader - SPI INT SUPPORT"]
pub type SpiIntSupportR = crate::FieldReader;
#[doc = "Field `SPI_INT_SUPPORT` writer - SPI INT SUPPORT"]
pub type SpiIntSupportW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPI INT SUPPORT"]
    #[inline(always)]
    pub fn spi_int_support(&self) -> SpiIntSupportR {
        SpiIntSupportR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI INT SUPPORT"]
    #[inline(always)]
    #[must_use]
    pub fn spi_int_support(&mut self) -> SpiIntSupportW<SpiSpec> {
        SpiIntSupportW::new(self, 0)
    }
}
#[doc = "SPI Interrupt Support Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSpec;
impl crate::RegisterSpec for SpiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi::R`](R) reader structure"]
impl crate::Readable for SpiSpec {}
#[doc = "`write(|w| ..)` method takes [`spi::W`](W) writer structure"]
impl crate::Writable for SpiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI to value 0"]
impl crate::Resettable for SpiSpec {
    const RESET_VALUE: u32 = 0;
}
