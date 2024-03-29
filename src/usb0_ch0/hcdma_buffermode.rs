#[doc = "Register `HCDMA_BUFFERMODE` reader"]
pub type R = crate::R<HcdmaBuffermodeSpec>;
#[doc = "Register `HCDMA_BUFFERMODE` writer"]
pub type W = crate::W<HcdmaBuffermodeSpec>;
#[doc = "Field `DMAAddr` reader - DMA Address"]
pub type DmaaddrR = crate::FieldReader<u32>;
#[doc = "Field `DMAAddr` writer - DMA Address"]
pub type DmaaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DmaaddrR {
        DmaaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DmaaddrW<HcdmaBuffermodeSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "Host Channel DMA Address Register \\[BUFFERMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma_buffermode::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma_buffermode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcdmaBuffermodeSpec;
impl crate::RegisterSpec for HcdmaBuffermodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma_buffermode::R`](R) reader structure"]
impl crate::Readable for HcdmaBuffermodeSpec {}
#[doc = "`write(|w| ..)` method takes [`hcdma_buffermode::W`](W) writer structure"]
impl crate::Writable for HcdmaBuffermodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCDMA_BUFFERMODE to value 0"]
impl crate::Resettable for HcdmaBuffermodeSpec {
    const RESET_VALUE: u32 = 0;
}
