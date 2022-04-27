use crate::item::*;
use araiseal_types::*;
use araiseal_ui::*;
use std::convert::TryFrom;

use iced::{scrollable, Color, Column, Container, Element, Length, Row, Scrollable};

#[allow(dead_code)]
#[derive(Derivative)]
#[derivative(Default())]
pub struct ItemUI {
    #[derivative(Default(value = "Vec::with_capacity(MAX_ITEMS)"))]
    pub data: Vec<(ItemData, bool)>,
    menu: ItemUiMenu,
    generic: ItemUiGeneric,  //Generic Item Data.
    cbufs: ItemUiCombatBufs, //combat buffers.
    sbufs: ItemUiSkillBufs,  //skill buffers.
    data_ui: ItemUiData,     //Item Generic Data Types.
    currentid: usize,
    scroll: scrollable::State,
}

impl UiRenderer for ItemUI {
    type Message = Message;
    fn update(&mut self, msg: Message) -> Option<Box<dyn UiRenderer<Message = Message>>> {
        match msg {
            Message::SaveAllButtonPress => {
                self.save_all();
                return None;
            }
            Message::SaveButtonPress => {
                self.data[self.currentid]
                    .0
                    .save_file(self.currentid)
                    .unwrap();
                return None;
            }
            Message::RevertButtonPress => {
                let item = ItemData::load_file(self.currentid).unwrap();
                self.data[self.currentid].0 = item.0;
                self.data[self.currentid].1 = false;
                self.set_object_to_layout(self.currentid);
                return None;
            }
            Message::ListSelect(data) => {
                self.currentid = data.id;
                self.menu.list_selected = Some(data);
                self.set_object_to_layout(self.currentid);
                return None;
            }
            Message::ReqTypeSelect(data) => {
                self.data[self.currentid].0.reqstattype = data;
                self.data[self.currentid].0.reqstat = 0;
                self.generic.reqtype_selected = Some(data);
                self.generic.req_list.clear();

                match data {
                    StatType::None => {
                        self.generic.req_list.push(ListData::new(0, "None".into()));
                        self.generic.req_selected = Some(ListData::new(0, "None".into()));
                    }
                    StatType::Combat => {
                        for i in 0..CombatStat::Count as usize {
                            self.generic.req_list.push(ListData::new(
                                i,
                                CombatStat::try_from(i as u8)
                                    .unwrap_or(CombatStat::Count)
                                    .to_string(),
                            ));
                        }

                        self.generic.req_selected = Some(ListData::new(
                            CombatStat::Slash as usize,
                            CombatStat::Slash.to_string(),
                        ));
                    }
                    StatType::Skill => {
                        for i in 0..SkillStat::Count as usize {
                            self.generic.req_list.push(ListData::new(
                                i,
                                SkillStat::try_from(i as u8)
                                    .unwrap_or(SkillStat::Count)
                                    .to_string(),
                            ));
                        }

                        self.generic.req_selected = Some(ListData::new(
                            SkillStat::Gathering as usize,
                            SkillStat::Gathering.to_string(),
                        ));
                    }
                }
                return None;
            }
            Message::ReqSelect(data) => {
                self.data[self.currentid].0.reqstat = data.id as u8;
                self.generic.req_selected = Some(data);
            }
            Message::CombatInput((i, data)) => {
                self.cbufs.input[i].value = data.get_data();
                self.data[self.currentid].0.combatstats[i] = self.cbufs.input[i].value;
            }
            Message::SkillInput((i, data)) => {
                self.sbufs.input[i].value = data.get_data();
                self.data[self.currentid].0.skillstats[i] = self.sbufs.input[i].value;
            }
            Message::DataInput((i, data)) => {
                self.data_ui.input[i].value = data.get_data();
                self.data[self.currentid].0.data[i] = self.data_ui.input[i].value;
            }
            Message::TypeSelect(item_type) => {
                self.generic.type_selected = Some(item_type);
                self.data[self.currentid].0.itemtype = item_type;
            }
            Message::NameInput(value) => {
                if value.len() < 64 {
                    self.generic.txt_value = value;
                    self.data[self.currentid].0.name = self.generic.txt_value.clone();
                    self.menu.list[self.currentid].name = self.generic.txt_value.clone();
                    self.menu.list_selected = Some(self.menu.list[self.currentid].clone());
                } else {
                    return None;
                }
            }
            Message::GenericInput((id, data)) => {
                let value = data.get_data();

                match id {
                    0 => {
                        self.generic.skill_input.value = value;
                        self.data[self.currentid].0.skill = value;
                    }
                    1 => {
                        self.generic.level_input.value = value;
                        self.data[self.currentid].0.levelreq = value;
                    }
                    2 => {
                        self.generic.sound_input.value = value;
                        self.data[self.currentid].0.soundid = value;
                    }
                    3 => {
                        self.generic.stack_limit_input.value = value;
                        self.data[self.currentid].0.stacklimit = value;
                    }
                    4 => {
                        self.generic.sprite_input.value = value;
                        self.data[self.currentid].0.sprite = value;
                    }
                    5 => {
                        self.generic.type2.value = value;
                        self.data[self.currentid].0.itemtype2 = value as u8;
                    }
                    6 => {
                        self.generic.req_input.value = value;
                        self.data[self.currentid].0.reqstatlvl = value;
                    }
                    _ => return None,
                }
            }
            Message::AnimationInput((_, data)) => {
                self.generic.animation_input.value = data.get_data();

                if self.generic.animation_input.value < 0 {
                    self.data[self.currentid].0.animation = None;
                } else {
                    self.data[self.currentid].0.animation =
                        Some(self.generic.animation_input.value as u32);
                }
            }
            Message::BasePriceInput((_, data)) => {
                self.generic.base_price_input.value = data.get_data();
                self.data[self.currentid].0.baseprice = self.generic.base_price_input.value;
            }
            Message::Repairable(value) => {
                self.generic.repairable = value;
                self.data[self.currentid].0.repairable = value;
            }
            Message::Stackable(value) => {
                self.generic.stackable = value;
                self.data[self.currentid].0.stackable = value;
            }
            Message::Breakable(value) => {
                self.generic.breakable = value;
                self.data[self.currentid].0.breakable = value;
            }
            Message::ChooseColor => {
                self.generic.color_state.show(true);
                return None;
            }
            Message::SubmitColor(color) => {
                self.generic.color = color;
                self.data[self.currentid].0.rgba.r = (color.r * 255.0) as i16;
                self.data[self.currentid].0.rgba.g = (color.g * 255.0) as i16;
                self.data[self.currentid].0.rgba.b = (color.b * 255.0) as i16;
                self.data[self.currentid].0.rgba.a = (color.a * 255.0) as i16;
                self.generic.color_state.show(false);
            }
            Message::CancelColor => {
                self.generic.color_state.show(false);
                return None;
            }
        }

        self.data[self.currentid].1 = true;
        None
    }

