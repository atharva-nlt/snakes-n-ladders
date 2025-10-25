use std::ops::Range;

use iced::{
    Alignment::Center,
    Background, Border, Color, Element,
    Length::{self, Fill},
    Pixels, Point, Renderer, Size, Task,
    border::Radius,
    mouse,
    widget::{
        self, Container, PickList,
        canvas::{self, Path, Stroke},
        column,
        container::Style,
        row,
    },
    window::{Position as WindowPosition, Settings},
};
use rand::Rng;

#[derive(Debug, Clone)]
enum Message {
    Menu(MenuMsg),
    Game(GameMsg),
}

enum Screen {
    Menu(MenuPage),
    Game(GamePage),
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Menu(MenuPage::default())
    }
}

#[derive(Debug, Clone)]
enum MenuMsg {
    LaunchGame(Config),
    PickList(Mode),
    AddPlayer,
    UpdatePlayer(usize, String),
    RemovePlayer(usize),
}

#[derive(Debug, Clone, Copy)]
enum GameMsg {
    GoToMenu,
    RollDice,
}

struct App {
    current: Screen,
}

impl Default for App {
    fn default() -> Self {
        App {
            current: Screen::default(),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match (&mut self.current, message) {
            (_, Message::Menu(MenuMsg::LaunchGame(config))) => {
                self.current = Screen::Game(GamePage::new(config));
                iced::window::get_latest().and_then(|id| iced::window::maximize(id, true))
            }

            (_, Message::Game(GameMsg::GoToMenu)) => {
                self.current = Screen::Menu(MenuPage::default());
                iced::window::get_latest().and_then(|id| iced::window::maximize(id, false))
            }

            (Screen::Menu(page), Message::Menu(msg)) => {
                page.update(msg);
                Task::none()
            }
            (Screen::Game(page), Message::Game(msg)) => {
                page.update(msg);
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match &self.current {
            Screen::Menu(page) => page.view().map(Message::Menu),
            Screen::Game(page) => page.view().map(Message::Game),
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum Mode {
    #[default]
    Friendly,
    Bump,
    Swap,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Mode::Friendly => "Friendly",
            Mode::Bump => "Bump",
            Mode::Swap => "Swap",
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct Config {
    players: Vec<Player>,
    possible_players: Vec<Player>,
    game_type: Mode,
}

struct MenuPage {
    config: Config,
}

impl Default for MenuPage {
    fn default() -> Self {
        MenuPage {
            config: Config {
                players: vec![
                    Player {
                        name: String::from("player"),
                        position: Position { x: 0, y: 0 },
                        color: Color::from_rgb8(255, 0, 0),
                        offset: (25, 25),
                    },
                    Player {
                        name: String::from("player"),
                        position: Position { x: 0, y: 0 },
                        color: Color::from_rgb8(0, 255, 0),
                        offset: (25, 75),
                    },
                ],
                possible_players: vec![
                    Player {
                        name: String::from("player"),
                        position: Position { x: 0, y: 0 },
                        color: Color::from_rgb8(255, 255, 0),
                        offset: (75, 25),
                    },
                    Player {
                        name: String::from("player"),
                        position: Position { x: 0, y: 0 },
                        color: Color::from_rgb8(0, 0, 255),
                        offset: (75, 75),
                    },
                ],
                game_type: Mode::default(),
            },
        }
    }
}

impl MenuPage {
    fn update(&mut self, message: MenuMsg) {
        match message {
            MenuMsg::PickList(mode) => self.config.game_type = mode,
            MenuMsg::UpdatePlayer(i, s) => self.config.players[i].name = s,
            MenuMsg::AddPlayer => {
                if let Some(p) = self.config.possible_players.pop() {
                    self.config.players.push(p);
                } else {
                }
            }
            MenuMsg::RemovePlayer(i) => {
                self.config
                    .possible_players
                    .push(self.config.players.remove(i));
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, MenuMsg> {
        let pick = PickList::new(
            [Mode::Friendly, Mode::Bump, Mode::Swap],
            Some(self.config.game_type),
            MenuMsg::PickList,
        );
        let players: Vec<Element<MenuMsg>> = self
            .config
            .players
            .iter()
            .enumerate()
            .map(|(i, s)| {
                if self.config.players.len() > 2 {
                    row![
                        Element::from(widget::text_input("not allowed", &s.name).on_input(
                            move |v| {
                                let v: String = if v.chars().count() > 25 {
                                    v.chars().take(25).collect()
                                } else {
                                    v
                                };
                                MenuMsg::UpdatePlayer(i, v)
                            }
                        )),
                        Container::new("")
                            .height(Length::Fixed(20.0))
                            .width(Length::Fixed(20.0))
                            .style(|_| {
                                Style {
                                    background: Some(Background::Color(s.color.clone())),
                                    ..Default::default()
                                }
                            }),
                        Element::from(widget::button("X").on_press(MenuMsg::RemovePlayer(i)))
                    ]
                    .align_y(Center)
                    .spacing(10)
                    .into()
                } else {
                    Container::new(
                        row![
                            Element::from(widget::text_input("not allowed", &s.name).on_input(
                                move |v| {
                                    let v: String = if v.chars().count() > 25 {
                                        v.chars().take(25).collect()
                                    } else {
                                        v
                                    };
                                    MenuMsg::UpdatePlayer(i, v)
                                }
                            )),
                            Container::new("")
                                .height(Length::Fixed(20.0))
                                .width(Length::Fixed(20.0))
                                .style(|_| {
                                    Style {
                                        background: Some(Background::Color(s.color.clone())),
                                        ..Default::default()
                                    }
                                }),
                        ]
                        .align_y(Center)
                        .spacing(10),
                    )
                    .align_x(Center)
                    .align_y(Center)
                    .into()
                }
            })
            .collect();
        let players = widget::column(players).spacing(10).padding(10);
        widget::column![
            widget::text("Main Menu").size(30),
            widget::row![widget::text("Select Game Mode:").size(20), pick,]
                .spacing(10)
                .align_y(Center),
            widget::row![
                widget::text("Enter player details:").size(20),
                widget::button(widget::text("Add").align_x(Center))
                    .on_press(MenuMsg::AddPlayer)
                    .width(iced::Length::Fill),
            ]
            .spacing(10)
            .padding(10)
            .align_y(Center),
            players,
            widget::container(
                widget::button(
                    widget::text("Start Game")
                        .size(20)
                        .align_x(Center)
                        .align_y(Center)
                        .width(Fill)
                )
                .padding(10)
                .on_press(MenuMsg::LaunchGame(self.config.clone()))
                .width(iced::Length::Fill)
            )
            .padding(10),
        ]
        .spacing(10)
        .align_x(Center)
        .into()
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Snake(Position),
    Ladder(Position),
    Target,
    None,
}

#[derive(Debug, Clone)]
struct Board {
    tile: [[Tile; 10]; 10],
    players: Vec<Player>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Board {
    pub const TILE_SIZE: i32 = 100;
    fn add_snl(
        num: u8,
        tile: &mut [[Tile; 10]; 10],
        r_start: Range<usize>,
        r_end: fn(usize) -> Range<usize>,
        make_tile: impl Fn(Position) -> Tile,
    ) {
        let mut rng = rand::thread_rng();
        let max_attempts = 50;
        let mut attempts = 0;
        for _ in 0..num {
            'outer: loop {
                let pos_start = rng.gen_range(r_start.clone());
                if !matches!(tile[pos_start / 10][pos_start % 10], Tile::None) {
                    continue;
                }
                loop {
                    let pos_end = rng.gen_range(r_end(pos_start));
                    if !matches!(tile[pos_end / 10][pos_end % 10], Tile::None) {
                        attempts += 1;
                        if attempts > max_attempts {
                            continue 'outer;
                        }
                        continue;
                    }
                    attempts = 0;
                    tile[pos_start / 10][pos_start % 10] = make_tile(Position {
                        x: (pos_end % 10) as i32,
                        y: (pos_end / 10) as i32,
                    });
                    tile[pos_end / 10][pos_end % 10] = Tile::Target;
                    break;
                }
                break;
            }
        }
    }

    fn new(players: Vec<Player>) -> Self {
        let mut tile: [[Tile; 10]; 10] = [[Tile::None; 10]; 10];
        // add snakes
        Board::add_snl(
            8,
            &mut tile,
            10..99,
            |pos| 1..(pos / 10) * 10,
            |pos| Tile::Snake(pos),
        );
        // add ladders
        Board::add_snl(
            8,
            &mut tile,
            1..90,
            |pos| (pos / 10 + 1) * 10..99,
            |pos| Tile::Ladder(pos),
        );
        // dbg!(&tile);
        Board { tile, players }
    }
}

impl<GameMsg> canvas::Program<GameMsg> for Board {
    type State = ();
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &iced_runtime::core::Theme,
        _bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(
            renderer,
            Size {
                width: 1000 as f32,
                height: 1000 as f32,
            },
        );

        let mut v: Vec<Path> = vec![Path::rectangle(
            Point::ORIGIN,
            Size {
                width: 10 as f32 * Board::TILE_SIZE as f32,
                height: 10 as f32 * Board::TILE_SIZE as f32,
            },
        )];

        let mut v_snake: Vec<Path> = Vec::new();
        let mut v_ladder: Vec<Path> = Vec::new();
        let mut v_player: Vec<(Path, Color)> = Vec::new();

        for tile_index in 0..100 {
            let tile = self.tile[tile_index / 10][tile_index % 10];
            let row = 9 - tile_index / 10;
            let col = tile_index % 10;
            let col = if row & 1 == 1 { col } else { 9 - col };
            let s = format!("{}", tile_index + 1);
            let p = Point {
                x: col as f32 * Board::TILE_SIZE as f32,
                y: row as f32 * Board::TILE_SIZE as f32,
            };
            let text = canvas::Text {
                content: s,
                position: Point {
                    x: p.x + Board::TILE_SIZE as f32 / 2.0,
                    y: p.y + Board::TILE_SIZE as f32 / 2.0,
                },
                color: Color::from_rgba8(200, 0, 255, 0.7),
                size: Pixels::from(30),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                ..Default::default()
            };
            v.push(Path::rectangle(
                p,
                Size {
                    width: Board::TILE_SIZE as f32,
                    height: Board::TILE_SIZE as f32,
                },
            ));
            match tile {
                Tile::Snake(pos) => {
                    let r = 9 - pos.y;
                    let c = if r & 1 == 1 { pos.x } else { 9 - pos.x };
                    v_snake.push(Path::line(
                        Point {
                            x: p.x + Board::TILE_SIZE as f32 / 2.0,
                            y: p.y + Board::TILE_SIZE as f32 / 2.0,
                        },
                        Point {
                            x: c as f32 * Board::TILE_SIZE as f32 + Board::TILE_SIZE as f32 / 2.0,
                            y: r as f32 * Board::TILE_SIZE as f32 + Board::TILE_SIZE as f32 / 2.0,
                        },
                    ));
                }
                Tile::Ladder(pos) => {
                    let r = 9 - pos.y;
                    let c = if r & 1 == 1 { pos.x } else { 9 - pos.x };
                    v_ladder.push(Path::line(
                        Point {
                            x: p.x + Board::TILE_SIZE as f32 / 2.0,
                            y: p.y + Board::TILE_SIZE as f32 / 2.0,
                        },
                        Point {
                            x: c as f32 * Board::TILE_SIZE as f32 + Board::TILE_SIZE as f32 / 2.0,
                            y: r as f32 * Board::TILE_SIZE as f32 + Board::TILE_SIZE as f32 / 2.0,
                        },
                    ));
                }
                _ => (),
            }
            frame.fill_text(text);
        }

        frame.fill(&v[0], Color::WHITE);

        for p in v {
            frame.stroke(
                &p,
                Stroke {
                    style: canvas::Style::Solid(Color::BLACK),
                    width: 3 as f32,
                    ..Default::default()
                },
            );
        }
        for p in v_snake {
            frame.stroke(
                &p,
                Stroke {
                    style: canvas::Style::Solid(Color::from_rgb8(255, 0, 0)),
                    width: 3 as f32,
                    ..Default::default()
                },
            );
        }
        for p in v_ladder {
            frame.stroke(
                &p,
                Stroke {
                    style: canvas::Style::Solid(Color::from_rgb8(0, 255, 0)),
                    width: 3 as f32,
                    ..Default::default()
                },
            );
        }

        for p in self.players.iter() {
            let pos = p.position;
            let r = 9 - pos.y;
            let c = if r & 1 == 1 { pos.x } else { 9 - pos.x };
            v_player.push((
                Path::circle(
                    Point {
                        x: c as f32 * Board::TILE_SIZE as f32 + p.offset.0 as f32,
                        y: r as f32 * Board::TILE_SIZE as f32 + p.offset.1 as f32,
                    },
                    10.0,
                ),
                p.color.clone(),
            ));
        }

        for (p, c) in v_player {
            frame.fill(&p, c);
        }
        vec![frame.into_geometry()]
    }
}

#[derive(Debug, Clone)]
struct Player {
    name: String,
    position: Position,
    color: Color,
    offset: (i32, i32),
}

struct GamePage {
    game_type: Mode,
    dice_value: u8,
    board: Board,
    player_turn: usize,
    ended: bool,
}

impl GamePage {
    fn new(config: Config) -> Self {
        GamePage {
            game_type: config.game_type,
            dice_value: 0,
            board: Board::new(config.players),
            player_turn: 0 as usize,
            ended: false,
        }
    }

    fn update(&mut self, message: GameMsg) {
        match message {
            GameMsg::RollDice => {
                self.game_logic();
            }
            _ => (),
        }
    }

    fn view(&self) -> Element<'_, GameMsg> {
        row![
            Container::new(
                widget::canvas(self.board.clone())
                    .width(Length::Fixed(1000 as f32))
                    .height(Length::Fixed(1000 as f32))
            )
            .align_x(Center)
            .align_y(Center)
            .height(Length::Fill)
            .width(Length::FillPortion(12)),
            widget::Rule::vertical(2),
            column![
                self.status(),
                widget::Rule::horizontal(4),
                self.dice(),
                widget::Rule::horizontal(2),
                self.ranking(),
                widget::Rule::horizontal(2),
                Container::new(
                    widget::button(
                        widget::text("Exit to main Menu")
                            .align_x(Center)
                            .align_y(Center)
                            .width(Fill)
                            .size(30)
                    )
                    .on_press(GameMsg::GoToMenu)
                    .width(Length::Fill)
                )
                .padding(50)
                .center_x(Length::Fill),
            ]
            .height(Length::Fill)
            .width(Length::FillPortion(4)),
        ]
        .into()
    }

    fn status(&self) -> Container<'_, GameMsg> {
        Container::new(column![
            Container::new(column![
                Container::new(widget::text("Status").size(30))
                    .padding(10)
                    .align_x(Center)
                    .width(Length::Fill),
                widget::Rule::horizontal(4),
                Container::new(widget::text("next player's turn"))
                    .padding(5)
                    .align_x(Center)
                    .width(Length::Fill),
            ])
            .style(|_| Style {
                background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
                border: Border {
                    color: Color::WHITE,
                    width: 2 as f32,
                    radius: Radius::new(20),
                },
                ..Default::default()
            })
            .padding(3)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Shrink)
            .align_top(Length::Shrink),
        ])
        .padding(50)
        .width(Length::Fill)
        .center_x(Length::Fill)
    }

    fn dice(&self) -> Container<'_, GameMsg> {
        Container::new(column![
            Container::new(
                widget::text(self.dice_value)
                    .size(100)
                    .align_x(Center)
                    .align_y(Center)
                    .width(Fill)
            ),
            Container::new(
                widget::button(
                    widget::text("Roll Dice")
                        .size(30)
                        .align_x(Center)
                        .align_y(Center)
                        .width(Fill)
                )
                .on_press(GameMsg::RollDice)
                .width(Length::Fill)
            )
        ])
        .padding(50)
        .align_x(Center)
        .align_y(Center)
        .width(Length::Fill)
    }

    fn ranking(&self) -> Container<'_, GameMsg> {
        let mut ranks: Vec<&Player> = self.board.players.iter().collect();
        ranks.sort_by_key(|p| -(p.position.y * 10 + p.position.x + 10));

        let r_list: Vec<Element<GameMsg>> = ranks
            .iter()
            .enumerate()
            .map(|(i, p)| {
                Container::new(column![
                    widget::Rule::horizontal(4),
                    row![
                        widget::text(i + 1)
                            .size(20)
                            .width(Length::FillPortion(2))
                            .align_x(Center)
                            .align_y(Center),
                        widget::text(&p.name)
                            .size(20)
                            .width(Length::FillPortion(6))
                            .align_x(Center)
                            .align_y(Center),
                        Container::new(
                            Container::new("")
                                .width(Length::Fixed(16.0))
                                .height(Length::Fixed(16.0))
                                .style(|_| {
                                    Style {
                                        background: Some(Background::Color(p.color.clone())),
                                        ..Default::default()
                                    }
                                })
                        )
                        .width(Length::FillPortion(2))
                        .align_x(Center),
                        widget::text(p.position.y * 10 + p.position.x + 1)
                            .align_x(Center)
                            .align_y(Center)
                            .size(20)
                            .width(Length::FillPortion(2)),
                    ]
                    .padding(7)
                    .align_y(Center)
                ])
                .align_y(Center)
                .align_x(Center)
                .center_y(Length::Shrink)
                .width(Length::Fill)
                .into()
            })
            .collect();
        let r_list = widget::column(r_list);
        Container::new(column![
            Container::new(column![
                Container::new(widget::text("Ranking").size(30))
                    .padding(10)
                    .align_x(Center)
                    .width(Length::Fill),
                widget::Rule::horizontal(4),
                Container::new(column![
                    widget::Rule::horizontal(4),
                    row![
                        widget::text("Rank")
                            .size(20)
                            .width(Length::FillPortion(2))
                            .align_x(Center)
                            .align_y(Center),
                        widget::text("Player Name")
                            .size(20)
                            .width(Length::FillPortion(6))
                            .align_x(Center)
                            .align_y(Center),
                        Container::new(
                            widget::text("Color")
                                .align_x(Center)
                                .align_y(Center)
                                .size(20)
                                .width(Length::FillPortion(2)),
                        )
                        .width(Length::FillPortion(2))
                        .align_x(Center),
                        widget::text("Tile")
                            .align_x(Center)
                            .align_y(Center)
                            .size(20)
                            .width(Length::FillPortion(2)),
                    ]
                    .padding(7)
                    .align_y(Center)
                ])
                .align_y(Center)
                .align_x(Center)
                .center_y(Length::Shrink)
                .width(Length::Fill),
                r_list,
            ])
            .style(|_| Style {
                background: Some(Background::Color(Color::from_rgb8(0, 0, 0))),
                border: Border {
                    color: Color::WHITE,
                    width: 2 as f32,
                    radius: Radius::new(20),
                },
                ..Default::default()
            })
            .padding(3)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Shrink)
            .align_top(Length::Shrink),
        ])
        .padding(50)
        .width(Length::Fill)
        .center_x(Length::Fill)
    }

    fn roll_dice() -> u8 {
        rand::thread_rng().gen_range(1..=6) as u8
    }

    fn game_logic(&mut self) {
        if self.ended {
            return ();
        }
        self.dice_value = GamePage::roll_dice();
        let total_players = self.board.players.len();
        let (left, right) = self.board.players.split_at_mut(self.player_turn);
        let (current_player, right) = right.split_at_mut(1);
        let current_player = &mut current_player[0];
        let pos = current_player.position.y * 10 + current_player.position.x;
        let new_pos = pos + self.dice_value as i32;
        if self.dice_value != 6 {
            self.player_turn += 1;
            if self.player_turn == total_players {
                self.player_turn = 0;
            }
        }
        if new_pos > 99 {
            return ();
        } else if new_pos == 99 {
            self.ended = true;
        }
        let old_position = current_player.position;
        match self.board.tile[(new_pos / 10) as usize][(new_pos % 10) as usize] {
            Tile::Snake(pos) | Tile::Ladder(pos) => current_player.position = pos,
            _ => {
                current_player.position = Position {
                    x: new_pos % 10,
                    y: new_pos / 10,
                };
            }
        }

        match self.game_type {
            Mode::Bump => {
                for p in left.iter_mut().chain(right.iter_mut()) {
                    if current_player.position == p.position {
                        p.position = Position { x: 0, y: 0 };
                    }
                }
            }
            Mode::Swap => {
                if let Some(p) = left
                    .iter_mut()
                    .chain(right.iter_mut())
                    .find(|i| i.position == current_player.position)
                {
                    p.position = old_position;
                }
            }
            _ => (),
        }
    }
}

fn main() -> iced::Result {
    let window_settings = Settings {
        size: iced::Size {
            width: 320.0 as f32,
            height: 400.0 as f32,
        },
        position: WindowPosition::Centered,
        ..Default::default()
    };
    let app = iced::application("snakes_n_ladders", App::update, App::view);
    let app = app
        .window(window_settings)
        .theme(|_state| iced::Theme::Dracula);
    app.run()
}
