1.常用宏
（1）声明式宏: vec!、println!、write!
（2）过程式宏: #[derive(Debug)]、#[derive(PartialEq)] .
2.存储结构：
（1）结构体：
struct Person {
    name: String,
    age: u32
}

fn learn_struct() {
    // 初始化结构体
    let mut person = Person {
        name: String::from("Mr. Hello"),
        age: 23
    };

    // 索引成员
    println!("person: {}", person.name); // person: Mr. Hello

    // 修改结构体
    person.name = String::from("Mr. World");
    println!("person: {}", person.name); // person: Mr. World
}
（2）枚举结构
enum Result {
  Unknown,
  Err(u32),
  Content(String),
  Point{x: i32, y: i32}
}


fn learn_enum() {
    // 枚举初始化
    let mut result1 = Result::Unknown;
    let mut result2 = Result::Err(404);
    let mut result3 = Result::Content(String::from("hello enum"));
    let mut result4 = Result::Point{x: 100, y: 200};

    // 枚举查询
    if let Result::Point{x, y} = result4 {
        println!("point x: {}, y: {}", x, y);
    } 
    
    // 枚举更新
    match result4 {
        Result::Point{ref mut x, ref mut y} => {
            *x = 123;
            *y = 321;
        },
        _ =>()
    }

    // 再次查询更新结果
    if let Result::Point{x, y} = result4 {
        println!("point x: {}, y: {}", x, y);
    } 

}
（3）数组类型
fn learn_array() {
    // 定义和初始化
    let mut arr:[i32;4] = [10, 20, 30, 40];
    
    // 访问
    println!("[0]: {}, [1]: {}, [2]: {}, [3]: {}", arr[0], arr[1], arr[2], arr[3]);

    // 更新成员
    arr[0] = 11;
    arr[1] = 22;
    println!("[0]: {}, [1]: {}", arr[0], arr[1]);


    let mut matrix: [[i32;3];3] = [
        [1, 2, 3],
        [2, 3, 4],
        [4, 5, 6]
    ];
    // 更新二维数据
    matrix[0][0] = 23;
    println!("matrix[0][0]: {}", matrix[0][0]);

}