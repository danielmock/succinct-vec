//! The `SuccinctVec` behaves like a `Vec` with smaller asymptotic memory overhead.
//! It offers amortized constant time `push` and `pop` and constant worst-case time indexed access with `O(sqrt n)` asymptotic memory overhead.

use std::ops::{Index, IndexMut};

/// `SuccinctVec` guarantees `O(1)` amortized `push` and `pop` and worst-case `O(1)` indexed access.
/// The memory overhead is guaranteed to be `O(sqrt n)` where `n` is the length of the data structure, in contrast to `Vec`s linear overhead.
/// 
/// For performance it is recommended to use iterators instead of sequential indexed access.
/// 
/// ## Capacity and reallocation
/// Capacity and length are defined as for a `Vec`. 
/// However, if the length exceeds the capacity, its elements will not be reallocated.
/// Memory is allocated if and only if the length equals capacity.
/// Memory is deallocated if and only if there are two empty data blocks. 
/// However, this state is not observable for the user.
/// In both cases, the (de-)allocation has size `O(sqrt n)`.
/// 
/// ## Credits
/// Based on the paper "Resizable Arrays in Optimal Time and Space" by A. Brodnik, S. Carlsson, E. Demaine, J. Munro and R. Sedgewick.
/// Since most methods behave identically to the ones from `std::vec::Vec`, parts of its documentation were borrowed.
/// 
#[derive(Debug, Clone)]
pub struct SuccinctVec<T> {
    // the index pointing to the data blocks,
    data_blocks: Vec<Vec<T>>, 
    // number of elements
    len: usize, 

    // true iff number of superblocks is odd
    s_odd: bool,

    // length of super block (amount of data blocks)
    len_last_super: usize, 
    // capacity of super block (amount of data blocks)
    cap_last_super: usize, 
    empty_data_block: Option<Vec<T>> 
}

impl<T> SuccinctVec<T> {
    /// Returns the number of elements in the vector, also referred to as its 'length'.
    pub fn len(&self) -> usize {
        self.len
    }
    
    /// Returns `true` if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of elements the array can hold without reallocating (including the reserve data block)
    pub fn capacity(&self) -> usize {
        0 + match &self.empty_data_block {
            None => { 0 },
            Some(vec) => { vec.len() },
        } + if self.data_blocks.is_empty() { 0 } else { self.len - self.data_blocks.last().unwrap().len() + self.data_blocks.last().unwrap().capacity() }
    }

    /// Appends an element to the back of a collection.
    pub fn push(&mut self, value: T) {
        self.grow();
        self.data_blocks.last_mut().unwrap().push(value);
    }

