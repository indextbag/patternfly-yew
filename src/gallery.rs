use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub gutter: bool,
}

#[derive(Clone, PartialEq)]
pub struct Gallery {
    props: Props,
}

impl Component for Gallery {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-l-gallery");

        if self.props.gutter {
            classes.push("pf-m-gutter");
        }

        return html! {
            <div class=("pf-l-gallery",classes)>
            { for self.props.children.iter().map(|child|{
                html_nested!{
                    <div class="pf-l-gallery__item">
                        { child }
                    </div>
                }
            }) }
            </div>
        };
    }
}
