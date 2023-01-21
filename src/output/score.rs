use crate::error::ReplayApiError;
use crate::input::battle_results::VehicleEarning;

#[derive(GraphQLObject, Clone)]
pub struct Assistance {
    total: i32,
    stun: i32,
    spot: i32,
    track: i32,
    inspire: i32,
    smoke: i32,
}

#[derive(GraphQLObject, Clone)]
pub struct Xp {
    base: i32,
}

#[derive(GraphQLObject, Clone)]
pub struct Shots {
    total: i32,
    direct_hit: i32,
    penetration: i32
}

#[derive(GraphQLObject, Clone)]
pub struct Score {
    damages: i32,
    kills: i32,
    assistance: Assistance,
    xp: Xp,
    shots: Shots,
}

impl Score {
    pub fn parse(score: &VehicleEarning) -> Result<Score, ReplayApiError> {
        Ok(Score {
            damages: score.damage_dealt as i32,
            kills: score.kills as i32,
            assistance: Assistance {
                stun: score.damage_assisted_stun as i32,
                spot: score.damage_assisted_radio as i32,
                track: score.damage_assisted_track as i32,
                inspire: score.damage_assisted_inspire as i32,
                smoke: score.damage_assisted_smoke as i32,
                total: (score.damage_assisted_stun +
                    score.damage_assisted_radio +
                    score.damage_assisted_track +
                    score.damage_assisted_inspire +
                    score.damage_assisted_smoke) as i32
            },
            xp: Xp {
                base: score.xp as i32,
            },
            shots: Shots {
                total: score.shots as i32,
                direct_hit: score.direct_hits as i32,
                penetration: score.piercing_enemy_hits as i32
            }
        })
    }
}