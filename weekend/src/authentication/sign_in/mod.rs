use tower_layer::Layer;

pub struct SignInLayer<H> {
    inner_handler: H,
}

impl<H> SignInLayer<H> {
    pub fn new(inner_handler: H) -> Self {
        Self { inner_handler }
    }
}

impl<S, H: Clone> Layer<S> for SignInLayer<H> {
    type Service = SignInService<S, H>;

    fn layer(&self, inner: S) -> Self::Service {
        SignInService::new(inner, self.inner_handler.clone())
    }
}

pub struct SignInService<T, H> {
    inner: T,
    inner_handler: H,
}

impl<T, H> SignInService<T, H> {
    pub fn new(inner: T, inner_handler: H) -> Self {
        Self { inner, inner_handler }
    }
}
