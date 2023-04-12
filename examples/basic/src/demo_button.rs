use leptos::*;
use melt_ui::*;

#[component]
pub fn DemoButton(cx: Scope) -> impl IntoView {
    view! {cx,
        <Space>
            <Button type_=ButtonType::PRIMARY>
                "PRIMARY"
            </Button>
            <Button type_=ButtonType::SOLID>
                "SOLID"
            </Button>
            <Button type_=ButtonType::TEXT>
                "TEXT"
            </Button>
            <Button color=ButtonColor::PRIMARY>
                "PRIMARY Color"
            </Button>
            <Button color=ButtonColor::SUCCESS>
                "SUCCESS Color"
            </Button>
            <Button color=ButtonColor::WARNING>
                "WARNING Color"
            </Button>
            <Button color=ButtonColor::ERROR>
                "ERROR Color"
            </Button>
        </Space>
    }
}