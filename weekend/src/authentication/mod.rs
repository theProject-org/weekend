use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, ops::Deref};
mod builder;
mod challenge;
mod forbid;
mod sign_in;
mod sign_out;

// use builder::{AuthenticationBuilder, SignInBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    redirect_url: String,
}

/*
    TODO
    * Make error generic so we can compose errror together
    * Provide a simplified api to implement `Authentication`
*/

#[async_trait]
pub trait Authentication<State, Payload>
where
    State: Deref<Target = self::State>,
{
    type Error;

    /// The name of the authentication handler
    /// This is used to identify the handler in logs and other contexts
    const NAME: &'static str;

    /// Authenticate the current request
    ///
    /// This method is called to authenticate the current request
    async fn authenticate(&self) -> Result<(), Self::Error>;

    /// Forbid the current request
    ///
    /// This method is called to forbid the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `S`
    async fn forbid(&self, state: &State) -> Result<(), Self::Error>;

    /// Challenge the current request
    ///
    /// This method is called to challenge the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `Self::State`
    async fn challenge(&self, state: &State) -> Result<(), Self::Error>;

    /// Sign in the current request
    ///
    /// This method is called to sign-in
    ///
    /// # Arguments
    /// `state` - The current state of the request `Self::State`
    /// `payload` - The payload to be used for signing in `Payload`
    async fn sign_in(&self, state: &State, payload: &Payload) -> Result<(), Self::Error>;

    /// Sign out in the current request
    ///
    /// This method is called to sign-in
    ///
    /// # Arguments
    /// `state` - The current state of the request `Self::State`
    /// `payload` - The payload to be used for signing in `Payload`
    async fn sign_out(&self, state: &State, payload: &Payload) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod test {}
