use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

pub struct KvStore {
    data: RwLock<HashMap<String, String>>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        // Для обучения оставляем unwrap с паникой.
        let guard = self.data.read().unwrap();

        // если делать по старому, то guard не переживет выхода из метода.
        // guard.get(key).map(|s| s.as_str())
        // поэтому копируем. для обучения - ок.
        guard.get(key).cloned()
    }

    pub fn set(&self, key: impl Into<String>, value: impl Into<String>) {
        // Правило:
        // Метод сохраняет строку Into<String>.
        // Метод только читает строку &str.
        //
        // RwLock «отравляется» (poisoned), если поток паникует, пока держит
        // блокировку.
        // Для обучения оставляем unwrap с паникой.
        // Причины паники:
        // 1) Явный вызов panic!("...") — программист сам решил, что продолжать
        // нельзя.
        // 2) unwrap() / expect() на None или Err — самое частое.
        // Например, self.data.write().unwrap() — если лок отравлен,
        // unwrap паникует.
        // 3) Выход за границы массива — v[10] в Vec из 3 элементов.
        // 4) Деление на ноль — 1 / 0 для целых.
        // 5) Переполнение с проверкой — в debug-сборке i32::MAX + 1 паникует.
        // 6) assert! / assert_eq! — падают, если условие ложно.
        // 7) Явный unreachable!(), todo!(), unimplemented!() — заглушки,
        // которые паникуют при вызове.

        let mut guard = self.data.write().unwrap();
        // write() возвращает RwLockWriteGuard — умный указатель, который
        // ведёт себя как &mut HashMap

        guard.insert(key.into(), value.into());
    }

    pub fn delete(&self, key: &str) {
        // Для обучения оставляем unwrap с паникой.
        let mut guard = self.data.write().unwrap();
        guard.remove(key);
    }
}

fn main() {
    let store = Arc::new(KvStore::new());

    let mut handles = vec![];

    for i in 0..10 {
        let store = store.clone();
        let handle = thread::spawn(move || {
            store.set(format!("key{}", i), format!("value{}", i));
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    for i in 0..10 {
        println!("{:?}", store.get(&format!("key{}", i)));
    }
}
