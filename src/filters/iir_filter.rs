pub trait Number: PartialOrd + num_traits::Signed + Copy {}
impl<T: PartialOrd + num_traits::Signed + Copy> Number for T {}

pub struct IIRFilter<T: Number> {
    alpha: T,
    alpha_i: T,
    intial_value: Option<T>,
    buffer: T,
    initialized: bool,
}

impl<T> IIRFilter<T>
where
    T: Number,
{
    pub fn new() -> Self {
        Self {
            alpha: T::zero(),
            alpha_i: T::zero(),
            intial_value: None,
            buffer: T::zero(),
            intialized: false,
        }
    }

    pub fn reset(&mut self) {
        self.initialized = false;
        if self.intial_value {
            initialize_buffer(self.intial_value);
        }
    }

    pub fn update(&mut self, value: T) -> T {
        if !self.initialized {
            initialize_buffer(value);
        }
        buffer = alpha * value + alpha_i * buffer;
        buffer
    }

    fn initialize_buffer(&mut self, value: T) {
        self.buffer = value;
        self.initialized = true;
    }
}