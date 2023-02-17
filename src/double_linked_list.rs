use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// 双链表
pub struct LinkedList<T> {
    // TODO: YOUR CODE HERE
    marker: PhantomData<T>, // 可以去掉
}

/// 链表节点
struct Node<T> {
    // TODO: YOUR CODE HERE
    marker: PhantomData<T>, // 可以去掉
}

/// 链表迭代器
pub struct Iter<'a, T> {
    // TODO: YOUR CODE HERE
    marker: PhantomData<&'a T>,
}

/// 链表可变迭代器
pub struct IterMut<'a, T> {
    // TODO: YOUR CODE HERE
    marker: PhantomData<&'a mut T>,
}

impl<T> LinkedList<T> {
    /// 创建一个空链表
    pub fn new() -> Self {
        // Self {
        //     // TODO: YOUR CODE HERE
        //     marker: PhantomData,
        // }
        unimplemented!()
    }

    /// 将元素插入到链表头部
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1);
    /// assert_eq!(list.front(), Some(&1));
    /// ```
    pub fn push_front(&mut self, _elem: T) {
        // TODO: YOUR CODE HERE
        unimplemented!()
    }

    /// 将元素插入到链表尾部
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.back(), Some(&1));
    /// ```
    pub fn push_back(&mut self, _elem: T) {
        // TODO: YOUR CODE HERE
        unimplemented!()
    }

    /// 将第一个元素返回
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_front(1);
    /// assert_eq!(list.pop_front(), Some(1));
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        // TODO: YOUR CODE HERE
        unimplemented!()
    }

    /// 将最后一个元素返回
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.pop_back(), Some(1));
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        // TODO: YOUR CODE HERE
        unimplemented!()
    }

    /// 返回链表第一个元素的引用  
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.front(), None);
    /// list.push_front(1);
    /// assert_eq!(list.front(), Some(&1));
    /// ```
    pub fn front(&self) -> Option<&T> {
        // TODO: YOUR CODE HERE
        unimplemented!()
    }

    /// 返回链表第一个元素的可变引用   
    pub fn front_mut(&mut self) -> Option<&mut T> {
        // TODO: YOUR CODE HERE
        unimplemented!()
    }

    /// 返回链表最后一个元素的引用
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.back(), None);
    /// list.push_back(1);
    /// assert_eq!(list.back(), Some(&1));
    /// ```
    pub fn back(&self) -> Option<&T> {
        // TODO: YOUR CODE HERE
        unimplemented!()
    }

    /// 返回链表最后一个元素的可变引用
    pub fn back_mut(&mut self) -> Option<&mut T> {
        // TODO: YOUR CODE HERE
        unimplemented!()
    }

    /// 返回链表长度
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        // TODO: YOUR CODE HERE
        unimplemented!()
    }

    /// 判断链表是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 清空链表
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.len(), 2);
    /// list.clear();
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        // Oh look it's drop again
        while self.pop_front().is_some() {}
    }

    /// 返回一个迭代器
    pub fn iter(&self) -> Iter<T> {
        // Iter {
        //     // TODO: YOUR CODE HERE
        //     marker : PhantomData,
        // }
        unimplemented!();
    }

    /// 返回一个可变迭代器
    pub fn iter_mut(&mut self) -> IterMut<T> {
        // IterMut {
        //     // TODO: YOUR CODE HERE
        //     marker: PhantomData,
        // }
        unimplemented!();
    }

    /// 获取链表中指定位置的元素   
    /// 如果超出范围，使用panic!宏抛出异常
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.get(0), &1);
    /// ```
    pub fn get(&self, _at: usize) -> &T {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }

    /// 获取链表中指定位置的可变元素
    pub fn get_mut(&mut self, _at: usize) -> &mut T {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }

    /// 将元素插入到**下标为i**的位置    
    /// 如果超出范围，使用panic!宏抛出异常
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.insert(0,1);
    /// list.insert(1,3);
    /// list.insert(1,2);
    /// assert_eq!(list.get(0), &1);
    /// assert_eq!(list.get(1), &2);
    /// assert_eq!(list.get(2), &3);
    /// ```
    pub fn insert(&mut self, _at: usize, _data: T) {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }

    /// 移除链表中下标为i的元素
    /// 如果超出范围，使用panic!宏抛出异常
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::from_iter(vec![1,2,3]);
    /// assert_eq!(list.remove(1), 2);
    pub fn remove(&mut self, _at: usize) -> T {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }

    /// 将链表分割成两个链表，原链表为[0,at-1]，新链表为[at,len-1]。
    /// 如果超出范围，使用panic!宏抛出异常
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::from_iter(vec![1,2,3,4,5]);
    /// let new = list.split_off(2); // list = [1,2], new = [3,4,5]
    /// assert_eq!(list.len(), 2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(2));
    pub fn split_off(&mut self, _at: usize) -> LinkedList<T> {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }

    /// 查找链表中第一个满足条件的元素
    /// 
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::from_iter(vec![1,2,3]);
    /// assert_eq!(list.find_mut(|x| x % 2 == 0), Some(&mut 2));
    /// assert_eq!(list.find_mut(|x| x % 4 == 0), None);
    /// ```
    pub fn find_mut<P>(&mut self,predicate:P)->Option<&mut T> where P:Fn(&T) -> bool{
        // TODO: YOUR CODE HERE
        unimplemented!();
    }
}

impl<T: PartialEq> LinkedList<T> {
    /// 判断链表中是否包含某个元素
    ///
    /// # Examples
    /// ```
    /// use linked_list::double_linked_list::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// assert_eq!(list.contains(&1), true);
    /// assert_eq!(list.contains(&2), false);
    /// ```
    pub fn contains(&mut self, _data: &T) -> bool {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    // 返回下一个元素，当没有元素可返回时，返回None
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }

    // 返回(self.len, Some(self.len))即可
    fn size_hint(&self) -> (usize, Option<usize>) {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    // 返回前一个元素
    fn next_back(&mut self) -> Option<Self::Item> {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }
}

/// 提供归并排序的功能
pub trait MergeSort {
    /// 就地进行归并排序，按从小到大排序
    fn merge_sort(&mut self);
}

impl<T: PartialOrd + Default> LinkedList<T> {
    // 你可以在这里添加你需要的辅助函数
}

impl<T: PartialOrd + Default> MergeSort for LinkedList<T> {
    fn merge_sort(&mut self) {
        // TODO: YOUR CODE HERE
        unimplemented!();
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // Pop until we have to stop
        while self.pop_front().is_some() {}
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for item in self {
            new_list.push_back(item.clone());
        }
        new_list
    }
}
impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}
impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self {
            item.hash(state);
        }
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

unsafe impl<'a, T: Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}

unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}
unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}
