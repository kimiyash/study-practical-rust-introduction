#[derive(Debug)]
pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> Default for ToyVec<T> {
    fn default() -> Self {
        Self::new()
    }
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
            // Vec<T> の into_iter(self) なら要素の所有権が得られる
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }
}
