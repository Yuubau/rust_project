use serde::{Deserialize, Serialize};

pub type PublicLeaderBoard = Vec<Player>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SubscribeResult {
    Ok,
    Err (SubscribeError),
}

pub trait ChallengeResolve{
    type Input;
    type Output;
    fn name() -> String;
    fn new(input: Self::Input) -> Self;
    fn solve(&self) -> Self::Output;
    fn verify(&self, answer: &Self::Output) -> bool;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player{
    pub name: String,
    pub(crate) stream_id: String,
    pub(crate) total_used_time: f64,
    pub(crate) steps: u32,
    pub(crate) is_active: bool,
    pub(crate) score: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Challenge {
    MD5Hash(MD5HashInput)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeValue{
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer {
    MD5Hash(MD5HashOutput)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReChallengeResult{
    name: String,
    value: ChallengeValue
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Hello,

    Welcome {
        version: u8,
    },

    Subscribe {
        name: String,
    },

    SubscribeResult (SubscribeResult),

    Challenge(Challenge),

    ChallengeResult{
        answer: ChallengeAnswer,
        next_target: String
    },

    ChallengeTimeout(
        String
    ),

    PublicLeaderBoard(Vec<Player>),

    RoundSummary{
        challenge: String,
        chain: Vec<ReChallengeResult>
    },

    EndOfGame{leader_board: Vec<Player>}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MD5HashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashOutput {
    pub seed: u64,
    pub hashcode: String,
}

