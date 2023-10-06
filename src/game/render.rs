use std::vec;

use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use super::game_struct::{Game, Screen};
use super::utils::calculate_bar;

impl Game {
    pub fn get_screen_name(&self) -> String {
        match self.screen {
            Screen::Battle(_) => "[Battle]".into(),
            Screen::Stats => "[Main menu]".into(),
            Screen::Gains(_) => "[Battle Gains]".into(),
        }
    }

    pub fn render(&mut self) -> Paragraph {
        let empty = Line::from("");
        match &self.screen {
            Screen::Battle(battle) => {
                let mut lines: Vec<Line<'_>> = vec![];

                let player = &battle.player;
                let enemy = &battle.enemy;
                lines.push(Line::from(vec![
                    player.get_name().bold(),
                    Span::raw(if battle.is_players_turn() { " <" } else { "" }),
                ]));
                lines.push(Line::from(vec![
                    Span::raw(player.health.to_string()),
                    Span::styled(" HP", Style::default().light_red().bold()),
                ]));

                lines.push(empty.clone());

                lines.push(Line::from(vec![
                    enemy.get_name().bold(),
                    Span::raw(if !battle.is_players_turn() { " <" } else { "" }),
                ]));
                lines.push(Line::from(vec![
                    Span::raw(enemy.health.to_string()),
                    Span::styled(" HP", Style::default().light_red().bold()),
                ]));

                Paragraph::new(lines)
            }
            Screen::Stats => {
                let playername = Line::from(self.player.get_name());
                let health = Line::from(vec![
                    Span::raw("Health | "),
                    Span::styled(
                        self.player.get_health().to_string(),
                        Style::default().bold().green(),
                    ),
                ]);
                let damage = Line::from(vec![
                    Span::raw("Damage | "),
                    Span::styled(
                        self.player.get_damage().to_string(),
                        Style::default().bold().fg(Color::LightRed),
                    ),
                ]);
                let level = Line::from(vec![
                    Span::raw("Level  | "),
                    Span::styled(
                        self.player.get_level().to_string(),
                        Style::default().bold().fg(Color::Yellow),
                    ),
                ]);
                let (filled, missing) =
                    calculate_bar(self.player.get_xp(), self.player.get_nxp(), 10);
                let xpbar = Line::from(vec![
                    Span::raw("       | ["),
                    Span::styled(filled, Style::default().bold().fg(Color::Gray)),
                    Span::styled(missing, Style::default().bold().fg(Color::DarkGray)),
                    Span::raw("]  "),
                    Span::raw(self.player.get_xp().to_string()),
                    Span::styled("/", Style::default().fg(Color::DarkGray)),
                    Span::raw(self.player.get_nxp().to_string()),
                    Span::styled(" XP", Style::default().fg(Color::DarkGray)),
                ]);
                let coins = Line::from(vec![
                    Span::raw("Coins  | "),
                    Span::styled(
                        self.player.get_coins().to_string(),
                        Style::default().bold().fg(Color::LightYellow),
                    ),
                ]);
                let info = Line::from("Press T to enter a battle");
                Paragraph::new(vec![
                    playername,
                    empty.clone(),
                    health,
                    damage,
                    level,
                    xpbar,
                    coins,
                    empty.clone(),
                    info,
                ])
            }
            Screen::Gains(gains) => {
                let mut lines: Vec<Line<'_>> = vec![];

                lines.push(Line::from("Battle gains".bold()));
                if gains.player_won() {
                    lines.push(Line::from(vec![
                        Span::raw("You won a battle against "),
                        Span::styled(gains.get_enemy_name(), Style::default().bold()),
                    ]));
                    lines.push(Line::from(vec![
                        Span::raw("You had "),
                        Span::styled(
                            gains.get_left_hp().to_string(),
                            Style::default().bold().light_red(),
                        ),
                        Span::raw(" hp left."),
                    ]));
                } else {
                    lines.push(Line::from(vec![
                        Span::raw("You lost a battle against "),
                        Span::styled(gains.get_enemy_name(), Style::default().bold()),
                    ]));
                    lines.push(Line::from(vec![
                        Span::raw("Enemy had "),
                        Span::styled(
                            gains.get_left_hp().to_string(),
                            Style::default().bold().light_red(),
                        ),
                        Span::raw(" hp left."),
                    ]));
                }
                lines.push(empty.clone());
                if gains.get_coins() > 0 {
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("+{}", gains.get_coins()),
                            Style::default().bold().light_green(),
                        ),
                        Span::styled(" coins", Style::default().bold().light_yellow()),
                    ]));
                }
                if gains.get_xp() > 0 {
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("+{}", gains.get_xp()),
                            Style::default().bold().light_green(),
                        ),
                        Span::styled(" xp", Style::default().bold().light_blue()),
                    ]));
                }
                lines.push(empty.clone());
                lines.push(Line::from("Press Enter to continue..."));
                Paragraph::new(lines)
            }
        }
    }
}
