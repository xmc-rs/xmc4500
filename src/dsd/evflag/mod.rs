#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVFLAG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `RESEV0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESEV0R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "A new result has been stored in register RESMx"]
    VALUE2,
}
impl RESEV0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RESEV0R::VALUE1 => false,
            RESEV0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESEV0R {
        match value {
            false => RESEV0R::VALUE1,
            true => RESEV0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RESEV0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RESEV0R::VALUE2
    }
}
#[doc = "Possible values of the field `RESEV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESEV1R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "A new result has been stored in register RESMx"]
    VALUE2,
}
impl RESEV1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RESEV1R::VALUE1 => false,
            RESEV1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESEV1R {
        match value {
            false => RESEV1R::VALUE1,
            true => RESEV1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RESEV1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RESEV1R::VALUE2
    }
}
#[doc = "Possible values of the field `RESEV2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESEV2R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "A new result has been stored in register RESMx"]
    VALUE2,
}
impl RESEV2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RESEV2R::VALUE1 => false,
            RESEV2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESEV2R {
        match value {
            false => RESEV2R::VALUE1,
            true => RESEV2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RESEV2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RESEV2R::VALUE2
    }
}
#[doc = "Possible values of the field `RESEV3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESEV3R {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "A new result has been stored in register RESMx"]
    VALUE2,
}
impl RESEV3R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RESEV3R::VALUE1 => false,
            RESEV3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESEV3R {
        match value {
            false => RESEV3R::VALUE1,
            true => RESEV3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RESEV3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RESEV3R::VALUE2
    }
}
#[doc = "Possible values of the field `ALEV0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV0R {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ALEV0R::VALUE1 => false,
            ALEV0R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALEV0R {
        match value {
            false => ALEV0R::VALUE1,
            true => ALEV0R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALEV0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALEV0R::VALUE2
    }
}
#[doc = "Possible values of the field `ALEV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV1R {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ALEV1R::VALUE1 => false,
            ALEV1R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALEV1R {
        match value {
            false => ALEV1R::VALUE1,
            true => ALEV1R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALEV1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALEV1R::VALUE2
    }
}
#[doc = "Possible values of the field `ALEV2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV2R {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ALEV2R::VALUE1 => false,
            ALEV2R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALEV2R {
        match value {
            false => ALEV2R::VALUE1,
            true => ALEV2R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALEV2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALEV2R::VALUE2
    }
}
#[doc = "Possible values of the field `ALEV3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV3R {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV3R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ALEV3R::VALUE1 => false,
            ALEV3R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALEV3R {
        match value {
            false => ALEV3R::VALUE1,
            true => ALEV3R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALEV3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALEV3R::VALUE2
    }
}
#[doc = "Possible values of the field `ALEV4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV4R {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV4R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ALEV4R::VALUE1 => false,
            ALEV4R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALEV4R {
        match value {
            false => ALEV4R::VALUE1,
            true => ALEV4R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALEV4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALEV4R::VALUE2
    }
}
#[doc = "Possible values of the field `ALEV5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV5R {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV5R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ALEV5R::VALUE1 => false,
            ALEV5R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALEV5R {
        match value {
            false => ALEV5R::VALUE1,
            true => ALEV5R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALEV5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALEV5R::VALUE2
    }
}
#[doc = "Possible values of the field `ALEV6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV6R {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV6R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ALEV6R::VALUE1 => false,
            ALEV6R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALEV6R {
        match value {
            false => ALEV6R::VALUE1,
            true => ALEV6R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALEV6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALEV6R::VALUE2
    }
}
#[doc = "Possible values of the field `ALEV7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV7R {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV7R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ALEV7R::VALUE1 => false,
            ALEV7R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALEV7R {
        match value {
            false => ALEV7R::VALUE1,
            true => ALEV7R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALEV7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALEV7R::VALUE2
    }
}
#[doc = "Possible values of the field `ALEV8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV8R {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV8R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ALEV8R::VALUE1 => false,
            ALEV8R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALEV8R {
        match value {
            false => ALEV8R::VALUE1,
            true => ALEV8R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALEV8R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALEV8R::VALUE2
    }
}
#[doc = "Possible values of the field `ALEV9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALEV9R {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV9R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ALEV9R::VALUE1 => false,
            ALEV9R::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALEV9R {
        match value {
            false => ALEV9R::VALUE1,
            true => ALEV9R::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ALEV9R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ALEV9R::VALUE2
    }
}
#[doc = "Values that can be written to the field `RESEV0`"]
pub enum RESEV0W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "A new result has been stored in register RESMx"]
    VALUE2,
}
impl RESEV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESEV0W::VALUE1 => false,
            RESEV0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESEV0W<'a> {
    w: &'a mut W,
}
impl<'a> _RESEV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESEV0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEV0W::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEV0W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESEV1`"]
pub enum RESEV1W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "A new result has been stored in register RESMx"]
    VALUE2,
}
impl RESEV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESEV1W::VALUE1 => false,
            RESEV1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESEV1W<'a> {
    w: &'a mut W,
}
impl<'a> _RESEV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESEV1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEV1W::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEV1W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESEV2`"]
pub enum RESEV2W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "A new result has been stored in register RESMx"]
    VALUE2,
}
impl RESEV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESEV2W::VALUE1 => false,
            RESEV2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESEV2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESEV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESEV2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEV2W::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEV2W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESEV3`"]
pub enum RESEV3W {
    #[doc = "No result event"]
    VALUE1,
    #[doc = "A new result has been stored in register RESMx"]
    VALUE2,
}
impl RESEV3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESEV3W::VALUE1 => false,
            RESEV3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESEV3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESEV3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESEV3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RESEV3W::VALUE1)
    }
    #[doc = "A new result has been stored in register RESMx"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RESEV3W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALEV0`"]
