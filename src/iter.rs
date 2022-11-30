pub trait NightlyQuirksFunctionMut {
    type InputType;
    type OutputType;
    fn call_mut(&mut self, value: Self::InputType) -> Self::OutputType;
}

pub struct NightlyQuirksMap<I: Iterator<Item = F::InputType>, F: NightlyQuirksFunctionMut> {
    function: F,
    iterator: I,
}

impl<I: Iterator<Item = F::InputType>, F: NightlyQuirksFunctionMut> Iterator
    for NightlyQuirksMap<I, F>
{
    type Item = F::OutputType;
    fn next(&mut self) -> Option<F::OutputType> {
        self.iterator
            .next()
            .map(|item| self.function.call_mut(item))
    }
}

pub trait NightlyQuirksIterHelper<I: Iterator> {
    fn map_with_fn<F: NightlyQuirksFunctionMut<InputType = I::Item>>(
        self,
        function: F,
    ) -> NightlyQuirksMap<I, F>;
}

impl<I: Iterator> NightlyQuirksIterHelper<I> for I {
    fn map_with_fn<F: NightlyQuirksFunctionMut<InputType = I::Item>>(
        self,
        function: F,
    ) -> NightlyQuirksMap<I, F> {
        NightlyQuirksMap {
            function,
            iterator: self,
        }
    }
}
