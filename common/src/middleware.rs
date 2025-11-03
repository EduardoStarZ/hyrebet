use ntex::http::{self, HttpMessage};
use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::web::{Error, ErrorRenderer, HttpResponse, WebRequest, WebResponse};
use crate::session::{self, LoginToken};

pub struct CheckLogin;

impl<S> Middleware<S> for CheckLogin {
    type Service = CheckLoginMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        CheckLoginMiddleware { service }
    }
}

pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, Err> Service<WebRequest<Err>> for CheckLoginMiddleware<S>
where
    S: Service<WebRequest<Err>, Response = WebResponse, Error = Error>,
    Err: ErrorRenderer,
{
    type Response = WebResponse;
    type Error = Error;

    ntex::forward_ready!(service);
    ntex::forward_shutdown!(service);

    async fn call(
        &self,
        req: WebRequest<Err>,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> {
        // We only need to hook into the `start` for this middleware.

        let cookie = req.cookie("Auth"); // Change this to see the change in outcome in the browser

        if cookie.is_some() && session::check_token_val(&LoginToken(Some(cookie.unwrap().value().to_string()))) {
            ctx.call(&self.service, req).await
        } else {
            // Don't forward to /login if we are already on /login
            if req.path() == "/login" {
                ctx.call(&self.service, req).await
            } else {
                Ok(req.into_response(
                    HttpResponse::Found()
                        .header(http::header::LOCATION, "/login")
                        .finish()
                        .into_body(),
                ))
            }
        }
    }
}
