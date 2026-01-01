use std::fmt::Display;
use std::ops::{Add, Deref};

// =========================================================================
// Exercise 1: Traits, Derives, and Bounds (Topics 4.1, 4.4, 4.5)
// =========================================================================

// 1.1: Derive `Debug`, `Clone`, and `PartialEq` for the `Player` struct.
//      (We need Clone to duplicate players, PartialEq to compare them in tests).
#[derive(Debug, Clone, PartialEq)] // <--- TODO: Add derives here
pub struct Player {
    pub name: String,
    pub level: u32,
}

// 1.2: Define a custom trait named `Playable`.
//      It should have one method: `play(&self)`.
//      It should return a String (e.g., "Player [name] is playing level [level]").
pub trait Playable {
    // TODO: Define the function signature here
    fn play(&self) -> String;
}

// 1.3: Implement `Playable` for `Player`.
impl Playable for Player {
    // TODO: Implement the function here
    fn play(&self) -> String {
        format!("Player {} is playing level {}", self.name, self.level)
    }
}

// 1.4: Write a generic function `start_game` that accepts any type `T`
//      that implements the `Playable` trait.
//      It should call `.play()` on the item and return the resulting String.
pub fn start_game<T>(character: &T) -> String 
where
    T: Playable // <--- This is the trait bound
{
    character.play()
}

// =========================================================================
// Exercise 2: Operator Overloading & Copy Trait (Topics 4.3, 4.12)
// =========================================================================

// 2.1: Derive `Debug`, `Copy`, `Clone`, and `PartialEq`.
//      Remember: You can only derive Copy if all fields are Copy.
//      (i32 is Copy, so this is safe).
#[derive(Debug, Copy, Clone, PartialEq)] // <--- TODO: Add derives here
pub struct Millimeters(pub i32);

#[derive(Debug, Copy, Clone, PartialEq)]  // <--- TODO: Add derives here
pub struct Meters(pub i32);

// 2.2: Implement `Add<Meters>` for `Millimeters`.
//      This means: Millimeters + Meters = Millimeters.
//      Logic: Convert Meters to Millimeters (x 1000) and add.
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Self{
            0: self.0 + rhs.0 * 1000
        }
    }
}

// =========================================================================
// Exercise 3: From Trait (Topic 4.9)
// =========================================================================

#[derive(Debug, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Unknown,
}

// 3.1: Implement `From<u8>` for `ConnectionState`.
//      0 -> Disconnected
//      1 -> Connecting
//      2 -> Connected
//      Anything else -> Unknown
impl From<u8> for ConnectionState {
    fn from(code: u8) -> Self {
        match code {
            0 =>  ConnectionState::Disconnected,
            1 => ConnectionState::Connecting,
            2 => ConnectionState::Connected,
            _ => ConnectionState::Unknown,
        }
    }
}

// 3.2: Implement `From<&str>` for `ConnectionState`.
//      "off" -> Disconnected
//      "on"  -> Connected
//      Anything else -> Unknown
impl From<&str> for ConnectionState {
    fn from(s: &str) -> Self {
        match s {
            "off" => ConnectionState::Disconnected,
            "on" => ConnectionState::Connected,
            _ => ConnectionState::Unknown,
        }
    }
}

// =========================================================================
// Exercise 4: Deref Trait (Topic 4.7)
// =========================================================================

// This is a wrapper struct (Newtype pattern)
pub struct LoginToken {
    pub token: String,
}

// 4.1: Implement `Deref` for `LoginToken` so it points to the inner String.
//      This allows functions that accept `&str` to accept `&LoginToken`.
impl Deref for LoginToken {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

// =========================================================================
// Exercise 5: Associated Types (Topic 4.10)
// =========================================================================

pub trait Shape {
    // 5.1: Define an associated type named `Area`.
    //      (We do this because area might be f64 for a Circle, but u32 for a pixel Square).
    type Area;

    fn calculate_area(&self) -> Self::Area;
}

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

// 5.2: Implement `Shape` for `Rectangle`.
//      Set the `Area` type to `u32`.
impl Shape for Rectangle {
    type Area = u32; // <--- This fixes the type for this specific struct

    fn calculate_area(&self) -> Self::Area {
       &self.height * self.width
    }
}

// =========================================================================
// Exercise 6: Drop Trait (Topic 4.13)
// =========================================================================

// A struct representing a temporary file that should be closed when finished.
pub struct TempFile {
    pub filename: String,
}

// 6.1: Implement `Drop`.
//      Inside the drop function, print "Closing file: [filename]" to stdout.
impl Drop for TempFile {
    fn drop(&mut self) {
        // Note: In real code we would actually delete the file here.
        // For this exercise, just print the message.
        println!("Closing file: {} to stdout", self.filename)
    }
}

// =========================================================================
// TESTS
// =========================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_traits_generics() {
        let p = Player { name: "Hero".to_string(), level: 5 };
        // Test Clone (derived)
        let p2 = p.clone(); 
        // Test PartialEq (derived)
        assert_eq!(p, p2); 
        // Test trait method
        assert_eq!(start_game(&p), "Player Hero is playing level 5");
    }

    #[test]
    fn test_ex2_ops_copy() {
        let mm = Millimeters(500);
        let m = Meters(1); // 1 meter = 1000mm

        // Test Add implementation
        let result = mm + m; 
        assert_eq!(result, Millimeters(1500));

        // Test Copy (mm should still exist after being used in +)
        assert_eq!(mm.0, 500); 
    }

    #[test]
    fn test_ex3_from() {
        let state1: ConnectionState = ConnectionState::from(2);
        assert_eq!(state1, ConnectionState::Connected);

        let state2: ConnectionState = ConnectionState::from("off");
        assert_eq!(state2, ConnectionState::Disconnected);
    }

    #[test]
    fn test_ex4_deref() {
        let t = LoginToken { token: String::from("ABC-123") };
        // This function takes &str, but we pass &LoginToken.
        // Deref Coercion should happen automatically.
        fn check_length(s: &str) -> usize { s.len() }
        
        assert_eq!(check_length(&t), 7);
        assert_eq!(*t, "ABC-123");
    }

    #[test]
    fn test_ex5_associated_types() {
        let r = Rectangle { width: 10, height: 5 };
        let area: u32 = r.calculate_area();
        assert_eq!(area, 50);
    }

    // Note: We cannot easily test Drop automatically because it prints to stdout,
    // but running this test should show the output if you use `cargo test -- --nocapture`
    #[test]
    fn test_ex6_drop() {
        {
            let _f = TempFile { filename: String::from("test.txt") };
            // _f goes out of scope here -> Drop should trigger
        }
        // Verify visually in terminal output
    }
}