pub enum ALEV0W {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEV0W::VALUE1 => false,
            ALEV0W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEV0W<'a> {
    w: &'a mut W,
}
impl<'a> _ALEV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEV0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No alarm event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV0W::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV0W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALEV1`"]
pub enum ALEV1W {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEV1W::VALUE1 => false,
            ALEV1W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEV1W<'a> {
    w: &'a mut W,
}
impl<'a> _ALEV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEV1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No alarm event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV1W::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV1W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALEV2`"]
pub enum ALEV2W {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEV2W::VALUE1 => false,
            ALEV2W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEV2W<'a> {
    w: &'a mut W,
}
impl<'a> _ALEV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEV2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No alarm event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV2W::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV2W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALEV3`"]
pub enum ALEV3W {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEV3W::VALUE1 => false,
            ALEV3W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEV3W<'a> {
    w: &'a mut W,
}
impl<'a> _ALEV3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEV3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No alarm event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV3W::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV3W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALEV4`"]
pub enum ALEV4W {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEV4W::VALUE1 => false,
            ALEV4W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEV4W<'a> {
    w: &'a mut W,
}
impl<'a> _ALEV4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEV4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No alarm event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV4W::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV4W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALEV5`"]
pub enum ALEV5W {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEV5W::VALUE1 => false,
            ALEV5W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEV5W<'a> {
    w: &'a mut W,
}
impl<'a> _ALEV5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEV5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No alarm event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV5W::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV5W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALEV6`"]
pub enum ALEV6W {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEV6W::VALUE1 => false,
            ALEV6W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEV6W<'a> {
    w: &'a mut W,
}
impl<'a> _ALEV6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEV6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No alarm event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV6W::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV6W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALEV7`"]
pub enum ALEV7W {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEV7W::VALUE1 => false,
            ALEV7W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEV7W<'a> {
    w: &'a mut W,
}
impl<'a> _ALEV7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEV7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No alarm event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV7W::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV7W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALEV8`"]
pub enum ALEV8W {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEV8W::VALUE1 => false,
            ALEV8W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEV8W<'a> {
    w: &'a mut W,
}
impl<'a> _ALEV8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEV8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No alarm event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV8W::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV8W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALEV9`"]
pub enum ALEV9W {
    #[doc = "No alarm event"]
    VALUE1,
    #[doc = "An alarm event has occurred"]
    VALUE2,
}
impl ALEV9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALEV9W::VALUE1 => false,
            ALEV9W::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALEV9W<'a> {
    w: &'a mut W,
}
impl<'a> _ALEV9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALEV9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No alarm event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALEV9W::VALUE1)
    }
    #[doc = "An alarm event has occurred"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALEV9W::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Result Event"]
    #[inline]
    pub fn resev0(&self) -> RESEV0R {
        RESEV0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Result Event"]
    #[inline]
    pub fn resev1(&self) -> RESEV1R {
        RESEV1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Result Event"]
    #[inline]
    pub fn resev2(&self) -> RESEV2R {
        RESEV2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Result Event"]
    #[inline]
    pub fn resev3(&self) -> RESEV3R {
        RESEV3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Alarm Event"]
    #[inline]
    pub fn alev0(&self) -> ALEV0R {
        ALEV0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Alarm Event"]
    #[inline]
    pub fn alev1(&self) -> ALEV1R {
        ALEV1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Alarm Event"]
    #[inline]
    pub fn alev2(&self) -> ALEV2R {
        ALEV2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Alarm Event"]
    #[inline]
    pub fn alev3(&self) -> ALEV3R {
        ALEV3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Alarm Event"]
    #[inline]
    pub fn alev4(&self) -> ALEV4R {
        ALEV4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Alarm Event"]
    #[inline]
    pub fn alev5(&self) -> ALEV5R {
        ALEV5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Alarm Event"]
    #[inline]
    pub fn alev6(&self) -> ALEV6R {
        ALEV6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Alarm Event"]
    #[inline]
    pub fn alev7(&self) -> ALEV7R {
        ALEV7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Alarm Event"]
    #[inline]
    pub fn alev8(&self) -> ALEV8R {
        ALEV8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Alarm Event"]
    #[inline]
    pub fn alev9(&self) -> ALEV9R {
        ALEV9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Result Event"]
    #[inline]
    pub fn resev0(&mut self) -> _RESEV0W {
        _RESEV0W { w: self }
    }
    #[doc = "Bit 1 - Result Event"]
    #[inline]
    pub fn resev1(&mut self) -> _RESEV1W {
        _RESEV1W { w: self }
    }
    #[doc = "Bit 2 - Result Event"]
    #[inline]
    pub fn resev2(&mut self) -> _RESEV2W {
        _RESEV2W { w: self }
    }
    #[doc = "Bit 3 - Result Event"]
    #[inline]
    pub fn resev3(&mut self) -> _RESEV3W {
        _RESEV3W { w: self }
    }
    #[doc = "Bit 16 - Alarm Event"]
    #[inline]
    pub fn alev0(&mut self) -> _ALEV0W {
        _ALEV0W { w: self }
    }
    #[doc = "Bit 17 - Alarm Event"]
    #[inline]
    pub fn alev1(&mut self) -> _ALEV1W {
        _ALEV1W { w: self }
    }
    #[doc = "Bit 18 - Alarm Event"]
    #[inline]
    pub fn alev2(&mut self) -> _ALEV2W {
        _ALEV2W { w: self }
    }
    #[doc = "Bit 19 - Alarm Event"]
    #[inline]
    pub fn alev3(&mut self) -> _ALEV3W {
        _ALEV3W { w: self }
    }
    #[doc = "Bit 20 - Alarm Event"]
    #[inline]
    pub fn alev4(&mut self) -> _ALEV4W {
        _ALEV4W { w: self }
    }
    #[doc = "Bit 21 - Alarm Event"]
    #[inline]
    pub fn alev5(&mut self) -> _ALEV5W {
        _ALEV5W { w: self }
    }
    #[doc = "Bit 22 - Alarm Event"]
    #[inline]
    pub fn alev6(&mut self) -> _ALEV6W {
        _ALEV6W { w: self }
    }
    #[doc = "Bit 23 - Alarm Event"]
    #[inline]
    pub fn alev7(&mut self) -> _ALEV7W {
        _ALEV7W { w: self }
    }
    #[doc = "Bit 24 - Alarm Event"]
    #[inline]
    pub fn alev8(&mut self) -> _ALEV8W {
        _ALEV8W { w: self }
    }
    #[doc = "Bit 25 - Alarm Event"]
    #[inline]
    pub fn alev9(&mut self) -> _ALEV9W {
        _ALEV9W { w: self }
    }
}
