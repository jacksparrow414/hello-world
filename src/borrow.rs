/**
 * https://course.rs/basic/ownership/ownership.html
 * 
 *栈中的所有数据都必须占用已知且固定大小的内存空间，假设数据大小是未知的，那么在取出数据时，你将无法取到你想要的数据
 * 这句话很重要，必须是已知且固定的大小。由此可以推断出，Rust中的基本类型是可以直接在栈中的
 * 
 * 下面这段话同样重要
 * 【与栈不同，对于大小未知或者可能变化的数据，我们需要将它存储在堆上】。
 * 当向堆上放入数据时，需要请求一定大小的内存空间。操作系统在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的指针, 该过程被称为在堆上分配内存，有时简称为 “分配”(allocating)。
 * 接着，该指针会被推入栈中，因为指针的大小是已知且固定的，在后续使用过程中，你将通过栈中的指针，来获取数据在堆上的实际内存位置，进而访问该数据。
 * 由上可知，堆是一种缺乏组织的数据结构。想象一下去餐馆就座吃饭: 进入餐馆，告知服务员有几个人，然后服务员找到一个够大的空桌子（堆上分配的内存空间）并领你们过去。如果有人来迟了，他们也可以通过桌号（栈上的指针）来找到你们坐在哪
 */

 /**
  * 如果你在其他语言中听说过术语 浅拷贝(shallow copy) 和 深拷贝(deep copy)，那么拷贝指针、长度和容量而不拷贝数据听起来就像浅拷贝，
  * 但是又因为 Rust 同时使第一个变量 s1 无效了，因此这个操作被称为 移动(move)，而不是浅拷贝
  * 将值传递给函数，一样会发生 【移动】 或者 【复制】
  */

  /**
   * 1、Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
   * 2、一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
   * 3、当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
   */
  fn main() {
    let s1 = String::from("hello");
    // https://course.rs/basic/ownership/ownership.html#%E8%BD%AC%E7%A7%BB%E6%89%80%E6%9C%89%E6%9D%83
    // 字符串深拷贝，调用clone
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
  }
  /**
   * 引用的作用域 s 从创建开始，一直持续到它最后一次使用的地方，这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号 }
   */