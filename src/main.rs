use liao_generator::constants::{GenerateFormula, GenerateOptions};
use warp::Filter;

use std::collections::HashMap;

async fn expression(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: provide forbidden number list to the generator
    // when the generator is ready to take it in new version
    // let mut nums = params
    //     .get("nums")
    //     .unwrap_or(&"".to_string())
    //     .split(",")
    //     .map(|x| x.trim().parse::<i32>().unwrap_or(-1))
    //     .collect::<Vec<i32>>();
    // if nums.len() == 1 && nums[0] == -1 {
    //     nums = vec![];
    // }

    let formula_ = params.get("formula");
    let formula_str = match formula_ {
        Some(x) => x.as_str(),
        None => "",
    };

    let formula = match formula_str {
        "NF" => GenerateFormula::NF,
        "LF" => GenerateFormula::LF,
        "BF" => GenerateFormula::BF,
        "FF" => GenerateFormula::FF,
        _ => GenerateFormula::NF,
    };

    let min = params
        .get("min")
        .unwrap_or(&"".to_string())
        .parse::<i32>()
        .unwrap_or(1);
    let max = params
        .get("max")
        .unwrap_or(&"".to_string())
        .parse::<i32>()
        .unwrap_or(99);
    let len = params
        .get("len")
        .unwrap_or(&"".to_string())
        .parse::<usize>()
        .unwrap_or(10);

    let options = GenerateOptions {
        formula,
        min: min as i64,
        max: max as i64,
        len: len,
    };

    let terms = options.generate();

    // Send the response
    Ok(warp::reply::json(&terms))
}

#[tokio::main]
async fn main() {
    let expression_route = warp::get()
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::path("expression"))
        .and(warp::body::json())
        .and(warp::path::end())
        .and_then(expression);

    warp::serve(expression_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
