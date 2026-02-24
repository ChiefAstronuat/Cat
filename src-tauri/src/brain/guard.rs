
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::brain::fsm::CatState;

pub struct GuardLayer {
    focus_mode: bool,
    last_action_times: HashMap<CatState, Instant>,
    action_counts: HashMap<CatState, u32>,
    last_reset_time: Instant,
}

impl GuardLayer {
    pub fn new() -> Self {
        Self {
            focus_mode: false,
            last_action_times: HashMap::new(),
            action_counts: HashMap::new(),
            last_reset_time: Instant::now(),
        }
    }

    pub fn set_focus_mode(&mut self, is_focus: bool) {
        self.focus_mode = is_focus;
    }

    pub fn check_action_allowed(&mut self, action: CatState) -> bool {
        let now = Instant::now();
        
        // 집중 모드 제약: 방해가 되는 고강도/소음 행동 강력 억제
        if self.focus_mode {
            match action {
                CatState::Meow | CatState::Play | CatState::Run => return false,
                _ => {}
            }
        }

        // 과반복 쿨다운 (최근 행동 기록 기반)
        if let Some(last_time) = self.last_action_times.get(&action) {
            let cooldown = match action {
                CatState::Meow => Duration::from_secs(30), // 집중모드 아닐 때도 연속 울음 방지
                CatState::Play => Duration::from_secs(60),
                CatState::Groom => Duration::from_secs(15),
                CatState::Stretch => Duration::from_secs(20),
                CatState::Nap => Duration::from_secs(60), // 낮잠 후 바로 다시 낮잠 자는 것 방지
                _ => Duration::from_secs(0),
            };

            if now.duration_since(*last_time) < cooldown {
                return false;
            }
        }

        // 과반복 카운트 제약 (동일 행동 1분 내 과도 발생 제한)
        if let Some(count) = self.action_counts.get(&action) {
            if *count > 5 {
                // 5회가 넘어가면 무조건 기각 처리하고 다른 행동 유도
                return false;
            }
        }

        true
    }

    pub fn mark_action(&mut self, action: CatState) {
        let now = Instant::now();
        
        // 1분 지나면 카운트 리셋
        if now.duration_since(self.last_reset_time) > Duration::from_secs(60) {
            self.action_counts.clear();
            self.last_reset_time = now;
        }

        let count = self.action_counts.entry(action).or_insert(0);
        *count += 1;
        self.last_action_times.insert(action, now);
    }
}
