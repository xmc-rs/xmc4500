#[doc = "Register `TRANSMIT_DESCRIPTOR_LIST_ADDRESS` reader"]
pub type R = crate::R<TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC>;
#[doc = "Register `TRANSMIT_DESCRIPTOR_LIST_ADDRESS` writer"]
pub type W = crate::W<TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC>;
#[doc = "Field `TDESLA_32bit` reader - Start of Transmit List"]
pub type TDESLA_32BIT_R = crate::FieldReader<u32>;
#[doc = "Field `TDESLA_32bit` writer - Start of Transmit List"]
pub type TDESLA_32BIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - Start of Transmit List"]
    #[inline(always)]
    pub fn tdesla_32bit(&self) -> TDESLA_32BIT_R {
        TDESLA_32BIT_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Transmit List"]
    #[inline(always)]
    #[must_use]
    pub fn tdesla_32bit(&mut self) -> TDESLA_32BIT_W<TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC, 2> {
        TDESLA_32BIT_W::new(self)
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
#[doc = "Transmit descripter Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_descriptor_list_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_descriptor_list_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC;
impl crate::RegisterSpec for TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`transmit_descriptor_list_address::R`](R) reader structure"]
impl crate::Readable for TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`transmit_descriptor_list_address::W`](W) writer structure"]
impl crate::Writable for TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRANSMIT_DESCRIPTOR_LIST_ADDRESS to value 0"]
impl crate::Resettable for TRANSMIT_DESCRIPTOR_LIST_ADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
