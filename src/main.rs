use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("전달된 슬라이스의 첫 요소는 {}입니다.", slice[0]);
    println!(
        "전달된 슬라이스는 {}개의 요소를 가지고 있습니다.",
        slice.len()
    );
}

fn main() {
    // 길이가 고정된 배열
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 모든 요소들을 같은 값으로 초기화
    let ys: [i32; 500] = [0; 500];

    println!("배열의 첫번째 요소: {}", xs[0]);
    println!("배열의 두번째 요소: {}", xs[1]);
    println!("배열의 요소의 갯수: {}", xs.len());

    println!("ys 배열의 첫번째 요소: {}", ys[0]);
    println!("ys 배열의 두번째 요소: {}", ys[1]);
    println!("ys 배열의 요소의 갯수: {}", ys.len());

    println!(
        "배열에는 {} 바이트가 할당되어 있습니다.",
        mem::size_of_val(&xs)
    );

    // 배열은 자동으로 슬라이스 형태로 빌릴수있다.
    println!("배열 전체를 슬라이스로 빌립니다.");
    analyze_slice(&xs);

    // 슬라이스느 배열의 특정 부분을 지정할 수 있습니다.
    // 문법은 [starting_index..ending_index] 입니다.
    // starting_index 는 슬라이스의 처음을 나타내고
    // ending_index 는 슬라이스의 마지막 위치를 나타냅니다.
    println!("배열의 일부를 슬라이스로 빌립니다");
    analyze_slice(&ys[1..4]);

    // 아래 문장에서는 index out of bounds 오류가 발생합니다.
    println!("{}", ys[5]);
}
