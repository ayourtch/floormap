pub struct MenuItem {
    title: String,
    link: String,
    sub: Vec<MenuItem>,
}

impl MenuItem {
    fn new(title: &str, link: &str) -> MenuItem {
        MenuItem {
            title: title.to_owned(),
            link: link.to_owned(),
            sub: vec![],
        }
    }
    fn item(self: &mut MenuItem, title: &str, link: &str) {
        let child = MenuItem::new(title, link);
        self.sub.push(child)
    }
}

pub fn get_page_menu(username: &str, role: &str) -> Vec<MenuItem> {
    let mut items: Vec<MenuItem> = vec![];
    let mut mi = MenuItem::new("x", "x");

    mi = MenuItem::new("Home", "/Default.aspx");
    items.push(mi);

    items
}

pub fn insert_menu(mut data: mustache::VecBuilder, menu: &Vec<MenuItem>) -> mustache::VecBuilder {
    for item in menu {
        data = data.push_map(|builder| {
            builder
                .insert_str("title", item.title.clone())
                .insert_str("link", item.link.clone())
                .insert_vec("sub", |builder| insert_menu(builder, &item.sub))
        })
    }
    data
}
