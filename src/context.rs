use yew::context::ContextHandle;
use yew::html::Scope;
use yew::{Callback, Component, Context};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OAuth2Context {
    NotInitialized,
    NotAuthenticated {
        reason: Reason,
    },
    Authenticated {
        access_token: String,
        refresh_token: Option<String>,
        expires: Option<u64>,
    },
    Failed(String),
}

impl OAuth2Context {
    /// Get the access token, if the context is [`OAuth2Context::Authenticated`]
    pub fn access_token(&self) -> Option<String> {
        match self {
            Self::Authenticated { access_token, .. } => Some(access_token.clone()),
            _ => None,
        }
    }
}

/// The reason why the context is un-authenticated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reason {
    NewSession,
    Expired,
    Logout,
}

/// Helper to get an unzipped version of the context.
pub trait UnzippedWith {
    fn unzipped_with(
        &self,
        callback: Callback<OAuth2Context>,
    ) -> (Option<OAuth2Context>, Option<ContextHandle<OAuth2Context>>);
}

/// Helper to get an unzipped version of the context.
pub trait Unzipped {
    type Message;

    fn unzipped<F>(&self, f: F) -> (Option<OAuth2Context>, Option<ContextHandle<OAuth2Context>>)
    where
        F: Fn(OAuth2Context) -> Self::Message + 'static;
}

impl<C> UnzippedWith for Context<C>
where
    C: Component,
{
    fn unzipped_with(
        &self,
        callback: Callback<OAuth2Context>,
    ) -> (Option<OAuth2Context>, Option<ContextHandle<OAuth2Context>>) {
        self.link().unzipped_with(callback)
    }
}

impl<C> UnzippedWith for Scope<C>
where
    C: Component,
{
    fn unzipped_with(
        &self,
        callback: Callback<OAuth2Context>,
    ) -> (Option<OAuth2Context>, Option<ContextHandle<OAuth2Context>>) {
        match self.context(callback) {
            Some((auth, handle)) => (Some(auth), Some(handle)),
            None => (None, None),
        }
    }
}

impl<C> Unzipped for Context<C>
where
    C: Component,
{
    type Message = C::Message;

    fn unzipped<F>(&self, f: F) -> (Option<OAuth2Context>, Option<ContextHandle<OAuth2Context>>)
    where
        F: Fn(OAuth2Context) -> Self::Message + 'static,
    {
        self.unzipped(f)
    }
}

impl<C> Unzipped for Scope<C>
where
    C: Component,
{
    type Message = C::Message;

    fn unzipped<F>(&self, f: F) -> (Option<OAuth2Context>, Option<ContextHandle<OAuth2Context>>)
    where
        F: Fn(OAuth2Context) -> Self::Message + 'static,
    {
        self.unzipped_with(self.callback(f))
    }
}
