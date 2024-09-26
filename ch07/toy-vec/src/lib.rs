#[derive(Debug)]
pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size) // T型のデフォルト値をszie個作り
            .collect::<Vec<_>>() // Vec<T> に収集してから
            .into_boxed_slice() // Box<[T]> に変換する
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        self.elements[self.len] = element;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    pub fn get_or<'a, 'b>(&'a self, index: usize, default: &'b T) -> &'a T
    where
        'b: 'a, // 'bは'aより長生きする
    {
        self.get(index).unwrap_or(default)
    }

    // pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
    //     self.get(index).unwrap_or(default)
    //     // match self.get(index) {
    //     //     Some(v) => v,
    //     //     None => default,
    //     // }
    // }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // 所有権を交換している
            // let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            let elem = std::mem::take(&mut self.elements[self.len]);
            Some(elem)
        }
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            // 要素分の領域確保
            self.elements = Self::allocate_in_heap(1);
        } else {
            // 現在の2倍の領域を確保
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            // self.elements を置き換える
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            // 既存の全要素を新しい領域へムーブする
            // Vec<T> の into_iter(self) なら各要素の所有権が得られる
            // into_vec も into_iter も レシーバーの所有権を奪う
            // レシーバの所有権を奪うことで各エレメントの所有権を得ることができる。みたい。
            // into_vec で参照じゃなく実態にしてから into_iteer で各所有権得ているのかな
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
            // let _ = old_elements[0]; // 所有権を奪われているので、これがエラーになる

            // // 下記だと *elem の Copy トレイトが要求されてコンパイルエラーになる
            // for (i, elem) in old_elements.iter().enumerate() {
            //     self.elements[i] = *elem;
            // }

            // // 下記でも一緒
            // for (i, elem) in old_elements.into_iter().enumerate() {
            //     self.elements[i] = *elem;
            // }
        }
    }

    // 要素へのイミュータブルな参照（Option<&T>）を返すイテレータを作る
    // 説明のためにライフタイムを明示しているが、実際には省略できる
    pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
        Iter {
            elements: &self.elements,
            len: self.len,
            pos: 0,
        }
    }

    // 要素へのイミュータブルな参照（Option<&mut T>）を返すイテレータを作る
    pub fn iter_mut<'vec>(&'vec mut self) -> IterMut<'vec, T> {
        IterMut {
            elements: &mut self.elements,
            len: self.len,
            pos: 0,
        }
    }

    // 要素の所有権をとる（Option<T>）イテレータを作る
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            elements: self.elements,
            len: self.len,
            pos: 0,
        }
    }
}

pub struct Iter<'vec, T> {
    elements: &'vec Box<[T]>,
    len: usize,
    pos: usize,
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    type Item = &'vec T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

impl<'vec, T: Default> IntoIterator for &'vec ToyVec<T> {
    type Item = &'vec T; // イテレータがイテレートする値の型
    type IntoIter = Iter<'vec, T>; // into_iterメソッドの戻り値の型

    // &ToyVec<T>に対するトレイト実装なので、selfの型は ToyVec<T> ではなく &ToyVec<T>
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct IterMut<'vec, T> {
    elements: &'vec mut Box<[T]>,  // ミュータブルな参照
    len: usize,
    pos: usize,
}

impl<'vec, T> Iterator for IterMut<'vec, T> {
    type Item = &'vec mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            // 要素を&'vec mut Tとして返したいが、&'a mut selfから要素を取り出すと
            // 要素が&'a mut Tになってしまい、ライフタイム要件が満たせない
            // そこで以下のように対応した
            //   1. &'a mut Tを生ポインタ*mut Tに変換してライフタイムをなくす
            //   2. *mut Tの参照外しをして要素Tにアクセス
            //   3. 要素Tから&'vec mut Tを得る
            let elem = unsafe { &mut *(&mut self.elements[self.pos] as *mut T) };
            self.pos += 1;
            Some(elem)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'vec, T: Default> IntoIterator for &'vec mut ToyVec<T> {
    type Item = &'vec mut T;
    type IntoIter = IterMut<'vec, T>;

    // selfの型はToyVec<T>ではなく&mut ToyVec<T>
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct IntoIter<T> {
    elements: Box<[T]>,  // ミュータブルな参照
    len: usize,
    pos: usize,
}

impl<T: Default> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            // &mut selfから要素Tをムーブアウトできないのでreplaceでデフォルト値と交換している
            let elem = std::mem::take(&mut self.elements[self.pos]);
            self.pos += 1;
            Some(elem)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T: Default> IntoIterator for ToyVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    // selfの型はToyVec<T>
    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<T: Default> Default for ToyVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Default> Clone for ToyVec<T> {
    fn clone(&self) -> Self {
        let mut cloned = Self::with_capacity(self.len());
        // 各要素のcloneを呼ぶことでdeepコピーを実現する
        for elem in self.iter() {
            cloned.push(elem.clone());
        }
        cloned
    }
}