    /// Removes the last element from a vector and returns it, or [`None`] if it is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let result = self.data_blocks.last_mut().unwrap().pop();
        self.shrink();
        result
    }

    /// Returns the last element of the slice, or None if it is empty.
    pub fn last(&self) -> Option<&T> {
        self.data_blocks.last().and_then(|vec| vec.last())
    }

    /// Returns a mutable pointer to the last item in the slice.
    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.data_blocks.last_mut().and_then(|vec| vec.last_mut())
    }

    /// Changes the state of the vector such that it can fit a new element at the end of the data structure (using push).
    /// Allocates memory iff `capacity == length`.
    fn grow(&mut self) {
        // The implementation follows the paper closely.
        self.len += 1;

        // added a necessary special case for the empty vector
        if self.data_blocks.is_empty() {
            self.len_last_super += 1;
            self.cap_last_super = 1;
            self.s_odd = true;
            self.data_blocks.push(self.empty_data_block.take().unwrap());
            return;
        }

        // 1. If the last nonempty data block DB[d-1] is full
        if self.data_blocks.last().unwrap().capacity() == self.data_blocks.last().unwrap().len() {
            let mut cap = self.data_blocks.last().unwrap().capacity();
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
                Some(x) => {  self.data_blocks.push(x); }, 
                None => { self.data_blocks.push(Vec::with_capacity(cap)); },
            }

        }
    }

    /// Changes the state of the vector such that it has an element less at the end. 
    /// The caller has to take care that the last element is removed.
    /// May deallocate memory.
    fn shrink(&mut self) {
        // The implementation follows the paper closely.

        //let result = self.data_blocks.last_mut().unwrap().pop();
        // 2. If DB[d-1] is empty
        if self.data_blocks.last().unwrap().is_empty() {
            // Overwrite the empty_data_block with the new one
            self.empty_data_block = self.data_blocks.pop();
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

        self.len -= 1;
        //result
    }

    /// Given the `index` it calculates the position of corresponding data block and the position inside this data block
    fn locate(index: usize) -> (usize, usize) {
        // The implementation follows the paper closely. It uses some small optimizations for Intel CPUs though.
        let index = index + 1;

        // the position of the leading 1 bit
        let k = std::mem::size_of::<usize>() * 8 - index.leading_zeros() as usize - 1; // size of index - 1

        // ceil(k/2), introduced for legibility reasosn
        let l = (k + 1) / 2; 

        // get the first floor(k/2) bits of index after the leading 1
        // remember that there are k bits after the leading 1 and that we have to cut off the last ceil(k/2) bits
        let b = (index & !(1 << k)) >> l; 

        // get the last ceil(k/2) bits of index
        let e = index & ((1 << l) - 1); 

        // There is an error in the paper. The number of data blocks in super blocks prior to SB[k] is not 2^k - 1, 
        // since an SB[i] has 2^floor(i/2) data blocks, not 2^i
        let p = 2 * ((1 << l) - 1) - (k % 2) * ((1 << l) / 2);

        // return e-th element of DB[p+b]
        (p + b, e)
    }

    /// Returns an iterator over the slice.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data_blocks.iter().flat_map(|x| x.iter())
    }


    /// Returns an iterator that allows modifying each value.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data_blocks.iter_mut().flat_map(|x| x.iter_mut())
    }

    /// Inserts an element at position index within the vector, shifting all elements after it to the right.
    pub fn insert(&mut self, index: usize, element: T) {
        let (a, b) = SuccinctVec::<T>::locate(index);

        // We move the last element of a data block to the first position of the next data block, from back to front to prevent the data blocks from growing
        self.grow();
        for data_block in (a+1..self.data_blocks.len()).rev() {
            let elem_to_move = self.data_blocks[data_block - 1].pop().unwrap();
            self.data_blocks[data_block].insert(0, elem_to_move);
        }
    
        let cap = self.data_blocks[a].capacity();
        self.data_blocks[a].insert(b, element);
        assert_eq!(cap, self.data_blocks[a].capacity());
    }

    /// Removes and returns the element at position index within the vector, shifting all elements after it to the left.
    /// # Panics
    /// Panics if index `is` out of bounds.
    pub fn remove(&mut self, index: usize) -> T {
        let (a, b) = Self::locate(index);
        let result = self.data_blocks[a].remove(b);

        for block in a+1..self.data_blocks.len() {
            let temp = self.data_blocks[block].remove(0);
            self.data_blocks[block - 1].push(temp);
        }

        self.shrink();
        result
    }

    /// Removes the element at `index` and returns it.
    /// The removed element is replaced by `replacement`.
    pub fn swap_replace(&mut self, index: usize, replacement: T) -> T {
        // TODO replace this with unsafe code swapping the elements in the data block directly (or find an appropiate method in `Vec`)
        let (a, b) = Self::locate(index);
        let last = self.data_blocks[a].pop().unwrap();
        self.data_blocks[a].push(replacement);
        let result = self.data_blocks[a].swap_remove(b);
        self.data_blocks[a].push(last);
        result
    }

    
    pub fn simple_sanity_check(&self) {
        if self.is_empty() {
            return;
        }
        // We count the number of elements in the vectors and we check that every vector except the last one(s) are full
        let length = self.data_blocks.iter().map(|vec| vec.len()).sum();
        let result = self.len() == length;
        assert!(result);

        for idx in 0..self.data_blocks.len() - 1 {
            let vec = &self.data_blocks[idx];
            assert_eq!(vec.capacity(), vec.len());
        }

    }
}

type VecIter<T> = std::vec::IntoIter<T>;
pub type SuccinctIter<T> = std::iter::FlatMap<VecIter<Vec<T>>, VecIter<T>, fn(Vec<T>) -> VecIter<T>>;

impl<T> IntoIterator for SuccinctVec<T> {
    type Item = T;
    type IntoIter = SuccinctIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data_blocks.into_iter().flat_map(IntoIterator::into_iter)
    }
}

impl<T> Index<usize> for SuccinctVec<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        let (a, b) = Self::locate(i);
        &self.data_blocks[a][b]
    }
}

impl<T> IndexMut<usize> for SuccinctVec<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        let (a, b) = Self::locate(i);
        &mut self.data_blocks[a][b]
    }
}

impl<T> Default for SuccinctVec<T> {
    fn default() -> SuccinctVec<T> {
        SuccinctVec {
            data_blocks: Vec::new(),
            len: 0,
            s_odd: true,
            len_last_super: 0,
            cap_last_super: 1,
            empty_data_block: Some(Vec::with_capacity(1)),
        }
    }
}
