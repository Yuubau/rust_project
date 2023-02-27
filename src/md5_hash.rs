use hex::encode;
use md5::{Digest, Md5, Md5Core};
use md5::digest::consts::U64;
use md5::digest::core_api::CoreWrapper;
use md5::digest::Output;

use crate::structs::Challenge;
use crate::structs::ChallengeAnswer;
use crate::structs::ChallengeResolve;
use crate::structs::MD5HashInput;
use crate::structs::MD5HashOutput;

pub struct MD5Hash {
    pub input: MD5HashInput
}

impl ChallengeResolve for MD5Hash {

    type Input = MD5HashInput;
    type Output = MD5HashOutput;

    fn name() -> String {
        "HashCash".to_string()
    }

    fn new(input: Self::Input) -> Self {
        MD5Hash {
            input
        }
    }

    fn solve(&self) -> Self::Output {
        let mut seed_count = 0;
        let mut seed_count_hexa: String;
        let mut hash: String = "".to_string();
        let mut seed: String = "".to_string();
        let mut base_data: String;
        let mut result: Output<CoreWrapper<Md5Core>>;
        loop {
            seed_count_hexa = format!("{:x}", seed_count).to_uppercase();
            if seed_count_hexa.len() < 2{
                seed_count_hexa = "0".to_string() + seed_count_hexa.as_str();
            }
            seed = "".to_string();
            for zero in 0 .. (16 - seed_count_hexa.len()){
                seed = seed + "0";
            }

            seed += seed_count_hexa.as_str();
            base_data = seed.clone();
            base_data += self.input.message.as_str();

            let mut result: Output<CoreWrapper<Md5Core>>;
            let mut md5 = Md5::new();
            let mut format: String;

            md5.update(base_data.clone().into_bytes());
            result = md5.finalize();

            hash = "".to_string();
            for msd5Hash in result.to_vec() {
                format = format!("{:x}", msd5Hash);
                if format.len() >= 2{
                    hash = hash + format.as_str();
                }else{
                    hash = hash + "0" + format.as_str();
                }
            }
            println!("{}", hash.to_uppercase());
            println!("{}", seed_count);

            let mut bool = true;

            let min: u32 = self.input.complexity / 8;
            let max: u32 = 2u32.pow(8 - ( self.input.complexity % 8 ) );
            for i in 0..min {
                if result.to_vec().get(i as usize).unwrap() != &0 {
                    bool = false;
                }
            };
            result.to_vec().get(min as usize).unwrap() < &(max as u8);

            if bool {
                return MD5HashOutput {
                    seed: seed_count,
                    hashcode: hash.to_uppercase(),
                }
            } else {
                seed_count += 1;
            }
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        //RAF
        return true;
    }

}
