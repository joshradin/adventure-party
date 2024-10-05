use askama_axum::Response;
use axum::body::{Body, Bytes, HttpBody};
use axum::response::IntoResponse;
use futures::Stream;
use http_body::Frame;
use pages::PageComponent;
use pin_project::pin_project;
use std::convert::Infallible;
use std::pin::Pin;
use std::task::{Context, Poll};
use tracing::trace;
use yew::{function_component, html, BaseComponent, Html, Properties, ServerRenderer};

/// A yew compatability response
#[derive(Debug)]
pub struct Yew<T: PageComponent>(pub T::Properties)
where
    T::Properties: Send + Clone;

impl<T: PageComponent> Yew<T>
where
    T::Properties: Send + Clone + Default,
{
    /// Creates a new Yew Component with default propreties
    pub fn new() -> Self {
        Self(T::Properties::default())
    }
}

#[function_component]
fn WrappedYew<T: PageComponent>(props: &T::Properties) -> Html
where
    T::Properties: Send + Clone,
{
    let page = T::page();
    html! {
        <html lang="en">
            <head>

            </head>

            <body>
                <T ..props.clone()/>
                <script type="module">
                    <>
                {r#"
                    import init, { hydrate, Page } from "/ssr_hydrate/ssr_hydrate.js"

                    if (!hydrate || !Page) {
                        throw new Error("hydrate or page is not defined")
                    }
                    await init({})
                    "#}
                    {
                    format!("hydrate({})", page)
                    }
                    </>
                </script>
            </body>
        </html>
    }
}

impl<T: PageComponent> IntoResponse for Yew<T>
where
    T::Properties: Send + Clone,
{
    fn into_response(self) -> Response {
        let renderer = ServerRenderer::<WrappedYew<T>>::with_props(|| self.0);

        Response::new(Body::new(YewBody::new(renderer)))
    }
}

#[pin_project]
struct YewBody {
    #[pin]
    stream: Pin<Box<dyn Stream<Item = String> + Send>>,
}

impl YewBody {
    fn new<T: BaseComponent>(renderer: ServerRenderer<T>) -> Self {
        Self {
            stream: Box::pin(renderer.render_stream()),
        }
    }
}

impl HttpBody for YewBody {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let mut me = self.project();
        match me.stream.as_mut().poll_next(cx) {
            Poll::Ready(ready) => {
                let frame = ready.map(|string| {
                    trace!("yew string ready: {string:?}");
                    Ok(Frame::data(Bytes::from(string)))
                });
                Poll::Ready(frame)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
