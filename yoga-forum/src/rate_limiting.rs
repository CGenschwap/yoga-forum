use crate::error::YogaError;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use anyhow::anyhow;
use futures_util::future::LocalBoxFuture;
use governor::{clock::DefaultClock, state::keyed::DefaultKeyedStateStore, Quota, RateLimiter};
use std::future::{ready, Ready};
use std::num::NonZeroU32;
use std::sync::Arc;

type IpRateLimiter = RateLimiter<String, DefaultKeyedStateStore<String>, DefaultClock>;

pub struct RateLimiting {
    limiter: Arc<IpRateLimiter>,
}

impl Default for RateLimiting {
    fn default() -> Self {
        // TODO: These paremeters are hard-coded but can easily be extracted.
        let limiter = RateLimiter::keyed(
            Quota::per_second(NonZeroU32::new(5u32).unwrap())
                .allow_burst(NonZeroU32::new(10u32).unwrap()),
        );
        let limiter = Arc::new(limiter);
        Self { limiter }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiting
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitingMiddleware {
            service,
            limiter: self.limiter.clone(),
        }))
    }
}

pub struct RateLimitingMiddleware<S> {
    service: S,
    limiter: Arc<IpRateLimiter>,
}

impl<S, B> Service<ServiceRequest> for RateLimitingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // TODO: Verify this is the correct IP key to be rate limiting on
        {
            let conn = req.connection_info();
            let ip = conn.realip_remote_addr();
            if let Some(ip) = ip {
                // Disable for DEBUG builds
                if cfg!(not(debug_assertions)) {
                    match self.limiter.check_key(&ip.to_string()) {
                        Ok(_) => {}
                        Err(err) => {
                            tracing::info!("User {ip} rate limited for {err:?}");
                            return Box::pin(ready(Err(YogaError::RateLimited.into())));
                        }
                    }
                }
            } else {
                return Box::pin(ready(Err(YogaError::Unknown(anyhow!(
                    "Could not extract IP Address"
                ))
                .into())));
            }
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
