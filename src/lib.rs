use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct BcdmsArray<T> {
    index: Vec<Vec<T>>, // the index pointing to the data blocks,

    // TODO: maybe compute ad hoc, could be quite expensive (high constant)
    n: usize, // number of elements

    // true iff number of superblocks is odd
    s_odd: bool,

    // Definitely needed IMO
    len_last_super: usize, // length of super block (amount of data blocks)
    cap_last_super: usize, // capacity of super block (amount of data blocks)

    empty_data_block: Option<Vec<T>>
}

impl<T> BcdmsArray<T> {
    pub fn push(&mut self, value: T) {
        self.grow();
        self.index.last_mut().unwrap().push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.shrink()
    }

    pub fn len(&self) -> usize {
        self.n
    }
    
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn capacity(&self) -> usize {
        self.n - self.index.last().unwrap().len() + self.index.last().unwrap().capacity()
    }

    pub fn read(&self, index: usize) -> Option<&T> {
        if index >= self.n {
            return None;
        }
        let (a, b) = BcdmsArray::<T>::locate(index);
        Some(&self.index[a][b])
    }

    fn grow(&mut self) {
        self.n += 1;

        // 1. If the last nonempty data block DB[d-1] is full
        if self.index.is_empty() {
            self.len_last_super += 1;
            self.cap_last_super = 1;
            self.s_odd = true;
            self.index.push(self.empty_data_block.take().unwrap());
            return;
        }
        if self.index.last().unwrap().capacity() == self.index.last().unwrap().len() {
            let mut cap = self.index.last().unwrap().capacity();
            // (a) If the last superblock SB[s-1] is full, add a new virtual superblock
            if self.len_last_super == self.cap_last_super {
                self.s_odd = !self.s_odd;
                if self.s_odd {
                    self.cap_last_super *= 2;
                } else {
                    cap *= 2;
                }
                self.len_last_super = 0;
            }
            self.len_last_super += 1;

            // (b) If there are no empty data blocks
            match self.empty_data_block.take() {
                Some(x) => {  self.index.push(x); }, 
                None => { self.index.push(Vec::with_capacity(cap)); },
            }

        }
    }

    fn shrink(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }

        let result = self.index.last_mut().unwrap().pop();
        // 2. If DB[d-1] is empty
        if self.index.last().unwrap().is_empty() {
            // Overwrite the empty_data_block with the new one
            self.empty_data_block = self.index.pop();
            self.len_last_super -= 1;
                // 2 b TODO reallocate index when quarter full???
            if self.len_last_super == 0 {
                self.s_odd = !self.s_odd;
                if !self.s_odd {
                    self.cap_last_super /= 2;
                }
                self.len_last_super = self.cap_last_super;
            }
        }

        self.n -= 1;
        result
    }

    fn locate(index: usize) -> (usize, usize) {
        let index = index + 1;

        let k = std::mem::size_of::<usize>() * 8 - index.leading_zeros() as usize - 1; // size of index - 1

        let l = (k + 1) / 2; // ceil(k/2)

        let b = (index & !(1 << k)) >> l; // get the first floor(k/2) bits of index after the leading One
                                          // remember that there are k bits after the leading 1 and that we have to cut off the last ceil(k/2) bits

        let e = index & ((1 << l) - 1); // get the last ceil(k/2) bits of index

        // There is an error in the paper. The number of data blocks in super blocks prior to SB[k] is not 2^k - 1, since an SB[i] has 2^floor(i/2) data blocks, not 2^i
        let p = 2 * ((1 << l) - 1) - (k % 2) * ((1 << l) / 2);

        // return e-th element of DB[p+b]
        (p + b, e)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.index.iter().flat_map(|x| x.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.index.iter_mut().flat_map(|x| x.iter_mut())
    }

    pub fn insert(&mut self, index: usize, element: T) {
        let (a, b) = BcdmsArray::<T>::locate(index);

        // We move the last element of a data block to the first position of the next data block, from back to front to prevent the data blocks from growing
        self.grow();
        for data_block in (a+1..self.index.len()).rev() {
            let elem_to_move = self.index[data_block - 1].pop().unwrap();
            self.index[data_block].insert(0, elem_to_move);
        }
    
        let cap = self.index[a].capacity();
        self.index[a].insert(b, element);
        assert_eq!(cap, self.index[a].capacity());
    }

    pub fn remove(&mut self, index: usize) -> T {
        let (a, b) = BcdmsArray::<T>::locate(index);
        let result = self.index[a].remove(b);

        for block in a+1..self.index.len() {
            let temp = self.index[block].remove(0);
            self.index[block - 1].push(temp);
        }
        self.index.last_mut().unwrap().push(result);

        self.shrink().unwrap()
    }

    
    pub fn simple_sanity_check(&self) {
        if self.is_empty() {
            return;
        }
        // We count the number of elements in the vectors and we check that every vector except the last one(s) are full
        let length = self.index.iter().map(|vec| vec.len()).sum();
        let result = self.len() == length;
        assert!(result);

        for idx in 0..self.index.len() - 1 {
            let vec = &self.index[idx];
            assert_eq!(vec.capacity(), vec.len());
        }

    }
}

type VecIter<T> = std::vec::IntoIter<T>;

impl<T> IntoIterator for BcdmsArray<T> {
    type Item = T;
    type IntoIter = std::iter::FlatMap<VecIter<Vec<T>>, VecIter<T>, fn(Vec<T>) -> VecIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.index.into_iter().flat_map(IntoIterator::into_iter)
    }
}

impl<T> Index<usize> for BcdmsArray<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        let (a, b) = BcdmsArray::<T>::locate(i);
        &self.index[a][b]
    }
}

impl<T> IndexMut<usize> for BcdmsArray<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        let (a, b) = BcdmsArray::<T>::locate(i);
        &mut self.index[a][b]
    }
}

impl<T> Default for BcdmsArray<T> {
    fn default() -> BcdmsArray<T> {
        BcdmsArray {
            index: Vec::new(),
            n: 0,
            s_odd: true,
            len_last_super: 0,
            cap_last_super: 1,
            empty_data_block: Some(Vec::with_capacity(1)),
        }
    }
}
