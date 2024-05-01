#[doc = "Register `TRANSMIT_DESCRIPTOR_LIST_ADDRESS` reader"]
pub type R = crate::R<TransmitDescriptorListAddressSpec>;
#[doc = "Register `TRANSMIT_DESCRIPTOR_LIST_ADDRESS` writer"]
pub type W = crate::W<TransmitDescriptorListAddressSpec>;
#[doc = "Field `TDESLA_32bit` reader - Start of Transmit List"]
pub type Tdesla32bitR = crate::FieldReader<u32>;
#[doc = "Field `TDESLA_32bit` writer - Start of Transmit List"]
pub type Tdesla32bitW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Start of Transmit List"]
    #[inline(always)]
    pub fn tdesla_32bit(&self) -> Tdesla32bitR {
        Tdesla32bitR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Transmit List"]
    #[inline(always)]
    #[must_use]
    pub fn tdesla_32bit(&mut self) -> Tdesla32bitW<TransmitDescriptorListAddressSpec> {
        Tdesla32bitW::new(self, 2)
    }
}
#[doc = "Transmit descripter Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_descriptor_list_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_descriptor_list_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TransmitDescriptorListAddressSpec;
impl crate::RegisterSpec for TransmitDescriptorListAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`transmit_descriptor_list_address::R`](R) reader structure"]
impl crate::Readable for TransmitDescriptorListAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`transmit_descriptor_list_address::W`](W) writer structure"]
impl crate::Writable for TransmitDescriptorListAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRANSMIT_DESCRIPTOR_LIST_ADDRESS to value 0"]
impl crate::Resettable for TransmitDescriptorListAddressSpec {
    const RESET_VALUE: u32 = 0;
}
