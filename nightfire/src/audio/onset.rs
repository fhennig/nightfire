pub fn onset_score(vs1: &Vec<f32>, vs2: &Vec<f32>) -> f32 {
    let n = vs1.len();
    let mut score = 0f32;
    for i in 0..n {
        score += (vs2[i] - vs1[i]).abs();
    }
    score
}