    fn view(&mut self) -> Element<Message> {
        self.layout()
    }

    fn title(&self) -> &str {
        "Item Editor"
    }
}

impl ItemUI {
    pub fn new() -> Self {
        let mut ui = ItemUI {
            data: ItemData::load_files().unwrap(),
            ..Default::default()
        };

        for (i, v) in ui.data.iter().enumerate() {
            ui.menu.list.push(ListData::new(i, v.0.name.clone()));
        }

        ui.menu.list_selected = Some(ui.menu.list[0].clone());

        for i in 0..=StatType::Skill as usize {
            ui.generic
                .reqtype_list
                .push(StatType::try_from(i as u8).unwrap_or(StatType::None))
        }

        ui.generic.reqtype_selected = Some(StatType::None);

        ui.generic.req_list.push(ListData::new(0, "None".into()));
        ui.generic.req_selected = Some(ListData::new(0, "None".into()));

        for i in 0..ItemTypes::Count as usize {
            ui.generic
                .type_list
                .push(ItemTypes::try_from(i as u8).unwrap_or(ItemTypes::None));
        }

        ui.set_object_to_layout(0);
        ui
    }

    pub fn save_all(&mut self) {
        for (i, v) in self.data.iter().enumerate() {
            if !v.1 {
                continue;
            }

            if let Err(e) = v.0.save_file(i) {
                println!("Could not save Item {}, err {}", i, e);
            }
        }
    }

