# SolanaApp

# Instructions

1. Clone repo
   ```js
   git clone
   ```
     
2. Install dependencies
     ```js
     npm install
     ```
     
2. Run test
     ```javascript
     Anchor test
     ```
     
 # Description
 
 The application was modeled after what a similar application would look like if built in Solidity. First I defined a struct that each track would use as a model when uploaded. The struct contained the track name and the cid assoted with it. This struct was then saved into a vector giving us a list of all track we have uploaed.
 
 The tricky part for me was the fucntion to read through the vector. I was able to write a function that takes in a number as an argument and then searches through the vector for the track that is associated with the id. The problem was when I tried to test this on the JS side I was getting an error that my fucntion `print_cid` does not exist on my program and I was unable to solve this issue. 
