/// Flappy Dragon 游戏
/// 一个简单的 Flappy Bird 风格游戏，使用 bracket-lib 实现。
use bracket_lib::prelude::*;


/// 游戏屏幕宽度
const SCREEN_WIDTH: i32 = 80;
/// 游戏屏幕高度
const SCREEN_HEIGHT: i32 = 50;
/// 帧持续时间（毫秒）
const FRAME_DURATION: f32 = 60.0;

/// 游戏模式枚举
/// 表示游戏当前所处的状态
enum GameMode {
    /// 主菜单状态
    Menu,
    /// 游戏进行中状态
    Playing,
    /// 游戏结束状态
    End,
}

/// 玩家角色结构体
struct Player {
    /// 玩家 X 坐标
    x: i32,
    /// 玩家 Y 坐标
    y: i32,
    /// 垂直速度
    velocity: f32,
}

impl Player {
    /// 创建新玩家
    ///
    /// # 参数
    /// * `x` - 初始 X 坐标
    /// * `y` - 初始 Y 坐标
    ///
    /// # 返回值
    /// 返回一个新的 Player 实例
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    /// 渲染玩家
    ///
    /// # 参数
    /// * `ctx` - BTerm 上下文，用于渲染
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'))
    }

    /// 应用重力并移动玩家
    ///
    /// 更新玩家的速度和位置，模拟重力效果
    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    /// 玩家拍打翅膀
    ///
    /// 给予玩家向上的速度
    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

/// 障碍物结构体
struct Obstacle {
    /// 障碍物 X 坐标
    x: i32,
    /// 缺口中心 Y 坐标
    gap_y: i32,
    /// 缺口大小
    size: i32,
}

impl Obstacle {
    /// 创建新障碍物
    ///
    /// # 参数
    /// * `x` - 障碍物的 X 坐标
    /// * `score` - 当前游戏得分，影响障碍物难度
    ///
    /// # 返回值
    /// 返回一个新的 Obstacle 实例
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    /// 渲染障碍物
    ///
    /// # 参数
    /// * `ctx` - BTerm 上下文，用于渲染
    /// * `player_x` - 玩家的 X 坐标，用于相对定位
    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        // 绘制上方障碍物
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        // 绘制下方障碍物
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    /// 检测玩家是否撞到障碍物
    ///
    /// # 参数
    /// * `player` - 玩家实例
    ///
    /// # 返回值
    /// 如果玩家撞到障碍物返回 true，否则返回 false
    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}

/// 游戏状态结构体
struct State {
    /// 玩家实例
    player: Player,
    /// 累计的帧时间
    frame_time: f32,
    /// 当前障碍物
    obstacle: Obstacle,
    /// 当前游戏模式
    mode: GameMode,
    /// 当前得分
    score: i32,
}

impl GameState for State {
    /// 游戏每一帧的主循环函数
    ///
    /// # 参数
    /// * `ctx` - BTerm 上下文
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

impl State {
    /// 创建新游戏状态
    ///
    /// # 返回值
    /// 返回一个初始化的 State 实例
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::Menu,
            score: 0,
        }
    }

    /// 处理主菜单状态
    ///
    /// # 参数
    /// * `ctx` - BTerm 上下文
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon!");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    /// 处理游戏进行中状态
    ///
    /// # 参数
    /// * `ctx` - BTerm 上下文
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press Space to flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        ctx.print(
            0,
            2,
            &format!("Player({}, {})", self.player.x, self.player.y),
        );
        ctx.print(
            0,
            3,
            &format!("Obstacle({}, {})", self.obstacle.x, self.obstacle.gap_y),
        );
        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    /// 处理游戏结束状态
    ///
    /// # 参数
    /// * `ctx` - BTerm 上下文
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    /// 重新开始游戏
    ///
    /// 重置所有游戏状态以开始新游戏
    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
        self.score = 0;
    }
}

/// 程序入口点
///
/// 创建游戏窗口并启动游戏循环
///
/// # 返回值
/// 返回 BError，表示游戏运行状态
fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
