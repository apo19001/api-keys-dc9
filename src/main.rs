use std::fs::OpenOptions;
use std::io::prelude::*;

use rand_core::{RngCore, SeedableRng, Error};
use rand_core::block::{BlockRng};

use rand_hc::Hc128Core;
use rand::Rng;
use rand::rngs::StdRng;

use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryInto;

use std::time::Duration;
use std::thread::sleep;


extern crate base64;
extern crate rand;

// this is our code for HC128Rng
// Gotten from: https://docs.rs/rand/0.5.0/src/rand/prng/hc128.rs.html#70
#[derive(Clone, Debug)]
pub struct Hc128Rng(BlockRng<Hc128Core>);

// we needed to create this impl for us to use Hc128 object. This was h
impl RngCore for Hc128Rng {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 { // create a 32-bit unsigned type
        self.0.next_u32()
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 { // create a 64-bit unsigned type
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.0.try_fill_bytes(dest)
    }
}

// we give the code a random string that we want to use it for
// Gotten from: https://docs.rs/rand/0.5.0/src/rand/prng/hc128.rs.html#70
impl SeedableRng for Hc128Rng {
    type Seed = <Hc128Core as SeedableRng>::Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        Hc128Rng(BlockRng::<Hc128Core>::from_seed(seed))
    }

    fn from_rng<R: RngCore>(rng: R) -> Result<Self, Error> {
        BlockRng::<Hc128Core>::from_rng(rng).map(Hc128Rng)
    }
}

/*
  Define User
  that will generate a random
  key to access api.
*/
struct User {
   id:u32,
   online_status:String,
   rank:String,
   first_name:String,
   last_name:String,
   email:String,
   password:String,
   permissions:String,
   hash:String,
   active:u32,
   reputation:u32,
   api_key:String
}

/* display()
displays the user that we want on our databases
*/
fn display(usr: User){
  println!("id: {} online_status: {} rank: {} first_name: {} last_name: {} email: {} api_key:{}", usr.id, usr.rank, usr.online_status, usr.first_name, usr.last_name, usr.email, usr.api_key);
}

/*
show _status()
tell the persons online status on the server
*/
fn show_status(usr: User)
{
  println!("active: {} reputation: {}", usr.active, usr.reputation);
}

/*
show _status()
hashing code
*/
fn permissions(usr: User)
{
  println!("password: {} hash: {} permissions: {}", usr.password, usr.hash, usr.permissions);
}

// our unofficial "constructor :) "
fn maker_user(id:u32, online_status: &str, rank:&str, first_name: &str, last_name:&str, email:&str, password:&str, permissions:&str, hash:&str, active:u32, reputation:u32, api_key:&str) -> User {
  let user = User {
    id:id,
    online_status:String::from(online_status ),
    rank:String::from(rank),
    first_name:String::from(first_name),
    last_name:String::from(last_name),
    email:String::from(email),
    password:String::from(password),
    permissions:String::from(permissions),
    hash:String::from(hash),
    active:active,
    reputation:reputation,
    api_key:String::from(api_key)
  };

  return user;
}
// https://users.rust-lang.org/t/how-can-i-return-vec-from-function/34405
// https://stackoverflow.com/questions/26593387/how-can-i-get-the-current-time-in-milliseconds
// https://stackoverflow.com/questions/59020767/how-can-i-input-an-integer-seed-for-producing-random-numbers-using-the-rand-crat
fn create_random_seed() -> Vec<u8>  // our random seed generator ... using time 
{
  let mut random_seed:Vec<u8> = Vec::new();
  // we want to see the number as something different each time.
  let start = SystemTime::now();
  let epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

  let mut in_ms = epoch.as_millis() % 1000; // make the number super small - we want to generate random ints. 
 
  let mut seed_r = StdRng::seed_from_u64(in_ms.try_into().unwrap()); // we wanted to get some sort of seed somehow.
  for _i in 0..32
  {
    let mut rand_num:u8 = seed_r.gen(); // we want to be able to create random numbers each time. 
    random_seed.push(rand_num);
  }
  return random_seed;
}


/*
convert()
Convert our vector into an array here
Source(s)
https://stackoverflow.com/questions/25428920/how-to-get-a-slice-as-an-array-in-rust
https://users.rust-lang.org/t/convert-vec-to-array-with-tryfrom/50727

We followed the practice of having a 32 element array and encoding it with 32.
*/
fn convert(v: Vec<u8>) -> [u8; 32] {
  let slice = v.as_slice();
  let arr: [u8; 32] = match slice.try_into() {
    Ok(ba) => ba,
    Err(_) => panic!("Expected vec of len {} but got {} ", 32, v.len()),
  };
  return arr;
}

