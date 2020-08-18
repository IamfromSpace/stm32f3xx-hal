use core::intrinsics::transmute;
use core::ops::Deref;
use cortex_m::interrupt;
use mutex_trait::prelude::Mutex;

pub struct GlobalInterrupt<T> {
    t: T,
}

// BE VERY CAREFUL WITH THIS!
// These traits only hold if there was exactly zero or one original refererce to reg.
// note: This only works with 0 sized structs
macro_rules! global_interrupt {
    ($REG:ident) => {
        impl From<$REG> for GlobalInterrupt<$REG> {
            fn from(t: $REG) -> GlobalInterrupt<$REG> {
                GlobalInterrupt { t }
            }
        }

        impl Deref for GlobalInterrupt<$REG> {
            type Target = $REG;

            fn deref(&self) -> &$REG {
                &self.t
            }
        }

        impl Mutex for GlobalInterrupt<$REG> {
            type Data = $REG;

            fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
                interrupt::free(|_| f(&mut self.t))
            }
        }

        impl Clone for GlobalInterrupt<$REG> {
            fn clone(&self) -> Self {
                // justification: There is exactly one $REG provided to the user (via
                // Peripherals::take()), and if any GlobalInterrupt<$REG> exist, the original must
                // have been taken to construct it.  Since any mutation of any GlobalInterrupt<$REG>
                // is atomic, it's safe to make as many as we'd like.
                // TODO: Does this hold in the face of Deref??
                let rcc = unsafe { transmute::<(), $REG>(()) };
                GlobalInterrupt { t: rcc }
            }
        }
    };
}

use crate::pac::{TIM15, TIM16, TIM17, TIM2};
global_interrupt!(TIM15);
global_interrupt!(TIM16);
global_interrupt!(TIM17);
global_interrupt!(TIM2);

// TODO: a lot more...

// TODO: Ideally this macro would execute in a place that already has these guards
#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f334",
    feature = "stm32f328",
    feature = "stm32f358",
    feature = "stm32f398"
))]
#[cfg(any(
    feature = "stm32f302",
    feature = "stm32f303",
    feature = "stm32f373",
    feature = "stm32f378",
    feature = "stm32f334",
    feature = "stm32f328",
    feature = "stm32f358",
    feature = "stm32f398"
))]
use crate::pac::TIM3;
global_interrupt!(TIM3);
#[cfg(any(feature = "stm32f303", feature = "stm32f358", feature = "stm32f398"))]
use crate::pac::TIM8;
#[cfg(any(feature = "stm32f303", feature = "stm32f358", feature = "stm32f398"))]
global_interrupt!(TIM8);
