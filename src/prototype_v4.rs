use std::marker::PhantomData;

use crate::nested_tuples::{Flatten, NestedTupleAppend};

pub trait Returns {
    type Original;
    type Ret;

    fn ret(&self) -> Self::Ret;
}

pub trait ReturnsBuilder<RBOriginal> {
    type RBOutput: Returns<Original = RBOriginal>;

    fn build_ret(&self) -> Self::RBOutput;

    fn quick_ret(&self) -> <Self::RBOutput as Returns>::Ret {
        self.build_ret().ret()
    }
}

pub struct ReturnsStr(&'static str);

impl Returns for ReturnsStr {
    type Original = i32;
    type Ret = &'static str;

    fn ret(&self) -> Self::Ret {
        self.0
    }
}

impl ReturnsBuilder<i32> for ReturnsStr {
    type RBOutput = Self;

    fn build_ret(&self) -> Self::RBOutput {
        ReturnsStr(self.0)
    }
}

pub struct ReturnsInt(i32);

impl Returns for ReturnsInt {
    type Original = i32;
    type Ret = i32;

    fn ret(&self) -> Self::Ret {
        self.0
    }
}

impl ReturnsBuilder<i32> for ReturnsInt {
    type RBOutput = Self;

    fn build_ret(&self) -> Self::RBOutput {
        ReturnsInt(self.0)
    }
}

#[derive(Default)]
pub struct ReturnsDefault<D: Default> {
    _phantom: PhantomData<D>,
}

impl<D: Default> Returns for ReturnsDefault<D> {
    type Original = D;
    type Ret = D;

    fn ret(&self) -> Self::Ret {
        D::default()
    }
}

pub struct ReturnsDefaultBuilder;

impl<D: Default> ReturnsBuilder<D> for ReturnsDefaultBuilder {
    type RBOutput = ReturnsDefault<D>;

    fn build_ret(&self) -> Self::RBOutput {
        ReturnsDefault::default()
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////

pub trait RetList: Returns {}

pub struct Nil;

impl Returns for Nil {
    type Original = ();
    type Ret = ();

    fn ret(&self) -> Self::Ret {}
}

impl RetList for Nil {}

pub struct OriginalMarker<O> {
    _phantom: PhantomData<O>,
}

impl<O> Default for OriginalMarker<O> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<O> Returns for OriginalMarker<O> {
    type Original = O;
    type Ret = ();

    fn ret(&self) -> Self::Ret {}
}

impl<O> RetList for OriginalMarker<O> {}

pub struct ConsRet<Head, Tail>
where
    Head: ReturnsBuilder<Tail::Original>,
    Tail: RetList,
    Tail::Ret: NestedTupleAppend<<Head::RBOutput as Returns>::Ret>,
{
    head: Head,
    tail: Tail,
}

impl<Head, Tail> RetList for ConsRet<Head, Tail>
where
    Head: ReturnsBuilder<Tail::Original>,
    Tail: RetList,
    Tail::Ret: NestedTupleAppend<<Head::RBOutput as Returns>::Ret>,
{
}

impl<Head, Tail> Returns for ConsRet<Head, Tail>
where
    Head: ReturnsBuilder<Tail::Original>,
    Tail: RetList,
    Tail::Ret: NestedTupleAppend<<Head::RBOutput as Returns>::Ret>,
{
    type Original = Tail::Original;
    type Ret = <Tail::Ret as NestedTupleAppend<<Head::RBOutput as Returns>::Ret>>::Output;

    fn ret(&self) -> Self::Ret {
        let head_ret = self.head.quick_ret();
        self.tail.ret().append(head_ret)
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
    type Original = Tail::Original;
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

impl ListWrapper<Nil> {
    pub fn original_marker<O>(self) -> ListWrapper<OriginalMarker<O>> {
        ListWrapper(OriginalMarker::default())
    }
}

impl<L: RetList> ListWrapper<L> {
    pub fn eval(&self) -> <L::Ret as Flatten>::Unpacked
    where
        L::Ret: Flatten,
    {
        self.0.ret().flatten_unpack()
    }

    pub fn push_ret_builder<B>(self, builder: B) -> ListWrapper<ConsRet<B, L>>
    where
        B: ReturnsBuilder<L::Original>,
        L::Ret: NestedTupleAppend<<B::RBOutput as Returns>::Ret>,
    {
        ListWrapper(ConsRet {
            head: builder,
            tail: self.0,
        })
    }

    pub fn push_blank(self) -> ListWrapper<ConsBlank<L>> {
        ListWrapper(ConsBlank { tail: self.0 })
    }
}

pub fn example() {
    let list = new_list()
        .original_marker::<i32>()
        .push_ret_builder(ReturnsInt(100))
        .push_ret_builder(ReturnsInt(200))
        .push_ret_builder(ReturnsStr("hello!"))
        .push_blank()
        .push_ret_builder(ReturnsDefaultBuilder)
        .push_ret_builder(ReturnsInt(300));

    let x = list.eval();

    println!("{x:?}");

    let list2 = new_list()
        .original_marker::<String>()
        .push_ret_builder(ReturnsDefaultBuilder);

    let single = list2.eval();

    println!("{single:?}");
}