/*
We want to create a unique api key for each user.

Sources(s):
https://stackoverflow.com/questions/14412132/whats-the-best-approach-for-generating-a-new-api-key
https://stackoverflow.com/questions/55009503/how-services-generate-and-use-public-and-secret-api-keys/61301438#61301438
https://stackoverflow.com/questions/62923895/how-do-i-generate-random-numbers-using-hc-128-in-rust
https://crates.io/crates/base64
https://stackoverflow.com/questions/58051863/convert-u8-array-to-base64-string-in-rust
https://users.rust-lang.org/t/how-do-i-convert-vec-of-i32-to-string/18669/10
https://rust-random.github.io/rand/rand_hc/struct.Hc128Rng.html
*/

fn make_api_key() -> String
{
  // vector we want to store all the integers in
  let mut vec: Vec<u64> = Vec::new(); // define array to shove our random numbers in.

  // we need to create a random string each time. 
  let seed = create_random_seed(); // this was a pain to implement -> 
  let mut rng = Hc128Rng::from_seed(convert(seed));  // our rng seed geberator that we want to use for our code.
  
  for _i in 0..4
  {
    let random_num = rng.next_u64();
    vec.push(random_num); //push this u64 integer into the 
  }

  let random_sequense: String = vec.iter().map(ToString::to_string).collect(); // iterate through our vector, and turn everything into a string to base64
  let api_key = base64::encode(&random_sequense);
  return api_key;
}

/*
export()  -> get api key from vector, and export -> make sure we can upload the api key to a server later.
Source (s): 
https://doc.rust-lang.org/std/fs/struct.File.html
https://www.tutorialspoint.com/rust/rust_file_input_output.htm
https://stackoverflow.com/questions/30684624/what-is-the-best-variant-for-appending-a-new-line-in-a-text-file
*/
fn export(filename: &str, message: &str) 
{
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", message) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

/*
Does_user_exist()
Check when we want to go through each user, check if the user already exists when added into main.
*/
fn does_user_exist(usr1: User, usr2: &User) -> bool{
  if usr1.email == usr2.email {
    return true;
  }
  return false;
}

/*
call main functuon, pass in the struct variables in order
to get stuff to work

Post these up to the github
https://doc.rust-lang.org/rust-by-example/std/vec.html
https://www.tutorialspoint.com/rust/rust_structure.htm
https://doc.rust-lang.org/std/fs/struct.File.html

https://stackoverflow.com/questions/28952938/how-can-i-put-the-current-thread-to-sleep
*/

fn main() {
  println!("Hello World! - this is an example of our database thtat we want to make (see user1-user4)");

  let user1 = maker_user(1, "online", "admin", "Moses", "Apostol", "mapo3126@gmail.com", "random", "admin", "random", 1, 100, "random");
  let user2 = maker_user(2, "online", "CTO", "Bill", "Apostol", "bill.apostol@gmail.com", "random", "admin", "random", 1, 130, "random");
  let user3 = maker_user(3, "online", "CFO", "Iromn", "Apostol", "iromn.apostol@gmail.com", "random", "admin", "random", 1, 500, "random");
  let same_useremail = maker_user(3, "online", "CFO", "Iromn", "Apostol", "iromn.apostol@gmail.com", "random", "admin", "random", 1, 500, "random");

  // all of the users in a user_list that no one can change! this is an example of a  immutable variable
  // our list of people we want to check our emails with
  let user_list = vec![user1, user2, user3];
  let mut apikeys: Vec<String> = Vec::new();

  // example of using a for loop

  // example of finding a user that already exists -> and that we need to generate a new api key.
  for user in user_list {
    if does_user_exist(user, &same_useremail) {
      println!("We found a duplicate email! Do not allow to generate new API key");
    }
    else{
      println!("We found a new email! Generate a new API key");
      let api_access: String = make_api_key(); // pass a list of a new api keys that we can work with.
      apikeys.push(api_access);
      sleep(Duration::from_millis(2)); // we want the milliseconds to be different each time, so we want it to sleep for a little but
    }
  }

  display(same_useremail);  

  for keys in apikeys {
    println!("{}", keys);
    export("api_keys.txt", &keys)
  }
}