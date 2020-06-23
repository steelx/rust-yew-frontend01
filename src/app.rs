use yew::prelude::*;

pub struct App {
    counter: i64,
    link: ComponentLink<Self>,
}

pub enum Msg {
    AddOne,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    //create is the equivalent of a constructor for the component.
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            counter: 0,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.counter += 1,
        }

        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <div>
                <p>{"Counter: "} {self.counter}</p>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{"Add One"}</button>
            </div>
        }
    }
}