    fn set_object_to_layout(&mut self, index: usize) {
        for (id, control) in self.cbufs.input.iter_mut().enumerate() {
            control.value = self.data[index].0.combatstats[id];
        }

        for (id, control) in self.sbufs.input.iter_mut().enumerate() {
            control.value = self.data[index].0.skillstats[id];
        }

        for (id, control) in self.data_ui.input.iter_mut().enumerate() {
            control.value = self.data[index].0.data[id];
        }

        self.generic.type_selected = Some(self.data[index].0.itemtype);
        self.generic.txt_value = self.data[index].0.name.clone();
        self.generic.skill_input.value = self.data[index].0.skill;
        self.generic.level_input.value = self.data[index].0.levelreq;
        self.generic.sound_input.value = self.data[index].0.soundid;
        self.generic.stack_limit_input.value = self.data[index].0.stacklimit;
        self.generic.sprite_input.value = self.data[index].0.sprite;
        self.generic.type2.value = self.data[index].0.itemtype2 as u16;
        self.generic.req_input.value = self.data[index].0.reqstatlvl;
        self.generic.base_price_input.value = self.data[index].0.baseprice;
        self.generic.repairable = self.data[index].0.repairable;
        self.generic.stackable = self.data[index].0.stackable;
        self.generic.breakable = self.data[index].0.breakable;

        self.generic.color = Color::new(
            f32::from(self.data[index].0.rgba.r) / 255.0,
            f32::from(self.data[index].0.rgba.g) / 255.0,
            f32::from(self.data[index].0.rgba.b) / 255.0,
            f32::from(self.data[index].0.rgba.a) / 255.0,
        );

        match self.data[index].0.animation {
            None => self.generic.animation_input.value = -1,
            Some(v) => self.generic.animation_input.value = v as i64,
        }

        self.generic.reqtype_selected = Some(self.data[index].0.reqstattype);
        self.generic.req_list.clear();

        match self.generic.reqtype_selected {
            Some(StatType::None) => {
                self.generic.req_list.push(ListData::new(0, "None".into()));
                self.generic.req_selected = Some(ListData::new(0, "None".into()));
            }
            Some(StatType::Combat) => {
                for i in 0..CombatStat::Count as usize {
                    self.generic.req_list.push(ListData::new(
                        i,
                        CombatStat::try_from(i as u8)
                            .unwrap_or(CombatStat::Count)
                            .to_string(),
                    ));
                }

                self.generic.req_selected = Some(ListData::new(
                    self.data[index].0.reqstat as usize,
                    CombatStat::try_from(self.data[index].0.reqstat as u8)
                        .unwrap_or(CombatStat::Count)
                        .to_string(),
                ));
            }
            Some(StatType::Skill) => {
                for i in 0..SkillStat::Count as usize {
                    self.generic.req_list.push(ListData::new(
                        i,
                        SkillStat::try_from(i as u8)
                            .unwrap_or(SkillStat::Count)
                            .to_string(),
                    ));
                }

                self.generic.req_selected = Some(ListData::new(
                    self.data[index].0.reqstat as usize,
                    SkillStat::try_from(self.data[index].0.reqstat as u8)
                        .unwrap_or(SkillStat::Count)
                        .to_string(),
                ));
            }
            None => {
                self.generic.req_list.push(ListData::new(0, "None".into()));
                self.generic.req_selected = Some(ListData::new(0, "None".into()));
            }
        }
    }

    pub fn layout(&mut self) -> Element<Message> {
        let item_type = self.generic.type_selected.unwrap_or(ItemTypes::None);

        let column =
            Column::new().spacing(20).push(self.menu.layout()).push(
                Scrollable::new(&mut self.scroll).push(
                    Row::new()
                        .push(
                            Column::new()
                                .spacing(5)
                                .push(
                                    Container::new(self.generic.layout(
                                        self.generic.type_selected.unwrap_or(ItemTypes::None),
                                    ))
                                    .style(araiseal_styles::UiContainer)
                                    .padding(5)
                                    .width(Length::Fill)
                                    .center_x()
                                    .center_y(),
                                )
                                .push(
                                    Container::new(self.data_ui.layout(item_type))
                                        .style(araiseal_styles::UiContainer)
                                        .padding(5)
                                        .width(Length::Fill)
                                        .center_x()
                                        .center_y(),
                                )
                                .push(
                                    Container::new(self.cbufs.layout())
                                        .style(araiseal_styles::UiContainer)
                                        .padding(5)
                                        .width(Length::Fill)
                                        .center_x()
                                        .center_y(),
                                )
                                .push(
                                    Container::new(self.sbufs.layout())
                                        .style(araiseal_styles::UiContainer)
                                        .padding(5)
                                        .width(Length::Fill)
                                        .center_x()
                                        .center_y(),
                                )
                                .width(Length::FillPortion(30)),
                        )
                        .push(Column::new().width(Length::FillPortion(1))),
                ),
            );

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}
