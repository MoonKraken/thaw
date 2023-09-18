use crate::components::SiteHeader;
use leptos::*;
use leptos_router::{use_location, use_navigate, Outlet};
use melt_ui::*;
use regex::Regex;

#[component]
pub fn ComponentsPage() -> impl IntoView {
    let loaction = use_location();
    let navigate = use_navigate();
    let selected = create_rw_signal(String::from(""));
    create_effect(move |_| {
        let pathname = loaction.pathname.get();

        let re = Regex::new(r"^/melt-ui/components/(.+)$").unwrap();
        let Some(caps) = re.captures(&pathname) else {
            return;
        };
        let Some(path) = caps.get(1) else {
            return;
        };
        let path = path.as_str().to_string();
        selected.set(path);
    });

    create_effect(move |value| {
        let selected = selected.get();
        if value.is_some() {
            navigate(&format!("/components/{selected}"), Default::default());
        }
        selected
    });
    view! {
        <Layout position=LayoutPosition::ABSOLUTE>
            <SiteHeader />
            <Layout has_sider=true position=LayoutPosition::ABSOLUTE style="top: 54px;">
                <LayoutSider>
                    <Menu selected>
                        <MenuItem key="menu" label="menu" />
                        <MenuItem key="slider" label="slider" />
                        <MenuItem key="tabbar" label="tabbar" />
                        <MenuItem key="input" label="input" />
                        <MenuItem key="image" label="image" />
                        <MenuItem key="modal" label="modal" />
                        <MenuItem key="nav-bar" label="nav-bar" />
                        <MenuItem key="button" label="button" />
                        <MenuItem key="checkbox" label="checkbox" />
                        <MenuItem key="toast" label="toast" />
                        <MenuItem key="tabs" label="tabs" />
                        <MenuItem key="select" label="select" />
                    </Menu>
                </LayoutSider>
                <Layout style="padding: 8px 12px 28px; overflow-y: scroll;">
                    <Outlet />
                </Layout>
            </Layout>
        </Layout>
    }
}