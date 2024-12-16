use nostr_minions::{
    key_manager::NostrIdProvider,
    relay_pool::{RelayProvider, UserRelay},
};
use upload_things::{UtPreSignedUrl, UtUpload};
use web_sys::{wasm_bindgen::JsCast, File, FileReader, FormData, HtmlInputElement, Request};
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let image_url = use_state(|| None);
    let state_handle = image_url.clone();
    let onclick = Callback::from(move |_| {
        let document =
            nostr_minions::browser_api::HtmlDocument::new().expect("Failed to get document");
        let input_element = document
            .find_element_by_id::<HtmlInputElement>("file-input")
            .expect("Failed to find element");
        let files = input_element.files().expect("Failed to get files");
        let file_clone: File = files.get(0).expect("No file");

        let file_req = upload_things::UtRequest::from(&file_clone);

        let form_data = FormData::new().expect("Failed to create form data");
        form_data.append_with_blob("file", &file_clone).unwrap();

        let reader = FileReader::new().expect("Failed to create reader");
        let state_handle = state_handle.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let reader_handle = reader.clone();
            let form_data = form_data.clone();
            let request = Request::new_with_str_and_init(
                "http://0.0.0.0:4200/api/uploadthing/presigned-url",
                &file_req.into(),
            )
            .expect("Failed to create request");
            let url: UtPreSignedUrl = nostr_minions::browser_api::BrowserFetch::request(&request)
                .await
                .expect("Failed to fetch");
            let closure = web_sys::wasm_bindgen::closure::Closure::wrap(Box::new(
                move |_: web_sys::ProgressEvent| {
                    if let Ok(_) = reader_handle.result() {
                        let url = url.clone();
                        let form_data = form_data.clone();
                        let state_handle = state_handle.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            let url_req =
                                url.try_into_request(form_data).expect("Failed to convert");
                            let response_json: UtUpload =
                                nostr_minions::browser_api::BrowserFetch::request(&url_req)
                                    .await
                                    .expect("Failed to fetch");
                            state_handle.set(Some(response_json.url));
                        });
                    }
                },
            )
                as Box<dyn FnMut(web_sys::ProgressEvent)>);

            reader.set_onloadend(Some(closure.as_ref().unchecked_ref()));
            reader.read_as_array_buffer(&file_clone).unwrap();
            closure.forget(); // Forget the closure to keep it alive
        });
    });
    use_effect_with(image_url.clone(), move |image_url| {
        if image_url.is_some() {
            let document =
                nostr_minions::browser_api::HtmlDocument::new().expect("Failed to get document");
            gloo::console::log!("Image URL: {:?}", format!("{:?}", image_url));
            let image_element = document
                .find_element_by_id::<web_sys::HtmlImageElement>("image")
                .expect("Failed to find element");
            image_element.set_src(image_url.as_ref().unwrap().as_str());
        }
        || {}
    });

    html! {
        <AppContextProviders>
            <div class="flex flex-col h-full flex-1 gap-4 p-4 text-center items-center justify-center">
                <h1 class="text-2xl font-bold">{"O2 Showcase"}</h1>
                <img id={"image"} class="max-w-md min-w-12 min-h-12" />
                <input type="file" id="file-input" />
                <button
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                   {onclick}
                >
                    {"Upload File"}
                </button>
            </div>
        </AppContextProviders>
    }
}

#[function_component(AppContextProviders)]
fn app_context_providers(props: &html::ChildrenProps) -> Html {
    let relays = vec![
        UserRelay {
            url: "wss://relay.illuminodes.com".to_string(),
            read: true,
            write: true,
        },
        UserRelay {
            url: "wss://relay.arrakis.lat".to_string(),
            read: true,
            write: true,
        },
    ];
    html! {
        <RelayProvider {relays} >
            <NostrIdProvider>
                {props.children.clone()}
            </NostrIdProvider>
        </RelayProvider>
    }
}
