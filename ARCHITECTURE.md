Code architecture Scheme

./home.sh
   | - Identifies mode
   | - Builds
   | - Installs
   V
 Main
   | - Loads config file
   V
  Env
   | - Runs "run" function
   | - "run" is the loop
   V 
