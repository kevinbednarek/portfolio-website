use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {

        <Stylesheet id="leptos" href="/style/output.css"/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <Home/> }/>
                <Route path="/games" view=  move || view! { <Home/> }/>  //TODO: Have this go to a games page
                <Route path="/resume" view=  move || view! { <Home/> }/>  //TODO: Have this go to a resume view where you can download
                //TODO: Add links for LinkedIn and GitHub
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <Body class="min-h-screen bg-gradient-to-b from-black to-slate-700 font-roboto"/>
        <div class="h-screen flex justify-center items-center">
            <NameBlock/>
        </div>
    }
}

#[component]
fn NameBlock() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto text-center">
            <h1 class="text-9xl text-teal-300">"Hi, I'm Kevin"</h1>
            <h2 class="text-7xl text-teal-300">"Professional Software Engineer"</h2>
        </div>
    }
}