# string literal
    string 有两种表达方式:
    1. 一对双引号之间的字符.
    2. 一对 三个双引号 之间的字符.
    要求用反斜杠转义四个符号: " \ { }
    其中花括号是用来包裹变量的, 这个被包裹的变量必须实现了 to_string()函数.
    TODO: raw_string. 前面加r#..#".
    TODO: 三引号里的引号, 我想让里面的引号转义, 但没有实现,
          这个当转移到rust中的时候会有bug
    """ abc " """ 会转成 " abc " ", rust 要报错
    这个的解决办法是在ide中集成, 
    在ide中有个快捷键或者 个引号会触发一个文本编辑器,
    在里面编辑想要的文本.

# WHITESPACE
    本语言对 空格 换行 tab符 不敏感.
    也就是说理论上任何两个非原子语法之间,
    都可以插入上述三种符号的任意组合而不改变语法的含义.
    但是鉴于可能会有非原本含义的误解析,
    比如会出现如果忘记打分号, 会把两行解析到一块的"误解".
    在有 换行, tab, 三个及三个以上空格 的情况下, 解析器不会自动去"修改优化"代码,
    而是会要求手动修改.
    TODO: 目前没有很好的 to_rust注释 的方法, 暂时把注释放在whitespace里, 跳过不解析.

# def fn head | function call statement | function call expr
    TODO: 符号重载
    定义函数的时候, 标准函数是这样的:
    abc ()
    abc (d: d)
    abc (d: d) ef ()
    abc (d: d) ef (g: g)

    中间的括号必须带参数, 空括号放在中间是无意义的.
    如果写成 abc (d: d) ef 这样, 也不会报错, ide会在尾部补一个空括号.

    函数调用的时候:
    abc () ef () 是语法糖, 去糖之后是:
        abc (_) ef ()
        下划线代表默认值
    abc (d) ef 是 abc (d) ef () 的语法糖

# number literal
    标准数字格式是 数字 + 空格 或者 数字 + 下划线 的形式.
    第一位必须是数字, 下划线和空格不能混用.
    合法: 001 002  => 1002
    合法: 012 003  => 12003
    合法: 01 2 0 0 3  => 12003
    合法: 12 003  => 12003
    合法: 12_0_0_3  => 12003
    合法: 12_003  => 12003
    不合法: _12 003  => 12003 // 第一位必须是数字
    不合法: 12__003  => 12003 // 数字之间只能有一个空格或者下划线
    不合法: 1 112__003  => 1112003 // 空格和下划线不能混用


