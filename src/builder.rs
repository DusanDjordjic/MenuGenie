use crate::menu::Menu;
use crate::menu_item::MenuItem;
use crate::MenuAction;
use crate::MenuGenie;

#[derive(Default)]
pub struct MenuBuilder<'a> {
    menus: Vec<Menu<'a>>,
    start_menu_id: Option<usize>,
}

impl<'a> MenuBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_menu(mut self, id: usize) -> Self {
        assert!(
            !self.menus.iter().any(|menu| menu.id == id),
            "Menu with id {id} already added"
        );

        self.menus.push(Menu::new(id));

        if self.start_menu_id.is_none() {
            self.start_menu_id = Some(self.menus.len())
        }

        self
    }

    pub fn with_menu_item(mut self, prompt: &'a str, action: MenuAction) -> Self {
        let last_added_menu = self
            .menus
            .last_mut()
            .expect("Menu must be added first before adding menu items");
        last_added_menu.menu_items.push(MenuItem {
            prompt,
            action,
            key: last_added_menu.menu_items.len() + 1,
        });
        self
    }

    pub fn with_back_button(mut self) -> Self {
        let last_added_menu = self
            .menus
            .last_mut()
            .expect("Menu must be added first before adding menu items");
        last_added_menu.menu_items.push(MenuItem {
            prompt: "Back",
            action: MenuAction::Back,
            key: 0,
        });
        self
    }

    pub fn with_quit_button(mut self) -> Self {
        let last_added_menu = self
            .menus
            .last_mut()
            .expect("Menu must be added first before adding menu items");
        last_added_menu.menu_items.push(MenuItem {
            prompt: "Quit",
            action: MenuAction::Quit,
            key: 0,
        });
        self
    }

    pub fn with_starting_menu(mut self, starting_menu_id: usize) -> Self {
        self.start_menu_id = Some(starting_menu_id);
        self
    }

    pub fn build(self) -> MenuGenie<'a> {
        assert_ne!(self.menus.len(), 0, "No menus added");
        MenuGenie {
            menus: self.menus,
            // UNWRAP its safe to unwrap here bacause if no menu is added we have assert
            // and if there are menus added starting_menu_id will be Some
            start_menu_id: self.start_menu_id.unwrap(),
            callstack: vec![self.start_menu_id.unwrap()],
        }
    }
}
