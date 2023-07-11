/*
Задание 2:
Реализовать собственный vector.
Ваш вектор должен иметь методы new, with_capacity (создаёт вектор с заранее заданым размером),
push, pop (возвращает последний элемент вектора, удаляя его из вектора),
remove (работает как pop, но принимает индекс элемента, который нужно вернуть), get, resize (изменяет размер вектора)
 */

use std::{fmt::{Display, Debug}, alloc::Layout};




fn main() {
    let mut v: V<i32> = V::with_capacity(30);
    v.resize(50);
    println!("{:?}", v);
    v.push(35);
    v.push(45);
    v.push(55);
    println!("Vector is: {}", v);
    println!("Vector length: {}", v.len());
    let x = v.pop().unwrap();
    println!("Droped: {:?}", x);
    println!("Vector is: {}", v);
    println!("Vector length: {}", v.len());
    println!("Vector is: {}", v);



    let mut v: V<&str> = V::new();
    v.push("Hello");
    v.push("World");
    println!("Vector is: {}", v);
    println!("Value removed: {}", v.remove(1).unwrap());
    println!("Vector is: {:?}", v);
    println!("Vector is: {}", v);
    v.push("!");
    println!("Vector is: {}", v);
    println!("Vector length: {}", v.len());
    v.insert(0, "W");
    println!("Vector is: {}", v);


}
#[derive(Clone, Debug)]
pub struct V<T> {
    ptr: *mut T,
    cap: usize,
    len: usize
}

impl<T> V<T> {

    pub fn new() -> V<T> {
        V {
            ptr: std::ptr::null_mut(),
            cap: 0,
            len: 0
        }
    }

    pub fn with_capacity(capacity: usize) -> V<T> {
        V {
            ptr: std::ptr::null_mut(),
            cap: if capacity > usize::MAX { usize::MAX } else { capacity },
            len: 0
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn resize(&mut self, capacity: usize) {
        let new_cap = if capacity > usize::MAX { usize::MAX } else { capacity };

        unsafe {
            self.ptr = std::alloc::realloc(self.ptr as *mut u8, Layout::array::<T>(capacity).unwrap(), new_cap) as *mut T;
        }

        self.cap = new_cap;

    }

    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.resize(self.cap + std::mem::size_of::<T>());
        }

        unsafe {
                std::ptr::write(self.ptr.add(self.len), value)
            }
        
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 { return None; }
        self.len -= 1;

        unsafe {
            Some(std::ptr::read(self.ptr.add(self.len)))
        }

    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.len { return None; }

        unsafe {
            Some(std::ptr::read(self.ptr.add(index)))
        }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        if self.len == self.cap {
            self.resize(self.cap + std::mem::size_of::<T>());
        }
        if index > self.len { panic!("Unavailable index"); }

        unsafe {
            std::ptr::copy(self.ptr.add(index), self.ptr.add(index + 1), self.len - index);
            std::ptr::write(self.ptr.add(index), value);
        }

        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index > self.len { return None }

        unsafe {
            let result = Some(std::ptr::read(self.ptr.add(index)));
            std::ptr::copy(self.ptr.add(index + 1), self.ptr.add(index), self.len - index - 1);
            result
        }
    }
}


impl<T: Display> Display for V<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("[")?;
        for i in 0..(self.len - 1) {
            write!(f, "{}, ", self.get(i).unwrap())?;
        }
        write!(f, "{}", self.get(self.len - 1).unwrap())?;
        f.write_str("]")?;
        Ok(())
    }
}


impl<T> Drop for V<T> {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.ptr as *mut u8, Layout::array::<T>(self.cap).unwrap());
        }
    }
}
