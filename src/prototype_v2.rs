use crate::nested_tuples::{Flatten, NestedTupleAppend};

pub trait Returns {
    type Ret;

    fn ret(&self) -> Self::Ret;
}

pub struct ReturnsStr(&'static str);

impl Returns for ReturnsStr {
    type Ret = &'static str;

    fn ret(&self) -> Self::Ret {
        self.0
    }
}

pub struct ReturnsInt(i32);

impl Returns for ReturnsInt {
    type Ret = i32;

    fn ret(&self) -> Self::Ret {
        self.0
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////

pub trait RetList: Returns {}

pub struct Nil;

impl Returns for Nil {
    type Ret = ();

    fn ret(&self) -> Self::Ret {}
}

impl RetList for Nil {}

pub struct ConsRet<Head, Tail>
where
    Head: Returns,
    Tail: RetList,
    Tail::Ret: NestedTupleAppend<Head::Ret>,
{
    head: Head,
    tail: Tail,
}

impl<Head, Tail> RetList for ConsRet<Head, Tail>
where
    Head: Returns,
    Tail: RetList,
    Tail::Ret: NestedTupleAppend<Head::Ret>,
{
}

impl<Head, Tail> Returns for ConsRet<Head, Tail>
where
    Head: Returns,
    Tail: RetList,
    Tail::Ret: NestedTupleAppend<Head::Ret>,
{
    type Ret = <Tail::Ret as NestedTupleAppend<Head::Ret>>::Output;

    fn ret(&self) -> Self::Ret {
        self.tail.ret().append(self.head.ret())
    }
}

pub struct ConsBlank<Tail>
where
    Tail: RetList,
{
    tail: Tail,
}

impl<Tail> Returns for ConsBlank<Tail>
where
    Tail: RetList,
{
    type Ret = Tail::Ret;

    fn ret(&self) -> Self::Ret {
        self.tail.ret()
    }
}

impl<Tail> RetList for ConsBlank<Tail> where Tail: RetList {}

pub struct ListWrapper<L: RetList>(L);

pub fn new_list() -> ListWrapper<Nil> {
    ListWrapper(Nil)
}

impl<L: RetList> ListWrapper<L> {
    pub fn eval(&self) -> <L::Ret as Flatten>::Unpacked
    where
        L::Ret: Flatten,
    {
        self.0.ret().flatten_unpack()
    }

    pub fn push_ret<R>(self, x: R) -> ListWrapper<ConsRet<R, L>>
    where
        R: Returns,
        L::Ret: NestedTupleAppend<R::Ret>,
    {
        ListWrapper(ConsRet {
            head: x,
            tail: self.0,
        })
    }

    pub fn push_blank(self) -> ListWrapper<ConsBlank<L>> {
        ListWrapper(ConsBlank { tail: self.0 })
    }
}

pub fn example() {
    let list = new_list()
        .push_ret(ReturnsInt(100))
        .push_ret(ReturnsInt(200))
        .push_ret(ReturnsStr("hello!"))
        .push_blank()
        .push_ret(ReturnsInt(300));

    let x = list.eval();

    println!("{x:?}");

    let list2 = new_list().push_ret(ReturnsInt(100));

    let single = list2.eval();

    println!("{single:?}");
}
