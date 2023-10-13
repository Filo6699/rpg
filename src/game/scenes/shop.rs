use super::{stats::StatisticsScene, Scene, SharedData};
use crate::game::{
    item::{Item, ItemProperties, ItemType},
    message_queue::MessageQueue,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Paragraph,
    },
};

const SCENE_ID: i32 = 3;

enum Stage {
    ItemSelecting,
    ConfirmBuy(bool),
}

pub struct ShopScene {
    items: Vec<Item>,
    selected_id: u8,
    stage: Stage,
    message_queue: MessageQueue,
}

impl ShopScene {
    pub fn new() -> Self {
        let sword = Item {
            name: "Sword".into(),
            item_type: ItemType::Sword,
            cost: 10,
            properties: ItemProperties {
                damage: 10,
                defence: 0,
            },
        };
        let shield = Item {
            name: "Shield".into(),
            item_type: ItemType::Shield,
            cost: 10,
            properties: ItemProperties {
                damage: 0,
                defence: 26,
            },
        };
        ShopScene {
            items: vec![sword, shield],
            selected_id: 0,
            stage: Stage::ItemSelecting,
            message_queue: MessageQueue::default(),
        }
    }

    pub fn scene_id() -> i32 {
        SCENE_ID
    }

    fn buy_item(&mut self, data: &mut SharedData) {
        let item = self.items.get((self.selected_id - 1) as usize).unwrap();
        if data.player_data.get_coins() < item.cost as u128 {
            let msg = format!("Not enough coins to buy {}", &item.name);
            self.message_queue.add_message(msg);
            return;
        }
        data.player_data.remove_coins(item.cost as u128);
        let equip = data.player_data.get_mut_equipment();
        match item.item_type {
            ItemType::Sword => equip.sword = Some(item.clone()),
            ItemType::Shield => equip.shield = Some(item.clone()),
        }
        let msg = format!("Succesfully bought {}!", &item.name);
        self.message_queue.add_message(msg);
    }
}

impl Scene for ShopScene {
    fn scene_id(&self) -> i32 {
        SCENE_ID
    }

    fn set_message_queue(&mut self, queue: MessageQueue) {
        self.message_queue = queue;
    }

    fn handle_input(&mut self, key: KeyEvent, data: &mut SharedData) {
        match self.stage {
            Stage::ItemSelecting => match key.code {
                KeyCode::Up => {
                    if self.selected_id > 0 {
                        self.selected_id -= 1
                    }
                }
                KeyCode::Down => {
                    self.selected_id = std::cmp::min(self.items.len() as u8, self.selected_id + 1)
                }
                KeyCode::Enter => {
                    if self.selected_id == 0 {
                        data.current_scene = StatisticsScene::scene_id()
                    } else {
                        self.stage = Stage::ConfirmBuy(true)
                    }
                }
                _ => (),
            },
            Stage::ConfirmBuy(confirmed) => match key.code {
                KeyCode::Right => self.stage = Stage::ConfirmBuy(false),
                KeyCode::Left => self.stage = Stage::ConfirmBuy(true),
                KeyCode::Enter => {
                    if confirmed {
                        self.buy_item(data)
                    }
                    self.stage = Stage::ItemSelecting
                }
                _ => (),
            },
        }
    }

    fn render(&self, frame: &mut crate::Frame, _: &SharedData) {
        let area = Rect {
            x: 0,
            y: 0,
            width: frame.size().width - 1,
            height: 1,
        };
        let style = if self.selected_id == 0 {
            Style::default().on_cyan()
        } else {
            Style::default()
        };
        let span = Span::styled("Go back", style);
        frame.render_widget(Paragraph::new(span), area);

        let mut shop_list: Vec<Line> = vec![];
        for (index, item) in self.items.iter().enumerate() {
            let mut style = Style::default();
            if (self.selected_id) as usize == index + 1 {
                style = style.bg(Color::Cyan);
            }
            let content = Span::styled(
                format!("[{}] {} - {} c.", index, &item.name, item.cost),
                style,
            );
            shop_list.push(Line::from(content))
        }
        let area = Rect {
            x: 0,
            y: 2,
            width: frame.size().width - 1,
            height: frame.size().height - 3,
        };
        let shop_list = Paragraph::new(shop_list);
        frame.render_widget(shop_list, area);

        if let Stage::ConfirmBuy(confirmed) = self.stage {
            let item = self.items.get((self.selected_id - 1) as usize).unwrap();
            let warning = "You sure you want to buy";
            let item_info = format!("{} for {}c?", &item.name, item.cost);
            let length = std::cmp::max(warning.len(), item_info.len()) + 2;
            let paragraph = Paragraph::new(format!("{}\n{}", warning, item_info)).bold();
            let title = "Confirm buy";
            let area = Rect {
                x: frame.size().width / 2 - length as u16 / 2,
                y: frame.size().height / 2 - 2,
                width: length as u16,
                height: 4,
            };
            let highlighted = Style::default().on_cyan();
            let confirmation = Line::from(vec![
                Span::styled(
                    "Yes",
                    if confirmed {
                        highlighted
                    } else {
                        Style::default()
                    },
                ),
                Span::raw(" / "),
                Span::styled(
                    "No",
                    if !confirmed {
                        highlighted
                    } else {
                        Style::default()
                    },
                ),
            ]);
            let block = Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(
                    Title::from(confirmation)
                        .position(Position::Bottom)
                        .alignment(Alignment::Right),
                )
                .title(Title::from(title.clone()).alignment(Alignment::Right));
            frame.render_widget(paragraph.clone().block(block), area);
        }
    }

    fn update(&mut self, _: &mut SharedData) {}
}
